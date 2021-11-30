// <project path>/project.yml

use crate::constants;
use crate::errors::{create_validation_error, map_figment_error_to_validation_errors};
use figment::value::{Dict, Map};
use figment::{
    providers::{Format, Yaml},
    Figment, Metadata, Profile, Provider,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use validator::{Validate, ValidationError, ValidationErrors};

fn validate_channel(value: &str) -> Result<(), ValidationError> {
    if !value.is_empty() && !value.starts_with('#') {
        return Err(create_validation_error(
            "invalid_channel",
            "project.channel",
            String::from("Must start with a #."),
        ));
    }

    Ok(())
}

pub struct FileGroups(HashMap<String, Vec<String>>);

#[derive(Debug, Deserialize, PartialEq, Serialize, Validate)]
pub struct ProjectMetadataConfig {
    name: String,

    description: String,

    owner: String,

    maintainers: Vec<String>,

    #[validate(custom = "validate_channel")]
    channel: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Validate)]
pub struct ProjectConfig {
    #[serde(rename = "dependsOn")]
    depends_on: Option<Vec<String>>,

    #[serde(rename = "fileGroups")]
    file_groups: Option<HashMap<String, Vec<String>>>,

    #[validate]
    project: ProjectMetadataConfig,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        ProjectConfig {
            depends_on: None,
            file_groups: None,
            project: ProjectMetadataConfig {
                name: String::from(""),
                description: String::from(""),
                owner: String::from(""),
                maintainers: vec![String::from("")],
                channel: String::from(""),
            },
        }
    }
}

impl Provider for ProjectConfig {
    fn metadata(&self) -> Metadata {
        Metadata::named(constants::CONFIG_PROJECT_FILENAME)
    }

    fn data(&self) -> Result<Map<Profile, Dict>, figment::Error> {
        figment::providers::Serialized::defaults(ProjectConfig::default()).data()
    }

    fn profile(&self) -> Option<Profile> {
        Some(Profile::Default)
    }
}

impl ProjectConfig {
    pub fn load(path: PathBuf) -> Result<ProjectConfig, ValidationErrors> {
        // Load and parse the yaml config file using Figment and handle accordingly.
        // Unfortunately this does some "validation", so instead of having 2 validation paths,
        // let's remap to a `validator` error type, so that downstream can handle easily.
        let config: ProjectConfig = match Figment::new().merge(Yaml::file(path)).extract() {
            Ok(cfg) => cfg,
            Err(error) => return Err(map_figment_error_to_validation_errors(&error)),
        };

        // Validate the fields before continuing
        if let Err(errors) = config.validate() {
            return Err(errors);
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::tests::handled_jailed_error;
    use figment;

    fn load_jailed_config() -> Result<ProjectConfig, figment::Error> {
        match ProjectConfig::load(PathBuf::from(constants::CONFIG_PROJECT_FILENAME)) {
            Ok(cfg) => {
                return Ok(cfg);
            }
            Err(errors) => {
                return Err(handled_jailed_error(&errors));
            }
        }
    }

    #[test]
    #[should_panic(expected = "Missing field `project`.")]
    fn empty_file() {
        figment::Jail::expect_with(|jail| {
            // Needs a fake yaml value, otherwise the file reading panics
            jail.create_file(constants::CONFIG_PROJECT_FILENAME, "fake: value")?;

            load_jailed_config()?;

            Ok(())
        });
    }

    mod depends_on {
        #[test]
        #[should_panic(
            expected = "Invalid field `dependsOn`. Expected a sequence type, received unsigned int `123`."
        )]
        fn invalid_type() {
            figment::Jail::expect_with(|jail| {
                jail.create_file(
                    super::constants::CONFIG_PROJECT_FILENAME,
                    r#"
dependsOn: 123
project:
    name: ''
    description: ''
    owner: ''
    maintainers: []
    channel: ''"#,
                )?;

                super::load_jailed_config()?;

                Ok(())
            });
        }
    }

    mod file_groups {
        #[test]
        #[should_panic(
            expected = "Invalid field `fileGroups`. Expected a map type, received unsigned int `123`."
        )]
        fn invalid_type() {
            figment::Jail::expect_with(|jail| {
                jail.create_file(
                    super::constants::CONFIG_PROJECT_FILENAME,
                    r#"
fileGroups: 123
project:
    name: ''
    description: ''
    owner: ''
    maintainers: []
    channel: ''"#,
                )?;

                super::load_jailed_config()?;

                Ok(())
            });
        }
    }

    mod project {
        #[test]
        #[should_panic(
            expected = "Invalid field `project`. Expected struct ProjectMetadataConfig type, received unsigned int `123`."
        )]
        fn invalid_type() {
            figment::Jail::expect_with(|jail| {
                jail.create_file(super::constants::CONFIG_PROJECT_FILENAME, "project: 123")?;

                super::load_jailed_config()?;

                Ok(())
            });
        }

        #[test]
        #[should_panic(
            expected = "Invalid field `project.name`. Expected a string type, received unsigned int `123`."
        )]
        fn invalid_name_type() {
            figment::Jail::expect_with(|jail| {
                jail.create_file(
                    super::constants::CONFIG_PROJECT_FILENAME,
                    r#"
project:
    name: 123
    description: ''
    owner: ''
    maintainers: []
    channel: ''"#,
                )?;

                super::load_jailed_config()?;

                Ok(())
            });
        }

        #[test]
        #[should_panic(
            expected = "Invalid field `project.description`. Expected a string type, received bool true."
        )]
        fn invalid_description_type() {
            figment::Jail::expect_with(|jail| {
                jail.create_file(
                    super::constants::CONFIG_PROJECT_FILENAME,
                    r#"
project:
    name: ''
    description: true
    owner: ''
    maintainers: []
    channel: ''"#,
                )?;

                super::load_jailed_config()?;

                Ok(())
            });
        }

        #[test]
        #[should_panic(
            expected = "Invalid field `project.owner`. Expected a string type, received map."
        )]
        fn invalid_owner_type() {
            figment::Jail::expect_with(|jail| {
                jail.create_file(
                    super::constants::CONFIG_PROJECT_FILENAME,
                    r#"
project:
    name: ''
    description: ''
    owner: {}
    maintainers: []
    channel: ''"#,
                )?;

                super::load_jailed_config()?;

                Ok(())
            });
        }

        #[test]
        #[should_panic(
            expected = "Invalid field `project.maintainers`. Expected a sequence type, received string \"abc\"."
        )]
        fn invalid_maintainers_type() {
            figment::Jail::expect_with(|jail| {
                jail.create_file(
                    super::constants::CONFIG_PROJECT_FILENAME,
                    r#"
project:
    name: ''
    description: ''
    owner: ''
    maintainers: 'abc'
    channel: ''"#,
                )?;

                super::load_jailed_config()?;

                Ok(())
            });
        }

        #[test]
        #[should_panic(
            expected = "Invalid field `project.channel`. Expected a string type, received unsigned int `123`."
        )]
        fn invalid_channel_type() {
            figment::Jail::expect_with(|jail| {
                jail.create_file(
                    super::constants::CONFIG_PROJECT_FILENAME,
                    r#"
project:
    name: ''
    description: ''
    owner: ''
    maintainers: []
    channel: 123"#,
                )?;

                super::load_jailed_config()?;

                Ok(())
            });
        }

        #[test]
        #[should_panic(expected = "Invalid field `project.channel`. Must start with a #.")]
        fn channel_leading_hash() {
            figment::Jail::expect_with(|jail| {
                jail.create_file(
                    super::constants::CONFIG_PROJECT_FILENAME,
                    r#"
project:
    name: ''
    description: ''
    owner: ''
    maintainers: []
    channel: name"#,
                )?;

                super::load_jailed_config()?;

                Ok(())
            });
        }
    }
}
