use crate::bins_hash::DenoBinsHash;
use crate::deps_hash::DenoDepsHash;
use crate::target_hash::DenoTargetHash;
use moon_action_context::ActionContext;
use moon_common::{color, is_ci, is_test_env, Id};
use moon_config::{
    BinEntry, DenoConfig, DependencyConfig, HasherConfig, HasherOptimization, PlatformType,
    ProjectConfig, TypeScriptConfig,
};
use moon_console::{Checkpoint, Console};
use moon_deno_lang::{load_lockfile_dependencies, DenoJson};
use moon_deno_tool::{get_deno_env_paths, DenoTool};
use moon_hash::ContentHasher;
use moon_logger::{debug, map_list};
use moon_platform::{Platform, Runtime, RuntimeReq};
use moon_process::Command;
use moon_project::Project;
use moon_task::Task;
use moon_tool::{
    get_proto_version_env, prepend_path_env_var, DependencyManager, Tool, ToolManager,
};
use moon_typescript_platform::TypeScriptTargetHash;
use moon_utils::async_trait;
use proto_core::{hash_file_contents, ProtoEnvironment, UnresolvedVersionSpec};
use rustc_hash::FxHashMap;
use std::sync::Arc;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

const LOG_TARGET: &str = "moon:deno-platform";

pub struct DenoPlatform {
    config: DenoConfig,

    console: Arc<Console>,

    proto_env: Arc<ProtoEnvironment>,

    toolchain: ToolManager<DenoTool>,

    typescript_config: Option<TypeScriptConfig>,

    workspace_root: PathBuf,
}

impl DenoPlatform {
    pub fn new(
        config: &DenoConfig,
        typescript_config: &Option<TypeScriptConfig>,
        workspace_root: &Path,
        proto_env: Arc<ProtoEnvironment>,
        console: Arc<Console>,
    ) -> Self {
        DenoPlatform {
            config: config.to_owned(),
            proto_env,
            toolchain: ToolManager::new(Runtime::new(PlatformType::Deno, RuntimeReq::Global)),
            typescript_config: typescript_config.to_owned(),
            workspace_root: workspace_root.to_path_buf(),
            console,
        }
    }
}

#[async_trait]
impl Platform for DenoPlatform {
    fn get_type(&self) -> PlatformType {
        PlatformType::Deno
    }

    fn get_runtime_from_config(&self, project_config: Option<&ProjectConfig>) -> Runtime {
        if let Some(config) = &project_config {
            if let Some(deno_config) = &config.toolchain.deno {
                if let Some(version) = &deno_config.version {
                    return Runtime::new_override(
                        PlatformType::Deno,
                        RuntimeReq::Toolchain(version.to_owned()),
                    );
                }
            }
        }

        if let Some(version) = &self.config.version {
            return Runtime::new(
                PlatformType::Deno,
                RuntimeReq::Toolchain(version.to_owned()),
            );
        }

        Runtime::new(PlatformType::Deno, RuntimeReq::Global)
    }

    fn matches(&self, platform: &PlatformType, runtime: Option<&Runtime>) -> bool {
        if matches!(platform, PlatformType::Deno) {
            return true;
        }

        if let Some(runtime) = &runtime {
            return matches!(runtime.platform, PlatformType::Deno);
        }

        false
    }

    // PROJECT GRAPH

    fn load_project_implicit_dependencies(
        &self,
        _project_id: &str,
        _project_source: &str,
    ) -> miette::Result<Vec<DependencyConfig>> {
        Ok(vec![])
    }

    // TOOLCHAIN

    fn is_toolchain_enabled(&self) -> miette::Result<bool> {
        Ok(self.config.version.is_some())
    }

    fn get_tool(&self) -> miette::Result<Box<&dyn Tool>> {
        let tool = self.toolchain.get()?;

        Ok(Box::new(tool))
    }

    fn get_tool_for_version(&self, req: RuntimeReq) -> miette::Result<Box<&dyn Tool>> {
        let tool = self.toolchain.get_for_version(&req)?;

        Ok(Box::new(tool))
    }

    fn get_dependency_configs(&self) -> miette::Result<Option<(String, String)>> {
        Ok(Some((
            "deno.lock".to_owned(),
            self.config.deps_file.to_owned(),
        )))
    }

    async fn setup_toolchain(&mut self) -> miette::Result<()> {
        let req = match &self.config.version {
            Some(v) => RuntimeReq::Toolchain(v.to_owned()),
            None => RuntimeReq::Global,
        };

        let mut last_versions = FxHashMap::default();

        if !self.toolchain.has(&req) {
            self.toolchain.register(
                &req,
                DenoTool::new(
                    Arc::clone(&self.proto_env),
                    Arc::clone(&self.console),
                    &self.config,
                    &req,
                )
                .await?,
            );
        }

        self.toolchain.setup(&req, &mut last_versions).await?;

        Ok(())
    }

    async fn teardown_toolchain(&mut self) -> miette::Result<()> {
        self.toolchain.teardown_all().await?;

        Ok(())
    }

    // ACTIONS

    async fn setup_tool(
        &mut self,
        _context: &ActionContext,
        runtime: &Runtime,
        last_versions: &mut FxHashMap<String, UnresolvedVersionSpec>,
    ) -> miette::Result<u8> {
        let req = &runtime.requirement;

        if !self.toolchain.has(req) {
            self.toolchain.register(
                req,
                DenoTool::new(
                    Arc::clone(&self.proto_env),
                    Arc::clone(&self.console),
                    &self.config,
                    req,
                )
                .await?,
            );
        }

        Ok(self.toolchain.setup(req, last_versions).await?)
    }

    async fn install_deps(
        &self,
        _context: &ActionContext,
        runtime: &Runtime,
        working_dir: &Path,
    ) -> miette::Result<()> {
        let deno = self.toolchain.get_for_version(&runtime.requirement)?;

        if !self.config.bins.is_empty() {
            self.console
                .out
                .print_checkpoint(Checkpoint::Setup, "deno install")?;

            debug!(
                target: LOG_TARGET,
                "Installing Deno binaries: {}",
                map_list(&self.config.bins, |b| color::label(b.get_name()))
            );

            for bin in &self.config.bins {
                let mut args = vec![
                    "install",
                    "--allow-net",
                    "--allow-read",
                    "--no-prompt",
                    "--lock",
                    "deno.lock",
                ];

                match bin {
                    BinEntry::Name(name) => args.push(name),
                    BinEntry::Config(cfg) => {
                        if cfg.local && is_ci() {
                            continue;
                        }

                        if cfg.force {
                            args.push("--force");
                        }

                        if let Some(name) = &cfg.name {
                            args.push("--name");
                            args.push(name);
                        }

                        args.push(&cfg.bin);
                    }
                };

                deno.create_command(&())?
                    .args(args)
                    .cwd(working_dir)
                    .create_async()
                    .exec_stream_output()
                    .await?;
            }
        }

        if self.config.lockfile {
            debug!(target: LOG_TARGET, "Installing dependencies");

            self.console
                .out
                .print_checkpoint(Checkpoint::Setup, "deno cache")?;

            deno.install_dependencies(&(), working_dir, !is_test_env())
                .await?;
        }

        Ok(())
    }

    async fn sync_project(
        &self,
        _context: &ActionContext,
        _project: &Project,
        _dependencies: &FxHashMap<Id, Arc<Project>>,
    ) -> miette::Result<bool> {
        Ok(false)
    }

    async fn hash_manifest_deps(
        &self,
        manifest_path: &Path,
        hasher: &mut ContentHasher,
        _hasher_config: &HasherConfig,
    ) -> miette::Result<()> {
        if !self.config.bins.is_empty() {
            hasher.hash_content(DenoBinsHash {
                bins: &self.config.bins,
            })?;
        }

        let project_root = manifest_path.parent().unwrap();
        let mut deps_hash = DenoDepsHash::default();

        if let Ok(Some(deno_json)) = DenoJson::read(manifest_path) {
            if let Some(imports) = deno_json.imports {
                deps_hash.dependencies.extend(imports);
            }

            if let Some(import_map_path) = &deno_json.import_map {
                if let Ok(Some(import_map)) = DenoJson::read(project_root.join(import_map_path)) {
                    if let Some(imports) = import_map.imports {
                        deps_hash.dependencies.extend(imports);
                    }
                }
            }

            if let Some(scopes) = deno_json.scopes {
                deps_hash.aliases.extend(scopes);
            }
        }

        // We can't parse TS files right now, so hash the file contents
        let deps_path = project_root.join(&self.config.deps_file);

        if deps_path.exists() {
            deps_hash.dependencies.insert(
                self.config.deps_file.to_owned(),
                hash_file_contents(deps_path)?,
            );
        }

        hasher.hash_content(deps_hash)?;

        Ok(())
    }

    async fn hash_run_target(
        &self,
        project: &Project,
        runtime: &Runtime,
        hasher: &mut ContentHasher,
        hasher_config: &HasherConfig,
    ) -> miette::Result<()> {
        let deno = self.toolchain.get_for_version(&runtime.requirement).ok();
        let mut target_hash = DenoTargetHash::new(
            deno.map(|n| n.config.version.as_ref().map(|v| v.to_string()))
                .unwrap_or_default(),
        );

        if matches!(hasher_config.optimization, HasherOptimization::Accuracy) {
            let resolved_dependencies = match deno {
                Some(inst) => inst.get_resolved_dependencies(&project.root).await?,
                None => {
                    if self.config.lockfile {
                        load_lockfile_dependencies(project.root.join("deno.lock"))?
                    } else {
                        FxHashMap::default()
                    }
                }
            };

            target_hash.hash_deps(BTreeMap::from_iter(resolved_dependencies));
        };

        hasher.hash_content(target_hash)?;

        if let Ok(Some(deno_json)) = DenoJson::read(&project.root) {
            if let Some(compiler_options) = &deno_json.compiler_options {
                let mut ts_hash = TypeScriptTargetHash::default();
                ts_hash.hash_compiler_options(compiler_options);

                hasher.hash_content(ts_hash)?;
            }
        }

        // Do we need this if we're using compiler options from deno.json?
        if let Some(typescript_config) = &self.typescript_config {
            let ts_hash = TypeScriptTargetHash::generate(
                typescript_config,
                &self.workspace_root,
                &project.root,
            )?;

            hasher.hash_content(ts_hash)?;
        }

        Ok(())
    }

    async fn create_run_target_command(
        &self,
        _context: &ActionContext,
        _project: &Project,
        task: &Task,
        runtime: &Runtime,
        _working_dir: &Path,
    ) -> miette::Result<Command> {
        let mut command = Command::new(&task.command);
        command.with_console(self.console.clone());
        command.args(&task.args);
        command.envs(&task.env);

        if let Ok(deno) = self.toolchain.get_for_version(&runtime.requirement) {
            if let Some(version) = get_proto_version_env(&deno.tool) {
                command.env("PROTO_DENO_VERSION", version);
            }
        }

        if !runtime.requirement.is_global() {
            command.env(
                "PATH",
                prepend_path_env_var(get_deno_env_paths(&self.proto_env)),
            );
        }

        Ok(command)
    }
}
