use crate::errors::ProjectError;
use moon_common::{consts, Id};
use moon_config::{
    format_error_line, format_figment_errors, ConfigError, DependencyConfig, DependencyScope,
    FilePath, InheritedTasksConfig, InheritedTasksManager, ProjectConfig, ProjectDependsOn,
    ProjectLanguage, ProjectType,
};
use moon_file_group::{FileGroup, FileGroupError};
use moon_logger::{debug, trace, Logable};
use moon_query::{Condition, Criteria, Field, LogicalOperator, QueryError, Queryable};
use moon_target::Target;
use moon_task::{Task, TouchedFilePaths};
use moon_utils::path;
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use starbase_styles::color;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use strum::Display;

type FileGroupsMap = FxHashMap<Id, FileGroup>;

type ProjectDependenciesMap = FxHashMap<Id, ProjectDependency>;

type TasksMap = BTreeMap<Id, Task>;

// moon.yml
fn load_project_config(
    log_target: &str,
    project_root: &Path,
    project_source: &str,
) -> Result<ProjectConfig, ProjectError> {
    let config_path = project_root.join(consts::CONFIG_PROJECT_FILENAME);

    trace!(
        target: log_target,
        "Attempting to find {} in {}",
        color::file(consts::CONFIG_PROJECT_FILENAME),
        color::path(project_root),
    );

    if config_path.exists() {
        return ProjectConfig::load(config_path).map_err(|e| {
            ProjectError::InvalidConfigFile(
                project_source.to_owned(),
                if let ConfigError::FailedValidation(valids) = e {
                    format_figment_errors(valids)
                } else {
                    format_error_line(e.to_string())
                },
            )
        });
    }

    Ok(ProjectConfig::default())
}

fn create_file_groups_from_config(
    log_target: &str,
    source: &str,
    config: &ProjectConfig,
    global_tasks_config: &InheritedTasksConfig,
) -> Result<FileGroupsMap, FileGroupError> {
    let mut file_groups = FxHashMap::<Id, FileGroup>::default();

    debug!(target: log_target, "Creating file groups");

    // Add global file groups first
    for (group_id, files) in &global_tasks_config.file_groups {
        file_groups.insert(
            Id::raw(group_id),
            FileGroup::new_with_source(group_id, source, files)?,
        );
    }

    // Override global configs with local
    for (group_id, files) in &config.file_groups {
        let group_id = Id::raw(group_id); // TODO

        if let Some(existing_group) = file_groups.get_mut(&group_id) {
            debug!(
                target: log_target,
                "Merging file group {} with global config",
                color::id(group_id)
            );

            existing_group.set_patterns(source, files);
        } else {
            file_groups.insert(
                group_id.clone(),
                FileGroup::new_with_source(group_id, source, files)?,
            );
        }
    }

    Ok(file_groups)
}

fn create_dependencies_from_config(
    log_target: &str,
    config: &ProjectConfig,
) -> ProjectDependenciesMap {
    let mut deps = FxHashMap::default();

    debug!(target: log_target, "Creating dependencies");

    for dep_cfg in &config.depends_on {
        match dep_cfg {
            ProjectDependsOn::String(id) => {
                deps.insert(
                    Id::raw(id),
                    ProjectDependency {
                        id: Id::raw(id),
                        ..ProjectDependency::default()
                    },
                );
            }
            ProjectDependsOn::Object(cfg) => {
                deps.insert(Id::raw(&cfg.id), ProjectDependency::from_config(cfg));
            }
        }
    }

    deps
}

fn create_tasks_from_config(
    log_target: &str,
    project_id: &Id,
    project_config: &ProjectConfig,
    global_tasks_config: &InheritedTasksConfig,
) -> Result<TasksMap, ProjectError> {
    let mut tasks = BTreeMap::<Id, Task>::new();

    debug!(target: log_target, "Creating tasks");

    // Gather inheritance configs
    let mut include_all = true;
    let mut include: FxHashSet<Id> = FxHashSet::default();
    let mut exclude: FxHashSet<Id> = FxHashSet::default();
    let mut rename: FxHashMap<Id, Id> = FxHashMap::default();

    if let Some(rename_config) = &project_config.workspace.inherited_tasks.rename {
        for (k, v) in rename_config {
            rename.insert(Id::raw(k), Id::raw(v));
        }
    }

    if let Some(include_config) = &project_config.workspace.inherited_tasks.include {
        include_all = false;

        for i in include_config {
            include.insert(Id::raw(i));
        }
    }

    if let Some(exclude_config) = &project_config.workspace.inherited_tasks.exclude {
        for i in exclude_config {
            exclude.insert(Id::raw(i));
        }
    }

    // Add global tasks first while taking inheritance config into account
    for (task_id, task_config) in &global_tasks_config.tasks {
        let task_id = Id::raw(task_id); // TODO

        // None = Include all
        // [] = Include none
        // ["a"] = Include "a"
        if !include_all {
            if include.is_empty() {
                trace!(
                    target: log_target,
                    "Not inheriting global tasks, empty `include` set"
                );

                break;
            } else if !include.contains(&task_id) {
                trace!(
                    target: log_target,
                    "Not inheriting global task {}, not explicitly included",
                    color::id(task_id)
                );

                continue;
            }
        }

        // None, [] = Exclude none
        // ["a"] = Exclude "a"
        if !exclude.is_empty() && exclude.contains(&task_id) {
            trace!(
                target: log_target,
                "Not inheriting global task {}, explicitly excluded",
                color::id(task_id)
            );

            continue;
        }

        let task_name = if let Some(renamed_task_id) = rename.get(&task_id) {
            trace!(
                target: log_target,
                "Renaming global task {} to {}",
                color::id(task_id),
                color::id(renamed_task_id)
            );

            renamed_task_id.to_owned()
        } else {
            trace!(
                target: log_target,
                "Inheriting global task {}",
                color::id(&task_id)
            );

            task_id
        };

        tasks.insert(
            task_name.to_owned(),
            Task::from_config(Target::new(project_id, task_name)?, task_config)?,
        );
    }

    // Add local tasks second
    for (task_id, task_config) in &project_config.tasks {
        let task_id = Id::raw(task_id); // TODO

        if let Some(existing_task) = tasks.get_mut(&task_id) {
            debug!(
                target: log_target,
                "Merging task {} with global config",
                color::id(task_id)
            );

            // Task already exists, so merge with it
            existing_task.merge(task_config)?;
        } else {
            // Insert a new task
            tasks.insert(
                task_id.clone(),
                Task::from_config(Target::new(project_id, task_id)?, task_config)?,
            );
        }
    }

    Ok(tasks)
}

#[derive(Clone, Debug, Default, Deserialize, Display, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectDependencySource {
    #[default]
    #[strum(serialize = "explicit")]
    Explicit,

    #[strum(serialize = "implicit")]
    Implicit,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectDependency {
    pub id: Id,
    pub scope: DependencyScope,
    pub source: ProjectDependencySource,
    pub via: Option<String>,
}

impl ProjectDependency {
    pub fn from_config(config: &DependencyConfig) -> ProjectDependency {
        ProjectDependency {
            id: Id::raw(&config.id),
            scope: config.scope.clone(),
            via: config.via.clone(),
            ..ProjectDependency::default()
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Project {
    /// Unique alias of the project, alongside its official ID.
    /// This is typically reserved for language specific semantics, like `name` from `package.json`.
    pub alias: Option<String>,

    /// Project configuration loaded from "moon.yml", if it exists.
    pub config: ProjectConfig,

    /// List of other projects this project depends on.
    pub dependencies: ProjectDependenciesMap,

    /// File groups specific to the project. Inherits all file groups from the global config.
    pub file_groups: FileGroupsMap,

    /// Unique ID for the project. Is the LHS of the `projects` setting.
    pub id: Id,

    /// Task configuration that was inherited from the global scope.
    pub inherited_config: InheritedTasksConfig,

    /// Primary programming language of the project.
    pub language: ProjectLanguage,

    /// Logging target label.
    #[serde(skip)]
    pub log_target: String,

    /// Absolute path to the project's root folder.
    pub root: PathBuf,

    /// Relative path of the project from the workspace root. Is the RHS of the `projects` setting.
    pub source: FilePath,

    /// Tasks specific to the project. Inherits all tasks from the global config.
    pub tasks: TasksMap,

    /// The type of project.
    #[serde(rename = "type")]
    pub type_of: ProjectType,
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config
            && self.file_groups == other.file_groups
            && self.id == other.id
            && self.root == other.root
            && self.source == other.source
            && self.tasks == other.tasks
    }
}

impl Logable for Project {
    fn get_log_target(&self) -> &str {
        &self.log_target
    }
}

impl Project {
    pub fn new<F>(
        id: &Id,
        source: &str,
        workspace_root: &Path,
        inherited_tasks: &InheritedTasksManager,
        detect_language: F,
    ) -> Result<Project, ProjectError>
    where
        F: FnOnce(&Path) -> ProjectLanguage,
    {
        let log_target = format!("moon:project:{id}");
        let source = path::normalize_separators(source);

        // For the root-level project, the "." dot actually causes
        // a ton of unwanted issues, so just use workspace root directly.
        let root = if source.is_empty() || source == "." {
            workspace_root.to_owned()
        } else {
            workspace_root.join(&source)
        };

        debug!(
            target: &log_target,
            "Loading project from {} (id = {}, path = {})",
            color::path(&root),
            color::id(id),
            color::file(&source),
        );

        if !root.exists() {
            return Err(ProjectError::MissingProjectAtSource(source));
        }

        let config = load_project_config(&log_target, &root, &source)?;
        let language = if matches!(config.language, ProjectLanguage::Unknown) {
            detect_language(&root)
        } else {
            config.language.clone()
        };
        let platform = config.platform.unwrap_or_else(|| language.clone().into());

        let global_tasks = inherited_tasks.get_inherited_config(
            &platform,
            &language,
            &config.type_of,
            &config.tags,
        );
        let file_groups =
            create_file_groups_from_config(&log_target, &source, &config, &global_tasks)?;
        let dependencies = create_dependencies_from_config(&log_target, &config);
        let tasks = create_tasks_from_config(&log_target, id, &config, &global_tasks)?;

        Ok(Project {
            alias: None,
            dependencies,
            file_groups,
            id: id.to_owned(),
            language,
            log_target,
            root,
            source,
            tasks,
            type_of: config.type_of,
            inherited_config: global_tasks,
            config,
        })
    }

    /// Return a list of project IDs this project depends on.
    pub fn get_dependency_ids(&self) -> Vec<&Id> {
        self.dependencies.keys().collect::<Vec<_>>()
    }

    /// Return a task with the defined ID.
    pub fn get_task(&self, task_id: &str) -> Result<&Task, ProjectError> {
        let task_id = Id::raw(task_id);

        self.tasks
            .get(&task_id)
            .ok_or_else(|| ProjectError::UnconfiguredTask(task_id.to_string(), self.id.to_string()))
    }

    /// Return true if this project is affected based on touched files.
    /// Since the project is a folder, we check if a file starts with the root.
    pub fn is_affected(&self, touched_files: &TouchedFilePaths) -> bool {
        for file in touched_files {
            if file.starts_with(&self.source) {
                return true;
            }
        }

        false
    }
}

impl Queryable for Project {
    /// Return true if this project matches the given query criteria.
    fn matches_criteria(&self, query: &Criteria) -> Result<bool, QueryError> {
        let match_all = matches!(query.op, LogicalOperator::And);
        let mut matched_any = false;

        for condition in &query.conditions {
            let matches = match condition {
                Condition::Field { field, .. } => {
                    let result = match field {
                        Field::Language(langs) => condition.matches_enum(langs, &self.language),
                        Field::Project(ids) => condition.matches(ids, &self.id),
                        Field::ProjectAlias(aliases) => {
                            if let Some(alias) = &self.alias {
                                condition.matches(aliases, alias)
                            } else {
                                Ok(false)
                            }
                        }
                        Field::ProjectSource(sources) => condition.matches(sources, &self.source),
                        Field::ProjectType(types) => condition.matches_enum(types, &self.type_of),
                        Field::Tag(tags) => condition.matches_list(
                            tags,
                            &self
                                .config
                                .tags
                                .iter()
                                .map(|t| t.to_string())
                                .collect::<Vec<_>>(),
                        ),
                        Field::Task(ids) => Ok(self
                            .tasks
                            .values()
                            .any(|task| condition.matches(ids, &task.id).unwrap_or_default())),
                        Field::TaskPlatform(platforms) => Ok(self.tasks.values().any(|task| {
                            condition
                                .matches_enum(platforms, &task.platform)
                                .unwrap_or_default()
                        })),
                        Field::TaskType(types) => Ok(self.tasks.values().any(|task| {
                            condition
                                .matches_enum(types, &task.type_of)
                                .unwrap_or_default()
                        })),
                    };

                    result?
                }
                Condition::Criteria { criteria } => self.matches_criteria(criteria)?,
            };

            if matches {
                matched_any = true;

                if match_all {
                    continue;
                } else {
                    break;
                }
            } else if match_all {
                return Ok(false);
            }
        }

        // No matches using the OR condition
        if !matched_any {
            return Ok(false);
        }

        Ok(true)
    }
}
