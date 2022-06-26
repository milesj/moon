use figment::{Error as FigmentError, Figment};
use serde_json::Value;
use std::borrow::Cow;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

pub fn create_validation_error(code: &'static str, path: &str, message: String) -> ValidationError {
    let mut error = ValidationError::new(code);
    error.message = Some(Cow::from(message));
    // Is there a better way to do this?
    error.add_param(Cow::from("path"), &path.to_owned());
    error
}

pub fn format_figment_errors(errors: Vec<FigmentError>) -> String {
    println!("{:#?}", errors);

    let mut list = vec![];

    for error in errors {
        for nested_error in error {
            list.push(format!("  <accent>▪</accent> {}", nested_error));
        }
    }

    list.join("\n")
}

pub fn map_validation_errors_to_figment_errors(
    figment: &Figment,
    validation_errors: &ValidationErrors,
) -> Vec<FigmentError> {
    let mut errors = vec![];
    let mut nested_errors = vec![];

    let mut push_error = |validation_error: &ValidationError| {
        println!("validation_error = {:#?}", validation_error);

        if validation_error.message.is_none() {
            return;
        }

        let mut figment_error = FigmentError::from(String::from(
            validation_error.message.as_ref().unwrap().clone(),
        ));

        figment_error.profile = Some(figment.profile().clone());

        if let Some(Value::String(path)) = validation_error.params.get("path") {
            if let Some(metadata) = figment.find_metadata(&path) {
                figment_error.metadata = Some(metadata.clone());
            }

            figment_error = figment_error.with_path(&path);
        };

        errors.push(figment_error);
    };

    for error_kind in validation_errors.errors().values() {
        match error_kind {
            ValidationErrorsKind::Struct(error) => {
                nested_errors.extend(map_validation_errors_to_figment_errors(figment, error));
            }
            ValidationErrorsKind::List(error_map) => {
                for error in error_map.values() {
                    nested_errors.extend(map_validation_errors_to_figment_errors(figment, error));
                }
            }
            ValidationErrorsKind::Field(error_list) => {
                for error in error_list {
                    push_error(error);
                }
            }
        }
    }

    errors.extend(nested_errors);
    errors
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use figment::error::Kind;
    use figment::Error;
    use validator::{ValidationErrors, ValidationErrorsKind};

    fn extract_first_error(errors: &ValidationErrors) -> String {
        for val in errors.errors().values() {
            match val {
                ValidationErrorsKind::Struct(obj) => {
                    let result = extract_first_error(obj);

                    if !result.is_empty() {
                        return result;
                    }
                }
                ValidationErrorsKind::List(list) => {
                    if !list.is_empty() {
                        let item = extract_first_error(list.values().next().unwrap());

                        if !item.is_empty() {
                            return item;
                        }
                    }
                }
                ValidationErrorsKind::Field(field) => {
                    if !field.is_empty() {
                        return format_validation_error(&field[0]);
                    }
                }
            }
        }

        String::from("")
    }

    pub fn handled_jailed_error(errors: &ValidationErrors) -> Error {
        Error::from(FigmentKind::Message(extract_first_error(errors)))
    }
}
