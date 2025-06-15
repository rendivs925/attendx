use rayon::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;
use validator::{ValidationError, ValidationErrors};

use crate::{
    prelude::MessageLookup,
    types::requests::auth::validation_request::ValidationRequest,
    validations::{
        email::validate_email, name::validate_name, password::validate_password,
        password_confirmation::validate_password_confirmation,
    },
};

pub type ValidatorFn = Box<
    dyn Fn(&dyn MessageLookup, &str, Option<&str>) -> Result<(), ValidationError> + Send + Sync,
>;

pub struct FieldValidation<'a> {
    pub field_name: &'static str,
    pub value: &'a str,
    pub value2: Option<&'a str>,
    pub validator: ValidatorFn,
}

fn into_validator<F>(f: F) -> ValidatorFn
where
    F: Fn(&dyn MessageLookup, &str) -> Result<(), ValidationError> + Send + Sync + 'static,
{
    Box::new(move |messages, value, _| f(messages, value))
}

pub fn validate_fields(
    fields: Vec<FieldValidation>,
    messages: &dyn MessageLookup,
) -> Result<(), ValidationErrors> {
    let errors = std::sync::Mutex::new(ValidationErrors::new());

    fields.par_iter().for_each(|f| {
        if let Err(error) = (f.validator)(messages, f.value, f.value2) {
            errors.lock().unwrap().add(f.field_name, error);
        }
    });

    let errors = errors.into_inner().unwrap();
    if errors.errors().is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn format_error_message(msg: &str) -> String {
    let parts: Vec<&str> = msg.split(',').collect();
    if parts.is_empty() {
        return String::new();
    }

    let first = parts[0].trim();
    let first_prefix = first
        .split_whitespace()
        .next()
        .filter(|s| s.len() >= 2)
        .unwrap_or("");

    let rest = parts
        .iter()
        .skip(1)
        .map(|part| {
            let trimmed = part.trim();
            if trimmed.starts_with(first_prefix) {
                trimmed[first_prefix.len()..].trim_start()
            } else {
                trimmed
            }
        })
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                Some(c) => format!("{}{}", c.to_lowercase(), chars.collect::<String>()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(", ");

    if rest.is_empty() {
        first.to_string()
    } else {
        format!("{}, {}", first, rest)
    }
}

pub fn validate_data(
    data: &ValidationRequest,
    messages: &dyn MessageLookup,
) -> Result<(), ValidationErrors> {
    let mut fields = Vec::new();

    if let Some(name) = data.name.as_deref() {
        fields.push(FieldValidation {
            field_name: "name",
            value: name,
            value2: None,
            validator: into_validator(validate_name),
        });
    }

    if let Some(email) = data.email.as_deref() {
        fields.push(FieldValidation {
            field_name: "email",
            value: email,
            value2: None,
            validator: into_validator(validate_email),
        });
    }

    if let Some(password) = data.password.as_deref() {
        fields.push(FieldValidation {
            field_name: "password",
            value: password,
            value2: None,
            validator: into_validator(validate_password),
        });
    }

    if let (Some(password), Some(confirm)) = (
        data.password.as_deref(),
        data.password_confirmation.as_deref(),
    ) {
        fields.push(FieldValidation {
            field_name: "password_confirmation",
            value: password,
            value2: Some(confirm),
            validator: Box::new(validate_password_confirmation),
        });
    }

    if fields.is_empty() {
        let mut errors = ValidationErrors::new();
        errors.add(
            "fields",
            ValidationError {
                code: "required".into(),
                message: Some("At least one field is required.".into()),
                params: Default::default(),
            },
        );
        return Err(errors);
    }

    validate_fields(fields, messages)
}

pub fn add_error(code: &'static str, message: String, field_value: &str) -> ValidationError {
    let mut params = HashMap::new();
    params.insert("value".into(), serde_json::json!(field_value));

    ValidationError {
        code: code.into(),
        message: Some(Cow::Owned(message)),
        params,
    }
}
