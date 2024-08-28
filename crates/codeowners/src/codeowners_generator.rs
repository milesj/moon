use moon_config::{CodeownersConfig, OwnersConfig, OwnersPaths, VcsProvider};
use starbase_utils::fs::{self, FsError};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::{debug, trace};

pub struct CodeownersGenerator {
    pub file_path: PathBuf,
    file: File,
    provider: VcsProvider,
}

impl CodeownersGenerator {
    pub fn new(
        workspace_root: &Path,
        provider: VcsProvider,
    ) -> miette::Result<CodeownersGenerator> {
        debug!("Aggregating code owners");

        let file_path = workspace_root.join(match provider {
            VcsProvider::GitHub => ".github/CODEOWNERS",
            VcsProvider::GitLab => ".gitlab/CODEOWNERS",
            _ => "CODEOWNERS",
        });

        let mut generator = CodeownersGenerator {
            file: fs::create_file(&file_path)?,
            file_path,
            provider,
        };

        generator.write("# Automatically generated by moon. DO NOT MODIFY!")?;
        generator.write("# https://moonrepo.dev/docs/guides/codeowners")?;

        Ok(generator)
    }

    pub fn add_project_entry(
        &mut self,
        id: &str,
        source: &str,
        config: &OwnersConfig,
        root_config: &CodeownersConfig,
    ) -> miette::Result<()> {
        if config.paths.is_empty() {
            return Ok(());
        }

        trace!(project = id, source, "Adding project entries");

        self.write("")?;

        // Render the header
        self.write(format!("# {}", id))?;

        let required_approvals = config
            .required_approvals
            .or(root_config.required_approvals)
            .unwrap_or(0);

        match &self.provider {
            VcsProvider::Bitbucket => {
                if required_approvals > 0 {
                    if let Some(default_owner) = &config.default_owner {
                        self.write(format!(
                            "Check({} >= {})",
                            default_owner, required_approvals
                        ))?;
                    }
                }
            }

            VcsProvider::GitLab => {
                let mut header = format!("[{id}]");

                if config.optional {
                    header = format!("^{header}")
                }

                if required_approvals > 0 {
                    header = format!("{header}[{}]", required_approvals);
                }

                if matches!(config.paths, OwnersPaths::List(_)) {
                    header = format!("{header} {}", config.default_owner.as_ref().unwrap());
                }

                self.write(header)?;
            }
            _ => {}
        };

        // Render the owner entries
        let root = PathBuf::from("/").join(source);

        match &config.paths {
            OwnersPaths::List(paths) => {
                for path in paths {
                    if matches!(self.provider, VcsProvider::GitLab) {
                        self.write(self.format_path(root.join(path)))?;
                    } else {
                        self.write(format!(
                            "{} {}",
                            self.format_path(root.join(path)),
                            config.default_owner.as_ref().unwrap()
                        ))?;
                    }
                }
            }
            OwnersPaths::Map(map) => {
                for (path, owners) in map {
                    if owners.is_empty() {
                        self.write(format!(
                            "{} {}",
                            self.format_path(root.join(path)),
                            config.default_owner.as_ref().unwrap()
                        ))?;
                    } else {
                        self.write(format!(
                            "{} {}",
                            self.format_path(root.join(path)),
                            owners.join(" ")
                        ))?;
                    }
                }
            }
        };

        Ok(())
    }

    pub fn add_workspace_entries(&mut self, config: &CodeownersConfig) -> miette::Result<()> {
        if config.global_paths.is_empty() {
            return Ok(());
        }

        trace!("Adding workspace entries");

        self.write("")?;
        self.write("# (workspace)")?;

        for (path, owners) in &config.global_paths {
            if !owners.is_empty() {
                self.write(format!(
                    "{} {}",
                    self.format_path(PathBuf::from(path)),
                    owners.join(" ")
                ))?;
            }
        }

        Ok(())
    }

    pub fn cleanup(self) -> miette::Result<()> {
        debug!(file = ?self.file_path, "Removing CODEOWNERS file");

        drop(self.file);

        fs::remove_file(&self.file_path)?;

        Ok(())
    }

    pub fn generate(mut self) -> miette::Result<()> {
        debug!(file = ?self.file_path, "Generating and writing CODEOWNERS file");

        self.file.flush().map_err(|error| FsError::Create {
            path: self.file_path.to_path_buf(),
            error: Box::new(error),
        })?;

        Ok(())
    }

    fn format_path(&self, path: PathBuf) -> String {
        path.to_string_lossy()
            // Always use forward slashes
            .replace('\\', "/")
            // Escape spaces
            .replace(' ', "\\ ")
    }

    fn write<T: AsRef<str>>(&mut self, message: T) -> miette::Result<()> {
        writeln!(self.file, "{}", message.as_ref()).map_err(|error| FsError::Create {
            path: self.file_path.to_path_buf(),
            error: Box::new(error),
        })?;

        Ok(())
    }
}
