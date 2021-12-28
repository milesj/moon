use crate::errors::create_validation_error;
use semver::Version;
use std::collections::HashMap;
use std::path::Path;
use validator::{Validate, ValidationError, ValidationErrors};

// Extend validator lib
pub trait VecValidate {
    fn validate(&self) -> Result<(), ValidationErrors>;
}

impl<T: Validate> VecValidate for Vec<T> {
    fn validate(&self) -> Result<(), ValidationErrors> {
        for i in self.iter() {
            i.validate()?
        }
        Ok(())
    }
}

pub trait HashMapValidate {
    fn validate(&self) -> Result<(), ValidationErrors>;
}

impl<T: Validate> HashMapValidate for HashMap<String, T> {
    fn validate(&self) -> Result<(), ValidationErrors> {
        for (_, value) in self.iter() {
            value.validate()?
        }
        Ok(())
    }
}

// Validate the value is a valid semver version/range.
pub fn validate_semver_version(key: &str, value: &str) -> Result<(), ValidationError> {
    if Version::parse(value).is_err() {
        return Err(create_validation_error(
            "invalid_semver",
            key,
            String::from("Must be a valid semantic version."),
        ));
    }

    Ok(())
}

// Validate the value is a valid child relative file system path.
// Will fail on absolute paths ("/"), and parent relative paths ("../").
pub fn validate_child_relative_path(key: &str, value: &str) -> Result<(), ValidationError> {
    let path = Path::new(value);

    if path.has_root() || path.is_absolute() {
        return Err(create_validation_error(
            "no_absolute",
            key,
            String::from("Absolute paths are not supported."),
        ));
    } else if path.starts_with("..") {
        return Err(create_validation_error(
            "no_parent_relative",
            key,
            String::from("Parent relative paths are not supported."),
        ));
    }

    Ok(())
}

// Validate the value is a valid child relative file system path or root path.
// Will fail on parent relative paths ("../") and absolute paths.
pub fn validate_child_or_root_path(key: &str, value: &str) -> Result<(), ValidationError> {
    let path = Path::new(value);

    if (path.has_root() || path.is_absolute()) && !path.starts_with("/") {
        return Err(create_validation_error(
            "no_absolute",
            key,
            String::from("Absolute paths are not supported. Root paths must start with `/`."),
        ));
    } else if path.starts_with("..") {
        return Err(create_validation_error(
            "no_parent_relative",
            key,
            String::from("Parent relative paths are not supported."),
        ));
    }

    Ok(())
}

// Validate the value is a target in the format of "project_id:task_name".
pub fn validate_target(key: &str, target: &str) -> Result<(), ValidationError> {
    // TODO regex
    if !target.contains(":") {
        return Err(create_validation_error(
            "invalid_target",
            key,
            String::from("Must be a valid target (project_id:task_name)."),
        ));
    }

    Ok(())
}
