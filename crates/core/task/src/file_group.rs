use crate::errors::FileGroupError;
use common_path::common_path_all;
use moon_logger::{color, map_list, trace};
use moon_utils::{glob, path};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const LOG_TARGET: &str = "moon:task:file-group";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FileGroup {
    pub files: Vec<String>,

    pub id: String,
}

impl FileGroup {
    pub fn new(id: &str, files: Vec<String>) -> FileGroup {
        trace!(
            target: LOG_TARGET,
            "Creating file group {} with files: {}",
            color::id(id),
            map_list(&files, |f| color::file(f))
        );

        FileGroup {
            files,
            id: id.to_owned(),
        }
    }

    pub fn merge(&mut self, files: Vec<String>) {
        // Local files should always override global
        self.files = files;
    }

    // Returns the file group as-is, with each file converted to an absolute path.
    // Paths and groups will be separated as they have different semantics.
    pub fn all(
        &self,
        workspace_root: &Path,
        project_root: &Path,
    ) -> Result<(Vec<PathBuf>, Vec<String>), FileGroupError> {
        let mut paths = vec![];
        let mut globs = vec![];

        for file in &self.files {
            let result = path::expand_to_workspace_relative(file, workspace_root, project_root);

            if glob::is_glob(file) {
                globs.push(glob::normalize(result)?);
            } else {
                paths.push(result);
            }
        }

        Ok((paths, globs))
    }

    /// Return the file group as an expanded list of directory paths.
    /// If a glob is detected, it will aggregate all directories found.
    pub fn dirs(
        &self,
        workspace_root: &Path,
        project_root: &Path,
    ) -> Result<Vec<PathBuf>, FileGroupError> {
        self.walk(true, workspace_root, project_root)
    }

    /// Return the file group as an expanded list of file paths.
    /// If a glob is detected, it will aggregate all files found.
    pub fn files(
        &self,
        workspace_root: &Path,
        project_root: &Path,
    ) -> Result<Vec<PathBuf>, FileGroupError> {
        self.walk(false, workspace_root, project_root)
    }

    /// Return the file group as a list of file globs (as-is),
    /// relative to the project root.
    pub fn globs(
        &self,
        workspace_root: &Path,
        project_root: &Path,
    ) -> Result<Vec<String>, FileGroupError> {
        let mut globs = vec![];

        for file in &self.files {
            if glob::is_glob(file) {
                globs.push(glob::normalize(path::expand_to_workspace_relative(
                    file,
                    workspace_root,
                    project_root,
                ))?);
            }
        }

        if globs.is_empty() {
            return Err(FileGroupError::NoGlobs(self.id.to_owned()));
        }

        Ok(globs)
    }

    /// Return the file group reduced down to the lowest common directory.
    /// If the reduced directories is not =1, the project root "." will be returned.
    pub fn root(
        &self,
        workspace_root: &Path,
        project_root: &Path,
    ) -> Result<PathBuf, FileGroupError> {
        let dirs = self.dirs(workspace_root, project_root)?;
        let project_source = project_root.strip_prefix(workspace_root).unwrap();

        if !dirs.is_empty() {
            let paths: Vec<&Path> = dirs
                .iter()
                .filter(|d| d.starts_with(project_source))
                .map(|d| d.strip_prefix(project_source).unwrap())
                .collect();
            let common_dir = common_path_all(paths);

            if let Some(dir) = common_dir {
                return Ok(project_source.join(dir));
            }
        }

        Ok(".".into())
    }

    fn walk(
        &self,
        is_dir: bool,
        workspace_root: &Path,
        project_root: &Path,
    ) -> Result<Vec<PathBuf>, FileGroupError> {
        let (paths, globs) = self.all(workspace_root, project_root)?;
        let mut list = vec![];

        // Paths are relative from workspace root!
        for path in paths {
            let allowed = if is_dir {
                workspace_root.join(&path).is_dir()
            } else {
                workspace_root.join(&path).is_file()
            };

            if allowed {
                list.push(path::normalize(path));
            }
        }

        if !globs.is_empty() {
            // Glob results are absolute paths!
            for path in glob::walk(workspace_root, &globs)? {
                let allowed = if is_dir {
                    path.is_dir()
                } else {
                    path.is_file()
                };

                if allowed {
                    list.push(path::normalize(path.strip_prefix(workspace_root).unwrap()));
                }
            }
        }

        Ok(list)
    }
}
