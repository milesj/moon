use crate::command_builder::CommandBuilder;
use crate::command_executor::CommandExecutor;
use crate::output_archiver::OutputArchiver;
use crate::output_hydrater::{HydrateFrom, OutputHydrater};
use crate::run_state::TaskRunCacheState;
use crate::task_runner_error::TaskRunnerError;
use moon_action::{ActionNode, ActionStatus, Operation, OperationList, OperationMeta};
use moon_action_context::{ActionContext, TargetState};
use moon_api::Moonbase;
use moon_app_context::AppContext;
use moon_cache::CacheItem;
use moon_console::TaskReportItem;
use moon_platform::PlatformManager;
use moon_process::ProcessError;
use moon_project::Project;
use moon_remote::{Digest, RemoteService};
use moon_task::Task;
use moon_task_hasher::TaskHasher;
use moon_time::{is_stale, now_millis};
use starbase_utils::fs;
use std::collections::BTreeMap;
use std::time::SystemTime;
use tracing::{debug, instrument, trace};

#[derive(Debug)]
pub struct TaskRunResult {
    pub hash: Option<String>,
    pub error: Option<miette::Report>,
    pub operations: OperationList,
}

pub struct TaskRunner<'task> {
    app: &'task AppContext,
    project: &'task Project,
    pub task: &'task Task,
    platform_manager: &'task PlatformManager,
    action_digest: Digest,

    archiver: OutputArchiver<'task>,
    hydrater: OutputHydrater<'task>,

    // Public for testing
    pub cache: CacheItem<TaskRunCacheState>,
    pub operations: OperationList,
    pub report_item: TaskReportItem,
}

impl<'task> TaskRunner<'task> {
    pub fn new(
        app: &'task AppContext,
        project: &'task Project,
        task: &'task Task,
    ) -> miette::Result<Self> {
        debug!(
            task_target = task.target.as_str(),
            "Creating a task runner for target"
        );

        let mut cache = app
            .cache_engine
            .state
            .load_target_state::<TaskRunCacheState>(&task.target)?;

        if cache.data.target.is_empty() {
            cache.data.target = task.target.to_string();
        }

        Ok(Self {
            cache,
            archiver: OutputArchiver { app, project, task },
            action_digest: Digest {
                hash: String::new(),
                size_bytes: 0,
            },
            hydrater: OutputHydrater { app, task },
            platform_manager: PlatformManager::read(),
            project,
            report_item: TaskReportItem {
                output_style: task.options.output_style,
                ..Default::default()
            },
            task,
            app,
            operations: OperationList::default(),
        })
    }

    pub fn set_platform_manager(&mut self, manager: &'task PlatformManager) {
        self.platform_manager = manager;
    }

    async fn internal_run(
        &mut self,
        context: &ActionContext,
        node: &ActionNode,
    ) -> miette::Result<Option<String>> {
        // If a dependency has failed or been skipped, we should skip this task
        if !self.is_dependencies_complete(context)? {
            self.skip(context)?;

            return Ok(None);
        }

        // If cache is enabled, then generate a hash and manage outputs
        if self.is_cache_enabled() {
            debug!(
                task_target = self.task.target.as_str(),
                "Caching is enabled for task, will generate a hash and manage outputs"
            );

            let hash = self.generate_hash(context, node).await?;

            self.report_item.hash = Some(hash.clone());

            // Exit early if this build has already been cached/hashed
            if self.hydrate(context, &hash).await? {
                return Ok(Some(hash));
            }

            // Otherwise build and execute the command as a child process
            self.execute(context, node).await?;

            // If we created outputs, archive them into the cache
            self.archive(&hash).await?;

            return Ok(Some(hash));
        }

        debug!(
            task_target = self.task.target.as_str(),
            "Caching is disabled for task, will not generate a hash, and will attempt to run a command as normal"
        );

        // Otherwise build and execute the command as a child process
        self.execute(context, node).await?;

        Ok(None)
    }

    #[instrument(skip(self, context))]
    pub async fn run(
        &mut self,
        context: &ActionContext,
        node: &ActionNode,
    ) -> miette::Result<TaskRunResult> {
        let result = self.internal_run(context, node).await;

        self.cache.data.last_run_time = now_millis();
        self.cache.save()?;

        match result {
            Ok(maybe_hash) => {
                self.report_item.hash = maybe_hash.clone();

                self.app.console.reporter.on_task_completed(
                    &self.task.target,
                    &self.operations,
                    &self.report_item,
                    None,
                )?;

                Ok(TaskRunResult {
                    error: None,
                    hash: maybe_hash,
                    operations: self.operations.take(),
                })
            }
            Err(error) => {
                self.inject_failed_task_execution(Some(&error))?;

                self.app.console.reporter.on_task_completed(
                    &self.task.target,
                    &self.operations,
                    &self.report_item,
                    Some(&error),
                )?;

                Ok(TaskRunResult {
                    error: Some(error),
                    hash: None,
                    operations: self.operations.take(),
                })
            }
        }
    }

    #[cfg(debug_assertions)]
    pub async fn run_with_panic(
        &mut self,
        context: &ActionContext,
        node: &ActionNode,
    ) -> miette::Result<TaskRunResult> {
        let result = self.run(context, node).await?;

        if let Some(error) = result.error {
            panic!("{}", error.to_string());
        }

        Ok(result)
    }

    #[instrument(skip(self))]
    pub async fn is_cached(&mut self, hash: &str) -> miette::Result<Option<HydrateFrom>> {
        let cache_engine = &self.app.cache_engine;

        debug!(
            task_target = self.task.target.as_str(),
            hash, "Checking if task has been cached using hash"
        );

        // If a lifetime has been configured, we need to check the last run and the archive
        // for staleness, and return a cache miss/skip
        let cache_lifetime = match &self.task.options.cache_lifetime {
            Some(lifetime) => Some(self.app.cache_engine.parse_lifetime(lifetime)?),
            None => None,
        };

        let is_cache_stale = || {
            if let Some(duration) = cache_lifetime {
                if is_stale(self.cache.data.last_run_time, duration) {
                    debug!(
                        task_target = self.task.target.as_str(),
                        hash,
                        "Cache skip, a lifetime has been configured and the last run is stale, continuing run"
                    );

                    return true;
                }
            }

            false
        };

        // If hash is the same as the previous build, we can simply abort!
        // However, ensure the outputs also exist, otherwise we should hydrate
        if self.cache.data.exit_code == 0
            && self.cache.data.hash == hash
            && self.archiver.has_outputs_been_created(true)?
        {
            if is_cache_stale() {
                return Ok(None);
            }

            debug!(
                task_target = self.task.target.as_str(),
                hash, "Hash matches previous run, reusing existing outputs"
            );

            return Ok(Some(HydrateFrom::PreviousOutput));
        }

        if !cache_engine.is_readable() {
            debug!(
                task_target = self.task.target.as_str(),
                hash, "Cache is not readable, continuing run"
            );

            return Ok(None);
        }

        // Set this *after* we checked the previous outputs above
        self.cache.data.hash = hash.to_owned();

        // If the previous run was a failure, avoid hydrating
        if self.cache.data.exit_code > 0 {
            debug!(
                task_target = self.task.target.as_str(),
                hash, "Previous run failed, avoiding hydration"
            );

            return Ok(None);
        }

        // Check if last run is stale
        if is_cache_stale() {
            return Ok(None);
        }

        // Check to see if a build with the provided hash has been cached locally.
        // We only check for the archive, as the manifest is purely for local debugging!
        let archive_file = cache_engine.hash.get_archive_path(hash);

        if archive_file.exists() {
            // Also check if the archive itself is stale
            if let Some(duration) = cache_lifetime {
                if fs::is_stale(&archive_file, false, duration, SystemTime::now())?.is_some() {
                    debug!(
                        task_target = self.task.target.as_str(),
                        hash,
                        archive_file = ?archive_file,
                        "Cache skip in local cache, a lifetime has been configured and the archive is stale, continuing run"
                    );

                    return Ok(None);
                }
            }

            debug!(
                task_target = self.task.target.as_str(),
                hash,
                archive_file = ?archive_file,
                "Cache hit in local cache, will reuse existing archive"
            );

            return Ok(Some(HydrateFrom::LocalCache));
        }

        // Check if the outputs have been cached in the remote service
        if let Some(remote) = RemoteService::session() {
            if remote.is_operation_cached(&self.action_digest).await? {
                debug!(
                    task_target = self.task.target.as_str(),
                    hash, "Cache hit in remote service, will attempt to download output blobs"
                );

                return Ok(Some(HydrateFrom::RemoteCache));
            }
        }

        // Check if archive exists in moonbase (remote storage) by querying the artifacts
        // endpoint. This only checks that the database record exists!
        if let Some(moonbase) = Moonbase::session() {
            if let Some((artifact, _)) = moonbase.read_artifact(hash).await? {
                debug!(
                    task_target = self.task.target.as_str(),
                    hash,
                    artifact_id = artifact.id,
                    "Cache hit in remote cache, will attempt to download the archive"
                );

                return Ok(Some(HydrateFrom::Moonbase));
            }
        }

        debug!(
            task_target = self.task.target.as_str(),
            hash, "Cache miss, continuing run"
        );

        Ok(None)
    }

    pub fn is_cache_enabled(&self) -> bool {
        // If the VCS root does not exist (like in a Docker container),
        // we should avoid failing and simply disable caching
        self.task.options.cache && self.app.vcs.is_enabled()
    }

    #[instrument(skip_all)]
    pub fn is_dependencies_complete(&self, context: &ActionContext) -> miette::Result<bool> {
        if self.task.deps.is_empty() {
            return Ok(true);
        }

        for dep in &self.task.deps {
            if let Some(dep_state) = context.target_states.get(&dep.target) {
                if dep_state.get().is_complete() {
                    continue;
                }

                debug!(
                    task_target = self.task.target.as_str(),
                    dependency_target = dep.target.as_str(),
                    "Task dependency has failed or has been skipped, skipping this task",
                );

                return Ok(false);
            } else {
                return Err(TaskRunnerError::MissingDependencyHash {
                    dep_target: dep.target.clone(),
                    target: self.task.target.clone(),
                }
                .into());
            }
        }

        Ok(true)
    }

    #[instrument(skip_all)]
    pub async fn generate_hash(
        &mut self,
        context: &ActionContext,
        node: &ActionNode,
    ) -> miette::Result<String> {
        debug!(
            task_target = self.task.target.as_str(),
            "Generating a unique hash for this task"
        );

        let hash_engine = &self.app.cache_engine.hash;
        let mut hasher = hash_engine.create_hasher(node.label());
        let mut operation = Operation::hash_generation();

        // Hash common fields
        trace!(
            task_target = self.task.target.as_str(),
            "Including common task related fields in the hash"
        );

        let mut task_hasher = TaskHasher::new(
            self.project,
            self.task,
            &self.app.vcs,
            &self.app.workspace_root,
            &self.app.workspace_config.hasher,
        );

        if self.task.script.is_none() && context.should_inherit_args(&self.task.target) {
            task_hasher.hash_args(&context.passthrough_args);
        }

        task_hasher.hash_deps({
            let mut deps = BTreeMap::default();

            for dep in &self.task.deps {
                if let Some(entry) = context.target_states.get(&dep.target) {
                    match entry.get() {
                        TargetState::Passed(hash) => {
                            deps.insert(&dep.target, hash.clone());
                        }
                        TargetState::Passthrough => {
                            deps.insert(&dep.target, "passthrough".into());
                        }
                        _ => {}
                    };
                }
            }

            deps
        });

        task_hasher.hash_inputs().await?;

        hasher.hash_content(task_hasher.hash())?;

        // Hash platform fields
        trace!(
            task_target = self.task.target.as_str(),
            platform = ?self.task.platform,
            "Including toolchain specific fields in the hash"
        );

        self.platform_manager
            .get(self.task.platform)?
            .hash_run_target(
                self.project,
                node.get_runtime(),
                &mut hasher,
                &self.app.workspace_config.hasher,
            )
            .await?;

        let (hash, size_bytes) = hash_engine.save_manifest(hasher)?;

        operation.meta.set_hash(&hash);
        operation.finish(ActionStatus::Passed);

        self.operations.push(operation);

        if size_bytes > 0 {
            self.action_digest = Digest {
                hash: hash.clone(),
                size_bytes: size_bytes as i64,
            };
        }

        debug!(
            task_target = self.task.target.as_str(),
            hash = &hash,
            "Generated a unique hash"
        );

        Ok(hash)
    }

    #[instrument(skip(self, context, node))]
    pub async fn execute(
        &mut self,
        context: &ActionContext,
        node: &ActionNode,
    ) -> miette::Result<()> {
        // If the task is a no-operation, we should exit early
        if self.task.is_no_op() {
            self.skip_no_op(context)?;

            return Ok(());
        }

        debug!(
            task_target = self.task.target.as_str(),
            "Building and executing the task command"
        );

        // Build the command from the current task
        let mut builder = CommandBuilder::new(self.app, self.project, self.task, node);
        builder.set_platform_manager(self.platform_manager);

        let command = builder.build(context).await?;

        // Execute the command and gather all attempts made
        let executor = CommandExecutor::new(self.app, self.project, self.task, node, command);

        let result = if let Some(mutex_name) = &self.task.options.mutex {
            let mut operation = Operation::mutex_acquisition();

            trace!(
                task_target = self.task.target.as_str(),
                mutex = mutex_name,
                "Waiting to acquire task mutex lock"
            );

            let mutex = context.get_or_create_mutex(mutex_name);
            let _guard = mutex.lock().await;

            trace!(
                task_target = self.task.target.as_str(),
                mutex = mutex_name,
                "Acquired task mutex lock"
            );

            operation.finish(ActionStatus::Passed);

            self.operations.push(operation);

            // This execution is required within this block so that the
            // guard above isn't immediately dropped!
            executor.execute(context, &mut self.report_item).await?
        } else {
            executor.execute(context, &mut self.report_item).await?
        };

        if let Some(last_attempt) = result.attempts.get_last_execution() {
            self.save_logs(last_attempt)?;
        }

        // Extract the attempts from the result
        self.operations.merge(result.attempts);

        // Update the action state based on the result
        context.set_target_state(&self.task.target, result.run_state);

        // If the execution as a whole failed, return the error.
        // We do this here instead of in `execute` so that we can
        // capture the attempts and report them.
        if let Some(result_error) = result.error {
            return Err(result_error);
        }

        if let Some(last_attempt) = self.operations.get_last_execution() {
            // If our last task execution was a failure, return a hard error
            if last_attempt.has_failed() {
                return Err(TaskRunnerError::RunFailed {
                    target: self.task.target.clone(),
                    error: Box::new(ProcessError::ExitNonZero {
                        bin: self.task.command.clone(),
                        status: last_attempt.get_output_status(),
                    }),
                }
                .into());
            }
        }

        Ok(())
    }

    #[instrument(skip_all)]
    pub fn skip(&mut self, context: &ActionContext) -> miette::Result<()> {
        debug!(task_target = self.task.target.as_str(), "Skipping task");

        self.operations.push(Operation::new_finished(
            OperationMeta::TaskExecution(Default::default()),
            ActionStatus::Skipped,
        ));

        context.set_target_state(&self.task.target, TargetState::Skipped);

        Ok(())
    }

    #[instrument(skip(self, context))]
    pub fn skip_no_op(&mut self, context: &ActionContext) -> miette::Result<()> {
        debug!(
            task_target = self.task.target.as_str(),
            "Skipping task as its a no-operation"
        );

        self.operations.push(Operation::new_finished(
            OperationMeta::NoOperation,
            ActionStatus::Passed,
        ));

        context.set_target_state(
            &self.task.target,
            TargetState::from_hash(self.report_item.hash.as_deref()),
        );

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn archive(&mut self, hash: &str) -> miette::Result<bool> {
        let mut operation = Operation::archive_creation();

        debug!(
            task_target = self.task.target.as_str(),
            "Running cache archiving operation"
        );

        let archived = match self
            .archiver
            .archive(&self.action_digest, self.operations.get_last_execution())
            .await?
        {
            Some(archive_file) => {
                debug!(
                    task_target = self.task.target.as_str(),
                    archive_file = ?archive_file,
                    "Ran cache archiving operation"
                );

                operation.finish(ActionStatus::Passed);

                true
            }
            None => {
                debug!(
                    task_target = self.task.target.as_str(),
                    "Nothing to archive"
                );

                operation.finish(ActionStatus::Skipped);

                false
            }
        };

        self.operations.push(operation);

        Ok(archived)
    }

    #[instrument(skip(self, context))]
    pub async fn hydrate(&mut self, context: &ActionContext, hash: &str) -> miette::Result<bool> {
        let mut operation = Operation::output_hydration();

        debug!(
            task_target = self.task.target.as_str(),
            "Running cache hydration operation"
        );

        // Not cached
        let Some(from) = self.is_cached(hash).await? else {
            debug!(
                task_target = self.task.target.as_str(),
                "Nothing to hydrate"
            );

            operation.finish(ActionStatus::Skipped);

            self.operations.push(operation);

            return Ok(false);
        };

        // Did not hydrate
        if !self.hydrater.hydrate(hash, from).await? {
            debug!(task_target = self.task.target.as_str(), "Did not hydrate");

            operation.finish(ActionStatus::Invalid);

            self.operations.push(operation);

            return Ok(false);
        }

        // Did hydrate
        debug!(
            task_target = self.task.target.as_str(),
            "Ran cache hydration operation"
        );

        // Fill in these values since the command executor does not run!
        self.report_item.output_prefix = Some(context.get_target_prefix(&self.task.target));

        self.load_logs(&mut operation)?;

        operation.finish(match from {
            HydrateFrom::Moonbase | HydrateFrom::RemoteCache => ActionStatus::CachedFromRemote,
            _ => ActionStatus::Cached,
        });

        context.set_target_state(&self.task.target, TargetState::Passed(hash.to_owned()));

        self.operations.push(operation);

        Ok(true)
    }

    // If a task fails *before* the command is actually executed, say during the command
    // build process, or the toolchain plugin layer, that error is not bubbled up as a
    // failure, and the last operation is used instead (which is typically skipped).
    // To handle this weird scenario, we inject a failed task execution at the end.
    fn inject_failed_task_execution(
        &mut self,
        report: Option<&miette::Report>,
    ) -> miette::Result<()> {
        let has_exec = self
            .operations
            .iter()
            .any(|operation| operation.meta.is_task_execution());

        if has_exec {
            return Ok(());
        }

        let mut operation = Operation::task_execution(&self.task.command);

        if let (Some(output), Some(error)) = (operation.get_output_mut(), report) {
            output.exit_code = Some(-1);
            output.set_stderr(error.to_string());
        }

        operation.finish(ActionStatus::Aborted);

        self.app.console.reporter.on_task_finished(
            &self.task.target,
            &operation,
            &self.report_item,
            report,
        )?;

        self.operations.push(operation);

        Ok(())
    }

    fn load_logs(&self, operation: &mut Operation) -> miette::Result<()> {
        if let Some(output) = operation.get_output_mut() {
            let state_dir = self
                .app
                .cache_engine
                .state
                .get_target_dir(&self.task.target);
            let err_path = state_dir.join("stderr.log");
            let out_path = state_dir.join("stdout.log");

            output.exit_code = Some(self.cache.data.exit_code);

            if err_path.exists() {
                output.set_stderr(fs::read_file(err_path)?);
            }

            if out_path.exists() {
                output.set_stdout(fs::read_file(out_path)?);
            }
        }

        Ok(())
    }

    fn save_logs(&mut self, operation: &Operation) -> miette::Result<()> {
        let state_dir = self
            .app
            .cache_engine
            .state
            .get_target_dir(&self.task.target);
        let err_path = state_dir.join("stderr.log");
        let out_path = state_dir.join("stdout.log");

        if let Some(output) = operation.get_output() {
            self.cache.data.exit_code = output.get_exit_code();

            fs::write_file(
                err_path,
                output
                    .stderr
                    .as_ref()
                    .map(|log| log.as_bytes())
                    .unwrap_or_default(),
            )?;

            fs::write_file(
                out_path,
                output
                    .stdout
                    .as_ref()
                    .map(|log| log.as_bytes())
                    .unwrap_or_default(),
            )?;
        } else {
            // Ensure logs from a previous run are removed
            fs::remove_file(err_path)?;
            fs::remove_file(out_path)?;
        }

        Ok(())
    }
}
