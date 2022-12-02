use mimalloc::MiMalloc;
use moon_cli::{run_cli, BIN_NAME};
use moon_constants::CONFIG_DIRNAME;
use moon_node_lang::NODE;
use moon_utils::path;
use std::env;
use std::path::{Path, PathBuf};
use tokio::process::Command;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg(target_os = "linux")]
fn get_global_lookups(home_dir: &Path) -> Vec<PathBuf> {
    let mut lookups = vec![];

    // Node
    lookups.push(home_dir.join(".nvm/versions/node"));
    lookups.push(home_dir.join(".local/share/pnpm"));
    lookups.push(home_dir.join(".config/yarn"));

    lookups
}

#[cfg(target_os = "macos")]
fn get_global_lookups(home_dir: &Path) -> Vec<PathBuf> {
    let mut lookups = vec![];

    // Node
    lookups.push(home_dir.join(".nvm/versions/node"));
    lookups.push(home_dir.join("Library/pnpm"));
    lookups.push(home_dir.join(".config/yarn"));

    lookups
}

#[cfg(target_os = "windows")]
fn get_global_lookups(home_dir: &Path) -> Vec<PathBuf> {
    let mut lookups = vec![];

    // Node
    lookups.push(home_dir.join("AppData\\npm"));
    lookups.push(home_dir.join("AppData\\Roaming\\npm"));
    lookups.push(home_dir.join("AppData\\Local\\pnpm"));
    lookups.push(home_dir.join("AppData\\Yarn\\config"));

    lookups
}

/// Check whether this binary has been installed globally or not.
/// If we encounter an error, simply abort early instead of failing.
fn is_globally_installed() -> bool {
    let exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(_) => return false,
    };

    // Global installs happen *outside* of moon's toolchain,
    // so we simply assume they are using their environment.
    let home_dir = path::get_home_dir().unwrap_or_else(|| PathBuf::from("."));
    let lookups = get_global_lookups(&home_dir);

    // If our executable path starts with the global dir,
    // then we must have been installed globally!
    lookups.iter().any(|lookup| exe_path.starts_with(lookup))
}

fn find_workspace_root(dir: &Path) -> Option<PathBuf> {
    let findable = dir.join(CONFIG_DIRNAME);

    if findable.exists() {
        return Some(dir.to_path_buf());
    }

    match dir.parent() {
        Some(parent_dir) => find_workspace_root(parent_dir),
        None => None,
    }
}

async fn run_bin(bin_path: &Path, current_dir: &Path) -> Result<(), std::io::Error> {
    // Remove the binary path from the current args list
    let args = env::args()
        .enumerate()
        .filter(|(i, arg)| {
            if *i == 0 {
                !arg.ends_with(BIN_NAME)
            } else {
                true
            }
        })
        .map(|(_, arg)| arg)
        .collect::<Vec<String>>();

    // Execute the found moon binary with the current filtered args
    Command::new(bin_path)
        .args(args)
        .current_dir(current_dir)
        .spawn()?
        .wait()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    // console_subscriber::init();

    let mut run = true;

    // Detect if we've been installed globally
    if let Ok(current_dir) = env::current_dir() {
        if is_globally_installed() {
            // If so, find the workspace root so we can locate the
            // locally installed `moon` binary in node modules
            if let Some(workspace_root) = find_workspace_root(&current_dir) {
                let moon_bin = workspace_root
                    // .join(NODE.vendor_dir)
                    // .join("@moonrepo")
                    // .join("cli")
                    .join("target/debug")
                    .join(BIN_NAME);

                // The binary exists! So let's run that one to ensure
                // we're running the version pinned in `package.json`,
                // instead of this global one!
                if moon_bin.exists() {
                    run = false;

                    run_bin(&moon_bin, &current_dir)
                        .await
                        .expect("Failed to run moon binary!");
                }
            }
        }
    }

    // Otherwise just run the CLI
    if run {
        run_cli().await
    }
}
