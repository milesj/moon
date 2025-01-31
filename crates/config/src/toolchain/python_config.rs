use schematic::{derive_enum, Config, ConfigEnum};
use serde::Serialize;
use version_spec::UnresolvedVersionSpec;
use warpgate_api::PluginLocator;

#[cfg(feature = "proto")]
use crate::inherit_tool;

derive_enum!(
    /// The available package managers for Python.
    #[derive(ConfigEnum, Copy, Default)]
    pub enum PythonPackageManager {
        #[default]
        Pip,
        Uv,
    }
);

#[derive(Clone, Config, Debug, PartialEq, Serialize)]
pub struct PipConfig {
    /// List of arguments to append to `pip install` commands.
    pub install_args: Vec<String>,
}

#[derive(Clone, Config, Debug, PartialEq, Serialize)]
pub struct UvConfig {
    /// Location of the WASM plugin to use for uv support.
    pub plugin: Option<PluginLocator>,

    /// List of arguments to append to `uv sync` commands.
    pub sync_args: Vec<String>,

    /// The version of uv to download, install, and run `uv` tasks with.
    #[setting(env = "MOON_UV_VERSION")]
    pub version: Option<UnresolvedVersionSpec>,
}

#[derive(Clone, Config, Debug, PartialEq)]
pub struct PythonConfig {
    /// The package manager to use for installing dependencies.
    pub package_manager: PythonPackageManager,

    /// Options for pip, when used as a package manager.
    #[setting(nested)]
    pub pip: PipConfig,

    /// Location of the WASM plugin to use for Python support.
    pub plugin: Option<PluginLocator>,

    /// Assumes only the root `requirements.txt` is used for dependencies.
    /// Can be used to support the "one version policy" pattern.
    pub root_requirements_only: bool,

    /// Options for uv, when used as a package manager.
    #[setting(nested)]
    pub uv: Option<UvConfig>,

    /// Defines the virtual environment name, which will be created in the workspace root.
    /// Project dependencies will be installed into this.
    #[setting(default = ".venv")]
    pub venv_name: String,

    /// The version of Python to download, install, and run `python` tasks with.
    #[setting(env = "MOON_PYTHON_VERSION")]
    pub version: Option<UnresolvedVersionSpec>,
}

#[cfg(feature = "proto")]
impl PythonConfig {
    inherit_tool!(UvConfig, uv, "uv", inherit_proto_uv);

    pub fn inherit_proto(&mut self, proto_config: &proto_core::ProtoConfig) -> miette::Result<()> {
        match &self.package_manager {
            PythonPackageManager::Pip => {
                // Built-in
            }
            PythonPackageManager::Uv => {
                if self.uv.is_none() {
                    self.uv = Some(UvConfig::default());
                }

                self.inherit_proto_uv(proto_config)?;
            }
        }

        Ok(())
    }
}
