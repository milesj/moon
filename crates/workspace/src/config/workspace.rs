// .monolith/workspace.yml

use crate::constants;
use figment::value::{Dict, Map};
use figment::{
    providers::{Format, Yaml},
    Error, Figment, Metadata, Profile, Provider,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum PackageManager {
    npm,
    pnpm,
    yarn,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeConfigShasums {
    pub linux: Option<Vec<String>>,
    pub macos: Option<Vec<String>>,
    pub windows: Option<Vec<String>>,
}

impl Default for NodeConfigShasums {
    fn default() -> Self {
        // https://nodejs.org/dist/v16.13.0/SHASUMS256.txt.asc
        NodeConfigShasums {
            linux: Some(vec![
                // linux-arm64
                String::from("46e3857f5552abd36d9548380d795b043a3ceec2504e69fe1a754fa76012daaf"),
                // linux-x64
                String::from("589b7e7eb22f8358797a2c14a0bd865459d0b44458b8f05d2721294dacc7f734"),
            ]),
            macos: Some(vec![
                // darwin-arm64
                String::from("46d83fc0bd971db5050ef1b15afc44a6665dee40bd6c1cbaec23e1b40fa49e6d"),
                // darwin-x64
                String::from("37e09a8cf2352f340d1204c6154058d81362fef4ec488b0197b2ce36b3f0367a"),
            ]),
            windows: Some(vec![
                // x64
                String::from("bf55b68293b163423ea4856c1d330be23158e78aea18a8756cfdff6fb6ffcd88"),
            ]),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeConfig {
    pub version: String,
    pub package_manager: Option<PackageManager>,
    pub shasums: NodeConfigShasums,
}

impl Default for NodeConfig {
    fn default() -> Self {
        NodeConfig {
            version: String::from("16.13.0"),
            package_manager: Some(PackageManager::npm),
            shasums: NodeConfigShasums::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageManagerConfig {
    pub version: String,
}

impl Default for PackageManagerConfig {
    fn default() -> Self {
        PackageManagerConfig {
            version: String::from("unknown"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceConfig {
    pub node: NodeConfig,
    pub packages: Vec<String>,
    // Package managers
    pub npm: Option<PackageManagerConfig>,
    pub pnpm: Option<PackageManagerConfig>,
    pub yarn: Option<PackageManagerConfig>,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        WorkspaceConfig {
            node: NodeConfig::default(),
            packages: vec![],
            npm: None,
            pnpm: None,
            yarn: None,
        }
    }
}

impl Provider for WorkspaceConfig {
    fn metadata(&self) -> Metadata {
        Metadata::named(constants::CONFIG_WORKSPACE_FILENAME)
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        figment::providers::Serialized::defaults(WorkspaceConfig::default()).data()
    }

    fn profile(&self) -> Option<Profile> {
        Some(Profile::Default)
    }
}

impl WorkspaceConfig {
    pub fn load(path: PathBuf) -> Result<WorkspaceConfig, Error> {
        let config: WorkspaceConfig = Figment::new().merge(Yaml::file(path)).extract()?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use figment;

    #[test]
    fn empty_file() {
        //     figment::Jail::expect_with(|jail| {
        //         jail.create_file(
        //             constants::CONFIG_WORKSPACE_FILENAME,
        //             r#"
        //     name = "Just a TOML App!"
        //     count = 100
        // "#,
        //         )?;
        //     })
    }
}
