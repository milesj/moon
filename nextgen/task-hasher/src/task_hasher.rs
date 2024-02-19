use miette::IntoDiagnostic;
use moon_common::consts::CONFIG_PROJECT_FILENAME;
use moon_common::path::{PathExt, WorkspaceRelativePath, WorkspaceRelativePathBuf};
use moon_common::{color, is_ci};
use moon_config::{HasherConfig, HasherWalkStrategy};
use moon_project::Project;
use moon_task::Task;
use moon_vcs::BoxedVcs;
use rustc_hash::FxHashSet;
use starbase_utils::glob::{self, GlobSet};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use tracing::{debug, warn};

// Hash all inputs for a task, but exclude outputs and moon specific configuration files!
pub struct TaskHasher<'task> {
    pub hasher_config: &'task HasherConfig,
    pub project: &'task Project,
    pub task: &'task Task,
    pub vcs: &'task BoxedVcs,
    pub workspace_root: &'task Path,

    aggregated_files: FxHashSet<PathBuf>,
    processed_files: FxHashSet<WorkspaceRelativePathBuf>,
}

impl<'task> TaskHasher<'task> {
    pub async fn aggregate_inputs(&mut self) -> miette::Result<()> {
        let mut files = FxHashSet::default();

        if !self.task.input_files.is_empty() {
            for input in &self.task.input_files {
                files.insert(input.to_path(self.workspace_root));
            }
        }

        if !self.task.input_globs.is_empty() {
            let use_globs = self.project.root == self.workspace_root
                || matches!(self.hasher_config.walk_strategy, HasherWalkStrategy::Glob);

            // Collect inputs by walking and globbing the file system
            if use_globs {
                files.extend(glob::walk_files(
                    self.workspace_root,
                    &self.task.input_globs,
                )?);

                // Collect inputs by querying VCS
            } else {
                // Using VCS to collect inputs in a project is faster than globbing
                for file in self.vcs.get_file_tree(self.project.source.as_str()).await? {
                    files.insert(file.to_path(self.workspace_root));
                }

                // However that completely ignores workspace level globs,
                // so we must still manually glob those here!
                let workspace_globs = self
                    .task
                    .input_globs
                    .iter()
                    .filter(|g| !g.starts_with(self.project.source.as_str()))
                    .collect::<Vec<_>>();

                if !workspace_globs.is_empty() {
                    files.extend(glob::walk_files(self.workspace_root, workspace_globs)?);
                }
            }
        }

        // Include local file changes so that development builds work.
        // Also run this LAST as it should take highest precedence!
        if !is_ci() {
            for local_file in self.vcs.get_touched_files().await?.all() {
                let local_file = local_file.to_path(self.workspace_root);

                // Deleted files are listed in `git status` but are
                // not valid inputs, so avoid hashing them!
                if local_file.exists() {
                    files.insert(local_file);
                }
            }
        }

        self.aggregated_files = files;

        Ok(())
    }

    pub fn process_files(&mut self) -> miette::Result<()> {
        let mut files = FxHashSet::default();
        let ignore = GlobSet::new(&self.hasher_config.ignore_patterns)?;
        let ignore_missing = GlobSet::new(&self.hasher_config.ignore_missing_patterns)?;

        for path in &self.aggregated_files {
            // We need to use relative paths from the workspace root
            // so that it works the same across all machines
            let rel_path = path.relative_to(&self.workspace_root).into_diagnostic()?;

            // `git hash-object` will fail if you pass an unknown file
            if !path.exists() && self.hasher_config.warn_on_missing_inputs {
                if self.hasher_config.ignore_missing_patterns.is_empty()
                    || !ignore_missing.is_match(path)
                {
                    warn!(
                        "Attempted to hash input {} but it does not exist, skipping",
                        color::rel_path(&rel_path),
                    );
                }

                continue;
            }

            if !path.is_file() {
                warn!(
                    "Attempted to hash input {} but only files can be hashed, try using a glob instead",
                   color::rel_path(&rel_path),
                );

                continue;
            }

            if ignore.is_match(path) {
                debug!(
                    "Not hashing input {} as it matches an ignore pattern",
                    color::rel_path(&rel_path),
                );
            } else {
                files.insert(rel_path);
            }
        }

        // Filter out unwanted files
        if !files.is_empty() {
            let globset = self.task.create_globset()?;

            files.retain(|file| self.is_valid_input_source(&globset, file));
        }

        self.processed_files = files;

        Ok(())
    }

    pub async fn generate_hashes(
        self,
    ) -> miette::Result<BTreeMap<WorkspaceRelativePathBuf, String>> {
        let mut hashed_files = BTreeMap::default();

        if !self.processed_files.is_empty() {
            let files = self
                .processed_files
                .into_iter()
                .map(|file| file.to_string())
                .collect::<Vec<_>>();

            hashed_files.extend(
                self.vcs
                    .get_file_hashes(&files, true, self.hasher_config.batch_size)
                    .await?,
            );
        }

        Ok(hashed_files)
    }

    fn is_valid_input_source(
        &self,
        sources_globset: &GlobSet,
        workspace_relative_path: &WorkspaceRelativePath,
    ) -> bool {
        // Don't invalidate existing hashes when moon.yml changes
        // as we already hash the contents of each task!
        if workspace_relative_path.ends_with(CONFIG_PROJECT_FILENAME) {
            return false;
        }

        // Remove outputs first
        if sources_globset.is_negated(workspace_relative_path.as_str()) {
            return false;
        }

        for output in &self.task.output_files {
            if workspace_relative_path == output || workspace_relative_path.starts_with(output) {
                return false;
            }
        }

        // Filter inputs second
        self.task.input_files.contains(workspace_relative_path)
            || sources_globset.matches(workspace_relative_path.as_str())
    }
}
