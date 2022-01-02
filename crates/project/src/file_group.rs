use crate::errors::ProjectError;
use globwalk::GlobWalkerBuilder;
use moon_utils::fs::is_glob;
use std::fs;
use std::path::PathBuf;

pub struct FileGroup {
    files: Vec<String>,

    project_root: PathBuf,
}

impl FileGroup {
    pub fn new(files: Vec<String>, project_root: PathBuf) -> FileGroup {
        FileGroup {
            files,
            project_root,
        }
    }

    /// Returns the file group as an expanded list of absolute directory paths.
    pub fn dirs(&self) -> Result<Vec<PathBuf>, ProjectError> {
        self.walk(true)
    }

    /// Returns the file group as an expanded list of absolute file paths.
    pub fn files(&self) -> Result<Vec<PathBuf>, ProjectError> {
        self.walk(false)
    }

    fn walk(&self, is_dir: bool) -> Result<Vec<PathBuf>, ProjectError> {
        let mut list = vec![];

        for file in &self.files {
            if is_glob(file) {
                let walker = GlobWalkerBuilder::from_patterns(&self.project_root, &[file])
                    .follow_links(false)
                    .build()?;

                for entry in walker {
                    let entry_path = entry.unwrap(); // Handle error?

                    let allowed = if is_dir {
                        entry_path.file_type().is_dir()
                    } else {
                        entry_path.file_type().is_file()
                    };

                    if allowed {
                        list.push(entry_path.into_path());
                    }
                }
            } else {
                let entry_path = self.project_root.join(file);

                let allowed = match fs::metadata(&entry_path) {
                    Ok(meta) => {
                        if is_dir {
                            meta.is_dir()
                        } else {
                            meta.is_file()
                        }
                    }
                    // Branch exists for logging
                    Err(_) => false,
                };

                if allowed {
                    list.push(entry_path);
                }
            }
        }

        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use moon_utils::test::get_fixtures_dir;

    mod dirs {
        use super::*;

        #[test]
        fn returns_all_dirs() {
            let root = get_fixtures_dir("base");
            let file_group = FileGroup::new(vec!["**/*".to_owned()], root.join("files-and-dirs"));

            assert_eq!(
                file_group.dirs().unwrap(),
                vec![
                    root.join("files-and-dirs/dir"),
                    root.join("files-and-dirs/dir/subdir")
                ]
            );
        }

        #[test]
        fn doesnt_return_files() {
            let root = get_fixtures_dir("base");
            let file_group =
                FileGroup::new(vec!["file.ts".to_owned()], root.join("files-and-dirs"));
            let result: Vec<PathBuf> = vec![];

            assert_eq!(file_group.dirs().unwrap(), result);
        }
    }

    mod files {
        use super::*;

        #[test]
        fn returns_all_files() {
            let root = get_fixtures_dir("base");
            let file_group = FileGroup::new(
                vec!["**/*.{ts,tsx}".to_owned()],
                root.join("files-and-dirs"),
            );

            assert_eq!(
                file_group.files().unwrap(),
                vec![
                    root.join("files-and-dirs/file.ts"),
                    root.join("files-and-dirs/dir/subdir/another.ts"),
                    root.join("files-and-dirs/dir/other.tsx"),
                ]
            );
        }

        #[test]
        fn doesnt_return_dirs() {
            let root = get_fixtures_dir("base");
            let file_group = FileGroup::new(vec!["dir".to_owned()], root.join("files-and-dirs"));
            let result: Vec<PathBuf> = vec![];

            assert_eq!(file_group.files().unwrap(), result);
        }
    }
}
