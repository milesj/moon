use crate::action::{Action, ActionStatus};
use crate::context::ActionContext;
use crate::errors::ActionError;
use moon_config::PackageManager;
use moon_error::map_io_to_fs_error;
use moon_lang_node::{package::PackageJson, NPM};
use moon_logger::{color, debug, warn};
use moon_terminal::{label_checkpoint, Checkpoint};
use moon_utils::{fs, is_offline};
use moon_workspace::Workspace;
use std::sync::Arc;
use tokio::sync::RwLock;

const LOG_TARGET: &str = "moon:action:install-node-deps";

/// Add `packageManager` to root `package.json`.
#[track_caller]
fn add_package_manager(workspace: &Workspace, package_json: &mut PackageJson) -> bool {
    let manager_version = match workspace.config.node.package_manager {
        PackageManager::Npm => format!("npm@{}", workspace.config.node.npm.version),
        PackageManager::Pnpm => format!(
            "pnpm@{}",
            workspace.config.node.pnpm.as_ref().unwrap().version
        ),
        PackageManager::Yarn => format!(
            "yarn@{}",
            workspace.config.node.yarn.as_ref().unwrap().version
        ),
    };

    if manager_version != "npm@inherit"
        && workspace.toolchain.get_node().is_corepack_aware()
        && package_json.set_package_manager(&manager_version)
    {
        debug!(
            target: LOG_TARGET,
            "Adding package manager version to root {}",
            color::file(&NPM.manifest_filename)
        );

        return true;
    }

    false
}

/// Add `engines` constraint to root `package.json`.
fn add_engines_constraint(workspace: &Workspace, package_json: &mut PackageJson) -> bool {
    if workspace.config.node.add_engines_constraint
        && package_json.add_engine("node", &workspace.config.node.version)
    {
        debug!(
            target: LOG_TARGET,
            "Adding engines version constraint to root {}",
            color::file(&NPM.manifest_filename)
        );

        return true;
    }

    false
}

pub async fn install_node_deps(
    _action: &mut Action,
    context: &ActionContext,
    workspace: Arc<RwLock<Workspace>>,
) -> Result<ActionStatus, ActionError> {
    // Writes root `package.json`
    {
        let workspace = workspace.write().await;
        let mut root_package_json = PackageJson::read(workspace.root.join("package.json")).await?;

        if let Some(root_package_json_mut) = &mut root_package_json {
            let added_manager = add_package_manager(&workspace, root_package_json_mut);
            let added_engines = add_engines_constraint(&workspace, root_package_json_mut);

            if added_manager || added_engines {
                root_package_json_mut.save().await?;
            }
        }
    }

    // Read only
    {
        let workspace = workspace.read().await;
        let mut cache = workspace.cache.cache_workspace_state().await?;
        let manager = workspace.toolchain.get_node().get_package_manager();
        let node_config = &workspace.config.node;

        // Create nvm/nodenv config file
        if let Some(version_manager) = &node_config.sync_version_manager_config {
            let rc_name = version_manager.get_config_filename();
            let rc_path = workspace.root.join(&rc_name);

            fs::write(&rc_path, &node_config.version).await?;

            debug!(
                target: LOG_TARGET,
                "Syncing Node.js version to root {}",
                color::file(&rc_name)
            );
        }

        // Get the last modified time of the root lockfile
        let lockfile_name = manager.get_lock_filename();
        let lockfile = workspace.root.join(&lockfile_name);
        let mut last_modified = 0;

        if lockfile.exists() {
            let lockfile_metadata = fs::metadata(&lockfile).await?;

            last_modified = cache.to_millis(
                lockfile_metadata
                    .modified()
                    .map_err(|e| map_io_to_fs_error(e, lockfile.clone()))?,
            );
        }

        // If a package.json has been modified manually, we should account for that
        let has_modified_manifests = context
            .touched_files
            .iter()
            .any(|f| f.ends_with(&NPM.manifest_filename) || f.ends_with(&lockfile_name));

        // Install deps if the lockfile has been modified
        // since the last time dependencies were installed!
        if has_modified_manifests
            || last_modified == 0
            || last_modified > cache.item.last_node_install_time
        {
            debug!(target: LOG_TARGET, "Installing Node.js dependencies");

            if is_offline() {
                warn!(
                    target: LOG_TARGET,
                    "No internet connection, assuming offline and skipping install"
                );

                return Ok(ActionStatus::Skipped);
            }

            let install_command = match workspace.config.node.package_manager {
                PackageManager::Npm => "npm install",
                PackageManager::Pnpm => "pnpm install",
                PackageManager::Yarn => "yarn install",
            };

            println!("{}", label_checkpoint(install_command, Checkpoint::Pass));

            manager.install_dependencies(&workspace.toolchain).await?;

            if node_config.dedupe_on_lockfile_change {
                debug!(target: LOG_TARGET, "Dedupeing dependencies");

                manager.dedupe_dependencies(&workspace.toolchain).await?;
            }

            // Update the cache with the timestamp
            cache.item.last_node_install_time = cache.now_millis();
            cache.save().await?;

            return Ok(ActionStatus::Passed);
        }

        debug!(
            target: LOG_TARGET,
            "Lockfile has not changed since last install, skipping Node.js dependencies",
        );
    }

    Ok(ActionStatus::Skipped)
}
