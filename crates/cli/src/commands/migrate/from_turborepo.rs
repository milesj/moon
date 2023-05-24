use super::check_dirty_repo;
use moon::{generate_project_graph, load_workspace};
use moon_common::Id;
use moon_config2::{
    InheritedTasksConfig, PlatformType, ProjectConfig, TaskCommandArgs, TaskConfig,
};
use moon_constants as constants;
use moon_logger::{info, warn};
use moon_target::{Target, TargetError};
use moon_terminal::safe_exit;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use starbase::AppResult;
use starbase_utils::fs;
use starbase_utils::{json, yaml};
use std::path::PathBuf;

const LOG_TARGET: &str = "moon:migrate:from-turborepo";

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TurboTask {
    pub cache: Option<bool>,
    pub depends_on: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub inputs: Option<Vec<String>>,
    pub outputs: Option<Vec<String>>,
    pub persistent: Option<bool>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TurboJson {
    pub global_dependencies: Option<Vec<String>>,
    pub global_env: Option<Vec<String>>,
    pub pipeline: FxHashMap<String, TurboTask>,
}

pub fn extract_project_task_ids(key: &str) -> (Option<Id>, Id) {
    if key.contains('#') {
        let mut parts = key.split('#');

        return (
            Some(Id::raw(parts.next().unwrap())),
            Id::raw(parts.next().unwrap()),
        );
    }

    (None, Id::raw(key))
}

pub fn convert_globals(turbo: &TurboJson, tasks_config: &mut InheritedTasksConfig) -> bool {
    let mut modified = false;

    if let Some(global_deps) = &turbo.global_dependencies {
        tasks_config.implicit_inputs.extend(global_deps.to_owned());
        modified = true;
    }

    if let Some(global_env) = &turbo.global_env {
        for env in global_env {
            tasks_config.implicit_inputs.push(format!("${env}"));
        }

        modified = true;
    }

    modified
}

pub fn convert_task(name: Id, task: TurboTask) -> Result<TaskConfig, TargetError> {
    let mut config = TaskConfig::default();
    let mut inputs = vec![];

    config.command = TaskCommandArgs::String(format!("moon node run-script {name}"));

    if let Some(turbo_deps) = task.depends_on {
        let mut deps: Vec<Target> = vec![];

        for dep in turbo_deps {
            if dep.starts_with('^') {
                deps.push(Target::parse(&dep.replace('^', "^:"))?);
            } else if dep.contains('#') {
                deps.push(Target::parse(&dep.replace('#', ":"))?);
            } else if dep.starts_with('$') {
                inputs.push(dep);
            } else {
                deps.push(Target::parse(&dep)?);
            }
        }

        if !deps.is_empty() {
            config.deps = deps;
        }
    }

    if let Some(turbo_env) = task.env {
        for env in turbo_env {
            inputs.push(format!("${env}"));
        }
    }

    if let Some(turbo_inputs) = task.inputs {
        inputs.extend(turbo_inputs);
    }

    if let Some(turbo_outputs) = task.outputs {
        let mut outputs = vec![];

        for output in turbo_outputs {
            if output.ends_with("/**") {
                outputs.push(format!("{output}/*"));
            } else {
                outputs.push(output);
            }
        }

        if !outputs.is_empty() {
            config.outputs = outputs;
        }
    }

    if !inputs.is_empty() {
        config.inputs = inputs;
    }

    config.platform = PlatformType::Node;
    config.local = task.persistent.unwrap_or_default();
    config.options.cache = task.cache;

    Ok(config)
}

pub async fn from_turborepo(skip_touched_files_check: bool) -> AppResult {
    let mut workspace = load_workspace().await?;
    let turbo_file = workspace.root.join("turbo.json");

    if !turbo_file.exists() {
        eprintln!("No turbo.json was found in the workspace root.");
        safe_exit(1);
    }

    if skip_touched_files_check {
        info!(target: LOG_TARGET, "Skipping touched files check.");
    } else {
        check_dirty_repo(&workspace).await?;
    };

    let project_graph = generate_project_graph(&mut workspace).await?;
    let turbo_json: TurboJson = json::read_file(&turbo_file)?;
    let mut node_tasks_config = InheritedTasksConfig::default();
    let mut has_modified_global_tasks = false;

    // Convert globals first
    if convert_globals(&turbo_json, &mut node_tasks_config) {
        has_modified_global_tasks = true;
    }

    // Convert tasks second
    let mut has_warned_root_tasks = false;
    let mut modified_projects: FxHashMap<&PathBuf, ProjectConfig> = FxHashMap::default();

    for (id, task) in turbo_json.pipeline {
        if id.starts_with("//#") {
            if !has_warned_root_tasks {
                warn!(
                    target: LOG_TARGET,
                    "Unable to migrate root-level `//#` tasks. Create a root-level project manually to support similar functionality: https://moonrepo.dev/docs/guides/root-project"
                );
                has_warned_root_tasks = true;
            }

            continue;
        }

        match extract_project_task_ids(&id) {
            (Some(project_id), task_id) => {
                let project = project_graph.get(&project_id)?;
                let task_config = convert_task(task_id.clone(), task)?;

                if let Some(config) = modified_projects.get_mut(&project.root) {
                    config.tasks.insert(task_id, task_config);
                } else {
                    let mut config = project.config.clone();
                    config.tasks.insert(task_id, task_config);

                    modified_projects.insert(&project.root, config);
                }
            }
            (None, task_id) => {
                node_tasks_config
                    .tasks
                    .insert(task_id.clone(), convert_task(task_id, task)?);
                has_modified_global_tasks = true;
            }
        }
    }

    if has_modified_global_tasks {
        let tasks_dir = workspace.root.join(constants::CONFIG_DIRNAME).join("tasks");

        if !tasks_dir.exists() {
            fs::create_dir_all(&tasks_dir)?;
        }

        yaml::write_with_config(tasks_dir.join("node.yml"), &node_tasks_config)?;
    }

    for (project_root, project_config) in modified_projects {
        yaml::write_with_config(
            project_root.join(constants::CONFIG_PROJECT_FILENAME),
            &project_config,
        )?;
    }

    fs::remove_file(&turbo_file)?;

    println!("Successfully migrated from Turborepo to moon!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use moon_utils::string_vec;

    mod globals_conversion {
        use super::*;

        #[test]
        fn converst_deps() {
            let mut config = InheritedTasksConfig {
                implicit_inputs: string_vec!["existing.txt"],
                ..InheritedTasksConfig::default()
            };

            convert_globals(
                &TurboJson {
                    global_dependencies: Some(string_vec!["file.ts", "glob/**/*.js"]),
                    ..TurboJson::default()
                },
                &mut config,
            );

            assert_eq!(
                config.implicit_inputs,
                string_vec!["existing.txt", "file.ts", "glob/**/*.js"]
            );
        }

        #[test]
        fn converst_env() {
            let mut config = InheritedTasksConfig {
                implicit_inputs: string_vec!["$FOO"],
                ..InheritedTasksConfig::default()
            };

            convert_globals(
                &TurboJson {
                    global_env: Some(string_vec!["BAR", "BAZ"]),
                    ..TurboJson::default()
                },
                &mut config,
            );

            assert_eq!(config.implicit_inputs, string_vec!["$FOO", "$BAR", "$BAZ"]);
        }
    }

    mod task_conversion {
        use super::*;

        #[test]
        fn sets_command() {
            let config = convert_task("foo".into(), TurboTask::default()).unwrap();

            assert_eq!(
                config.command,
                TaskCommandArgs::String("moon node run-script foo".into())
            );
        }

        #[test]
        fn converts_deps() {
            let config = convert_task(
                "foo".into(),
                TurboTask {
                    depends_on: Some(string_vec!["normal", "^parent", "project#normal", "$VAR"]),
                    ..TurboTask::default()
                },
            )
            .unwrap();

            assert_eq!(
                config.deps,
                vec![
                    Target::new_self("normal").unwrap(),
                    Target::parse("^:parent").unwrap(),
                    Target::parse("project:normal").unwrap(),
                ]
            );
            assert_eq!(config.inputs, string_vec!["$VAR"]);
        }

        #[test]
        fn doesnt_set_deps_if_empty() {
            let config = convert_task("foo".into(), TurboTask::default()).unwrap();

            assert_eq!(config.deps, Vec::<_>::new());
        }

        #[test]
        fn converts_env_to_inputs() {
            let config = convert_task(
                "foo".into(),
                TurboTask {
                    env: Some(string_vec!["FOO", "BAR"]),
                    ..TurboTask::default()
                },
            )
            .unwrap();

            assert_eq!(config.inputs, string_vec!["$FOO", "$BAR"]);
        }

        #[test]
        fn inherits_inputs() {
            let config = convert_task(
                "foo".into(),
                TurboTask {
                    inputs: Some(string_vec!["file.ts", "some/folder", "some/glob/**/*"]),
                    ..TurboTask::default()
                },
            )
            .unwrap();

            assert_eq!(
                config.inputs,
                string_vec!["file.ts", "some/folder", "some/glob/**/*"]
            );
        }

        #[test]
        fn converts_outputs() {
            let config = convert_task(
                "foo".into(),
                TurboTask {
                    outputs: Some(string_vec![
                        "dir",
                        "dir/**/*",
                        "dir/**",
                        "dir/*",
                        "dir/*/sub"
                    ]),
                    ..TurboTask::default()
                },
            )
            .unwrap();

            assert_eq!(
                config.outputs,
                string_vec!["dir", "dir/**/*", "dir/**/*", "dir/*", "dir/*/sub"]
            );
        }

        #[test]
        fn doesnt_set_outputs_if_empty() {
            let config = convert_task("foo".into(), TurboTask::default()).unwrap();

            assert_eq!(config.outputs, Vec::<String>::new());
        }

        #[test]
        fn sets_local() {
            let config = convert_task(
                "foo".into(),
                TurboTask {
                    persistent: Some(true),
                    ..TurboTask::default()
                },
            )
            .unwrap();

            assert!(config.local);
        }

        #[test]
        fn sets_cache() {
            let config = convert_task(
                "foo".into(),
                TurboTask {
                    cache: Some(false),
                    ..TurboTask::default()
                },
            )
            .unwrap();

            assert_eq!(config.options.cache, Some(false));
        }
    }
}
