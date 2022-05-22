use dialoguer::{Confirm, Select};
use moon_config::constants::{CONFIG_DIRNAME, CONFIG_PROJECT_FILENAME, CONFIG_WORKSPACE_FILENAME};
use moon_config::package::{PackageJson, Workspaces};
use moon_config::{
    default_node_version, default_npm_version, default_pnpm_version, default_yarn_version,
    load_global_project_config_template, load_workspace_config_template,
};
use moon_logger::color;
use moon_utils::fs;
use moon_utils::path;
use std::collections::BTreeMap;
use std::env;
use std::fs::{read_to_string, OpenOptions};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use wax::{Glob, Pattern};

type AnyError = Box<dyn std::error::Error>;

/// Verify the destination and return a path to the `.moon` folder
/// if all questions have passed.
fn verify_dest_dir(dest_dir: &Path) -> Result<Option<PathBuf>, AnyError> {
    if Confirm::new()
        .with_prompt(format!("Initialize moon into {}?", color::path(dest_dir)))
        .interact()?
    {
        let moon_dir = dest_dir.join(CONFIG_DIRNAME);

        if moon_dir.exists()
            && !Confirm::new()
                .with_prompt("Moon has already been initialized, overwrite it?")
                .interact()?
        {
            return Ok(None);
        }

        return Ok(Some(moon_dir));
    }

    Ok(None)
}

/// Verify the package manager to use. If a `package.json` exists,
/// and the `packageManager` field is defined, use that.
async fn verify_package_manager(dest_dir: &Path) -> Result<(String, String), AnyError> {
    let pkg_path = dest_dir.join("package.json");
    let mut pm_type = String::new();
    let mut pm_version = String::new();

    // Extract value from `packageManager` field
    if pkg_path.exists() {
        if let Ok(pkg) = PackageJson::load(&pkg_path).await {
            if let Some(pm) = pkg.package_manager {
                let pm = pm.clone();

                if pm.contains('@') {
                    let mut parts = pm.split('@');

                    pm_type = parts.next().unwrap_or_default().to_owned();
                    pm_version = parts.next().unwrap_or_default().to_owned();
                } else {
                    pm_type = pm;
                }
            }
        }
    }

    // If no value, ask for explicit input
    if pm_type.is_empty() {
        let items = vec!["npm", "pnpm", "yarn"];
        let index = Select::new()
            .with_prompt("Package manager?")
            .items(&items)
            .default(0)
            .interact_opt()?
            .unwrap_or(0);

        pm_type = String::from(items[index]);
    }

    // If no version, fallback to configuration default
    if pm_version.is_empty() {
        if pm_type == "npm" {
            pm_version = default_npm_version();
        } else if pm_type == "pnpm" {
            pm_version = default_pnpm_version();
        } else if pm_type == "yarn" {
            pm_version = default_yarn_version();
        }
    }

    Ok((pm_type, pm_version))
}

/// Detect the Node.js version from local configuration files,
/// otherwise fallback the configuration default.
fn detect_node_version(dest_dir: &Path) -> Result<String, AnyError> {
    let nvmrc_path = dest_dir.join(".nvmrc");

    if nvmrc_path.exists() {
        return Ok(read_to_string(nvmrc_path)?.trim().to_owned());
    }

    let node_version_path = dest_dir.join(".node-version");

    if node_version_path.exists() {
        return Ok(read_to_string(node_version_path)?.trim().to_owned());
    }

    Ok(default_node_version())
}

/// Infer a project name from a source path, by using the name of
/// the project folder.
fn infer_project_name_from_source(source: &str) -> String {
    let source = path::standardize_separators(source);

    if source.contains('/') {
        source.split('/').last().unwrap().to_owned()
    } else {
        source
    }
}

/// For each pattern in the workspaces list, glob the file system
/// for potential projects, and infer their name and source.
fn inherit_projects_from_workspaces(
    dest_dir: &Path,
    workspaces: Vec<String>,
    projects: &mut BTreeMap<String, String>,
) -> Result<(), AnyError> {
    for pattern in workspaces {
        if path::is_glob(&pattern) {
            let glob = Glob::new(&pattern).unwrap();

            for entry in glob.walk(dest_dir, usize::MAX) {
                let entry = entry?;

                if entry.file_type().is_dir() {
                    let source = entry
                        .path()
                        .strip_prefix(dest_dir)
                        .unwrap()
                        .to_string_lossy();

                    projects.insert(
                        infer_project_name_from_source(&source),
                        String::from(source),
                    );
                }
            }
        } else {
            projects.insert(infer_project_name_from_source(&pattern), pattern.to_owned());
        }
    }

    Ok(())
}

/// Detect potential projects (for existing repos only) by
/// inspecting the `workspaces` field in a root `package.json`.
async fn detect_projects(dest_dir: &Path) -> Result<BTreeMap<String, String>, AnyError> {
    let pkg_path = dest_dir.join("package.json");
    let mut projects = BTreeMap::new();

    if pkg_path.exists() {
        if let Ok(pkg) = PackageJson::load(&pkg_path).await {
            if let Some(workspaces) = pkg.workspaces {
                if Confirm::new()
                    .with_prompt("Inherit projects from package.json workspaces?")
                    .interact()?
                {
                    let packages = match workspaces {
                        Workspaces::Array(list) => list,
                        Workspaces::Object(object) => object.packages.unwrap_or_default(),
                    };

                    inherit_projects_from_workspaces(dest_dir, packages, &mut projects)?;
                }
            }
        }
    }

    if projects.is_empty() {
        projects.insert("example".to_owned(), "apps/example".to_owned());
    }

    Ok(projects)
}

pub async fn init(dest: &str, force: bool) -> Result<(), AnyError> {
    let working_dir = env::current_dir().unwrap();
    let dest_path = PathBuf::from(dest);
    let dest_dir = if dest == "." {
        working_dir
    } else if dest_path.is_absolute() {
        dest_path
    } else {
        working_dir.join(dest)
    };

    // Extract template variables
    let dest_dir = path::normalize(&dest_dir);
    let moon_dir = match verify_dest_dir(&dest_dir)? {
        Some(dir) => dir,
        None => return Ok(()),
    };
    let package_manager = verify_package_manager(&dest_dir).await?;
    let node_version = detect_node_version(&dest_dir)?;
    let projects = detect_projects(&dest_dir).await?;

    println!("moon_dir={:#?}", moon_dir);
    println!("package_manager={:#?}", package_manager);
    println!("node_version={:#?}", node_version);
    println!("projects={:#?}", projects);

    // Create config files
    //     fs::create_dir_all(&moon_dir).await?;

    //     fs::write(
    //         &moon_dir.join(CONFIG_WORKSPACE_FILENAME),
    //         load_workspace_config_template(),
    //     )
    //     .await?;

    //     fs::write(
    //         &moon_dir.join(CONFIG_PROJECT_FILENAME),
    //         load_global_project_config_template(),
    //     )
    //     .await?;

    //     // Append to ignore file
    //     let mut file = OpenOptions::new()
    //         .create(true)
    //         .append(true)
    //         .open(dest_dir.join(".gitignore"))?;

    //     writeln!(
    //         file,
    //         r#"
    // # Moon
    // .moon/cache"#
    //     )?;

    println!(
        "Moon has successfully been initialized in {}",
        color::path(&dest_dir.canonicalize().unwrap()),
    );

    Ok(())
}
