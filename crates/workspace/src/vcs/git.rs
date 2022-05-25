use crate::errors::WorkspaceError;
use crate::vcs::{TouchedFiles, Vcs, VcsResult};
use async_trait::async_trait;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use moon_utils::fs;
use moon_utils::process::{output_to_string, output_to_trimmed_string, Command};
use regex::Regex;
use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

pub struct Git {
    default_branch: String,
    ignore: Option<Gitignore>,
    working_dir: PathBuf,
}

impl Git {
    pub fn new(default_branch: &str, working_dir: &Path) -> VcsResult<Self> {
        let mut ignore: Option<Gitignore> = None;
        let ignore_path = working_dir.join(".gitignore");

        if ignore_path.exists() {
            let mut builder = GitignoreBuilder::new(working_dir);

            if let Some(error) = builder.add(ignore_path) {
                return Err(WorkspaceError::Ignore(error));
            }

            ignore = Some(builder.build().map_err(WorkspaceError::Ignore)?);
        }

        Ok(Git {
            default_branch: String::from(default_branch),
            ignore,
            working_dir: working_dir.to_path_buf(),
        })
    }

    fn is_file_ignored(&self, file: &str) -> bool {
        if self.ignore.is_some() {
            self.ignore
                .as_ref()
                .unwrap()
                .matched(file, false)
                .is_ignore()
        } else {
            false
        }
    }

    async fn run_command(&self, command: &mut Command, trim: bool) -> VcsResult<String> {
        let output = command.exec_capture_output().await?;

        if trim {
            return Ok(output_to_trimmed_string(&output.stdout));
        }

        Ok(output_to_string(&output.stdout))
    }
}

#[async_trait]
impl Vcs for Git {
    fn create_command(&self, args: Vec<&str>) -> Command {
        let mut cmd = Command::new("git");
        cmd.args(args).cwd(&self.working_dir);
        cmd
    }

    async fn get_local_branch(&self) -> VcsResult<String> {
        self.run_command(
            &mut self.create_command(vec!["branch", "--show-current"]),
            true,
        )
        .await
    }

    async fn get_local_branch_revision(&self) -> VcsResult<String> {
        self.run_command(&mut self.create_command(vec!["rev-parse", "HEAD"]), true)
            .await
    }

    fn get_default_branch(&self) -> &str {
        &self.default_branch
    }

    async fn get_default_branch_revision(&self) -> VcsResult<String> {
        self.run_command(
            &mut self.create_command(vec!["rev-parse", &self.default_branch]),
            true,
        )
        .await
    }

    async fn get_file_hashes(&self, files: &[String]) -> VcsResult<BTreeMap<String, String>> {
        let mut objects = vec![];

        for file in files {
            if !self.is_file_ignored(file) {
                objects.push(file.clone());
            }
        }

        let output = self
            .create_command(vec!["hash-object", "--stdin-paths"])
            .exec_capture_output_with_input(&objects.join("\n"))
            .await?;
        let output = output_to_trimmed_string(&output.stdout);

        let mut map = BTreeMap::new();

        for (index, hash) in output.split('\n').enumerate() {
            if !hash.is_empty() {
                map.insert(files[index].clone(), hash.to_owned());
            }
        }

        Ok(map)
    }

    async fn get_file_tree_hashes(&self, dir: &str) -> VcsResult<BTreeMap<String, String>> {
        let output = self
            .run_command(
                &mut self.create_command(vec!["ls-tree", "HEAD", "-r", dir]),
                true,
            )
            .await?;
        let mut map = BTreeMap::new();

        for line in output.split('\n') {
            // <mode> <type> <hash>\t<file>
            let parts = line.split(' ');
            // <hash>\t<file>
            let mut last_parts = parts.last().unwrap().split('\t');
            let hash = last_parts.next().unwrap();
            let file = last_parts.next().unwrap();

            if !self.is_file_ignored(file) {
                map.insert(file.to_owned(), hash.to_owned());
            }
        }

        Ok(map)
    }

    // https://git-scm.com/docs/git-status#_short_format
    async fn get_touched_files(&self) -> VcsResult<TouchedFiles> {
        let output = self
            .run_command(
                &mut self.create_command(vec![
                    "status",
                    "--porcelain",
                    "--untracked-files",
                    // We use this option so that file names with special characters
                    // are displayed as-is and are not quoted/escaped
                    "-z",
                ]),
                false,
            )
            .await?;

        if output.is_empty() {
            return Ok(TouchedFiles::default());
        }

        let mut added = HashSet::new();
        let mut deleted = HashSet::new();
        let mut modified = HashSet::new();
        let mut untracked = HashSet::new();
        let mut staged = HashSet::new();
        let mut unstaged = HashSet::new();
        let mut all = HashSet::new();
        let xy_regex = Regex::new(r"^(M|T|A|D|R|C|U|\?|!| )(M|T|A|D|R|C|U|\?|!| ) ").unwrap();

        // Lines are terminated by a NUL byte:
        //  XY file\0
        //  XY file\0orig_file\0
        for line in output.split('\0') {
            if line.is_empty() {
                continue;
            }

            // orig_file\0
            if !xy_regex.is_match(line) {
                continue;
            }

            // XY file\0
            let mut chars = line.chars();
            let x = chars.next().unwrap_or_default();
            let y = chars.next().unwrap_or_default();
            let file = String::from(&line[3..]);

            match x {
                'A' | 'C' => {
                    added.insert(file.clone());
                    staged.insert(file.clone());
                }
                'D' => {
                    deleted.insert(file.clone());
                    staged.insert(file.clone());
                }
                'M' | 'R' => {
                    modified.insert(file.clone());
                    staged.insert(file.clone());
                }
                _ => {}
            }

            match y {
                'A' | 'C' => {
                    added.insert(file.clone());
                    unstaged.insert(file.clone());
                }
                'D' => {
                    deleted.insert(file.clone());
                    unstaged.insert(file.clone());
                }
                'M' | 'R' => {
                    modified.insert(file.clone());
                    unstaged.insert(file.clone());
                }
                '?' => {
                    untracked.insert(file.clone());
                }
                _ => {}
            }

            all.insert(file.clone());
        }

        Ok(TouchedFiles {
            added,
            all,
            deleted,
            modified,
            staged,
            unstaged,
            untracked,
        })
    }

    async fn get_touched_files_against_previous_revision(
        &self,
        revision: &str,
    ) -> VcsResult<TouchedFiles> {
        let rev = if self.is_default_branch(revision) {
            "HEAD"
        } else {
            revision
        };

        Ok(self
            .get_touched_files_between_revisions(&format!("{}~1", rev), rev)
            .await?)
    }

    async fn get_touched_files_between_revisions(
        &self,
        base_revision: &str,
        revision: &str,
    ) -> VcsResult<TouchedFiles> {
        let output = self
            .run_command(
                &mut self.create_command(vec![
                    "--no-pager",
                    "diff",
                    "--name-status",
                    "--no-color",
                    "--relative",
                    // We use this option so that file names with special characters
                    // are displayed as-is and are not quoted/escaped
                    "-z",
                    base_revision,
                    revision,
                ]),
                false,
            )
            .await?;

        if output.is_empty() {
            return Ok(TouchedFiles::default());
        }

        let mut added = HashSet::new();
        let mut deleted = HashSet::new();
        let mut modified = HashSet::new();
        let mut staged = HashSet::new();
        let mut all = HashSet::new();
        let x_with_score_regex = Regex::new(r"^(C|M|R)(\d{3})$").unwrap();
        let x_regex = Regex::new(r"^(A|D|M|T|U|X)$").unwrap();
        let mut last_status = "A";

        // Lines AND statuses are terminated by a NUL byte
        //  X\0file\0
        //  X000\0file\0
        //  X000\0file\0renamed_file\0
        for line in output.split('\0') {
            if line.is_empty() {
                continue;
            }

            // X\0
            // X000\0
            if x_with_score_regex.is_match(line) || x_regex.is_match(line) {
                last_status = &line[0..1];
                continue;
            }

            let x = last_status.chars().next().unwrap();
            let file = line.to_owned();

            match x {
                'A' | 'C' => {
                    added.insert(file.clone());
                    staged.insert(file.clone());
                }
                'D' => {
                    deleted.insert(file.clone());
                    staged.insert(file.clone());
                }
                'M' | 'R' => {
                    modified.insert(file.clone());
                    staged.insert(file.clone());
                }
                _ => {}
            }

            all.insert(file.clone());
        }

        Ok(TouchedFiles {
            added,
            all,
            deleted,
            modified,
            staged,
            unstaged: HashSet::new(),
            untracked: HashSet::new(),
        })
    }

    fn is_default_branch(&self, branch: &str) -> bool {
        if self.default_branch == branch {
            return true;
        }

        if self.default_branch.contains('/') {
            return self.default_branch.ends_with(&format!("/{}", branch));
        }

        false
    }

    fn is_enabled(&self) -> bool {
        fs::find_upwards(".git", &self.working_dir).is_some()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use moon_utils::string_vec;
    use moon_utils::test::create_fixtures_sandbox;

    mod get_file_hashes {
        use super::*;

        #[tokio::test]
        async fn filters_ignored_files() {
            let fixture = create_fixtures_sandbox("ignore");
            let git = Git::new("master", fixture.path()).unwrap();

            let hashes = git
                .get_file_hashes(&string_vec!["foo", "bar", "baz"])
                .await
                .unwrap();

            assert_eq!(
                hashes,
                BTreeMap::from([(
                    "foo".to_owned(),
                    "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391".to_owned()
                )])
            );
        }
    }

    mod get_file_tree_hashes {
        use super::*;

        #[tokio::test]
        async fn filters_ignored_files() {
            let fixture = create_fixtures_sandbox("ignore");
            let git = Git::new("master", fixture.path()).unwrap();

            let hashes = git.get_file_tree_hashes(".").await.unwrap();

            assert_eq!(
                hashes,
                BTreeMap::from([
                    (
                        ".gitignore".to_owned(),
                        "589c59be54beff591804a008c972e76dea31d2d1".to_owned()
                    ),
                    (
                        "foo".to_owned(),
                        "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391".to_owned()
                    )
                ])
            );
        }
    }
}
