use moon_error::MoonError;
// use moon_project::ProjectError;
// use moon_task::{TargetError, TaskError};
// use moon_toolchain::ToolchainError;
// use moon_vcs::VcsError;
// use moon_workspace::WorkspaceError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ActionError {
    #[error(transparent)]
    Moon(#[from] MoonError),
    // #[error(transparent)]
    // Project(#[from] ProjectError),

    // #[error(transparent)]
    // Task(#[from] TaskError),

    // #[error(transparent)]
    // Target(#[from] TargetError),

    // #[error(transparent)]
    // Toolchain(#[from] ToolchainError),

    // #[error(transparent)]
    // Vcs(#[from] VcsError),

    // #[error(transparent)]
    // Workspace(#[from] WorkspaceError),
}
