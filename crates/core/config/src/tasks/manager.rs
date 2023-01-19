use crate::{InheritedTasksConfig, PlatformType, ProjectLanguage, ProjectType};
use rustc_hash::{FxHashMap, FxHashSet};
use std::path::Path;

#[derive(Default)]
pub struct InheritedTasksManager {
    configs: FxHashMap<String, InheritedTasksConfig>,
}

impl InheritedTasksManager {
    pub fn add_config(&mut self, path: &Path, config: InheritedTasksConfig) {
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let name = if name == "tasks.yml" {
            "*"
        } else if name.ends_with(".yml") {
            name.strip_suffix(".yml").unwrap()
        } else {
            name.as_ref()
        };

        self.configs.insert(name.to_owned(), config);
    }

    pub fn get_inherited_config(
        &self,
        platform: PlatformType,
        language: ProjectLanguage,
        type_of: ProjectType,
    ) -> InheritedTasksConfig {
        let mut config = InheritedTasksConfig::default();
        let lookups = FxHashSet::from_iter([
            "*".into(),
            format!("{}", platform.to_string()),
            format!("{}", language.to_string()),
            format!("{}-{}", platform.to_string(), type_of.to_string()),
            format!("{}-{}", language.to_string(), type_of.to_string()),
        ]);

        for lookup in lookups {
            if let Some(managed_config) = &self.configs.get(&lookup) {
                config.merge(managed_config);

                if lookup != "*" {
                    config
                        .implicit_inputs
                        .push(format!("/.moon/tasks/{}.yml", lookup));
                }
            }
        }

        config.implicit_inputs.push("/.moon/*.yml".into());
        config
    }
}
