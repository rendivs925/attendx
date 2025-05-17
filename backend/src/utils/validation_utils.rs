use actix_web::HttpResponse;
use rayon::prelude::*;
use serde_json::json;
use shared::types::validation_fields::ValidationFields;
use std::borrow::Cow;
use std::collections::HashMap;
use validator::{ValidationError, ValidationErrors};

use crate::{
    types::responses::api_response::{ApiResponse, ErrorDetails},
    utils::locale_utils::Messages,
    validations::{email::validate_email, name::validate_name, password::validate_password},
};

type FieldValidation<'a> = (
    &'static str,
    &'a str,
    fn(&'a str, &Messages) -> Result<(), ValidationError>,
);

pub fn validate_fields(
    fields: Vec<FieldValidation>,
    messages: &Messages,
) -> Result<(), ValidationErrors> {
    let errors = std::sync::Mutex::new(ValidationErrors::new());

    fields.par_iter().for_each(|(field, value, validator)| {
        if let Err(error) = validator(value, messages) {
            let mut errors_lock = errors.lock().unwrap();
            errors_lock.add(field, error);
        }
    });

    let errors = errors.into_inner().unwrap();
    if errors.errors().is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn validate_data(data: &ValidationFields, messages: &Messages) -> Result<(), ValidationErrors> {
    let mut fields: Vec<FieldValidation> = Vec::new();

    if let Some(ref name) = data.name {
        fields.push(("name", name.as_str(), validate_name));
    }
    if let Some(ref email) = data.email {
        fields.push(("email", email.as_str(), validate_email));
    }
    if let Some(ref password) = data.password {
        fields.push(("password", password.as_str(), validate_password));
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

pub fn handle_validation_error(errors: ValidationErrors, msg: &str) -> HttpResponse {
    let error_details = ErrorDetails {
        details: Some(json!(&errors)),
    };
    HttpResponse::BadRequest().json(ApiResponse::<()>::error(msg, Some(error_details)))
}

pub fn handle_internal_error(err: impl ToString) -> HttpResponse {
    HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err.to_string(), None))
}

pub fn add_error(code: &'static str, message: String, field_value: &str) -> ValidationError {
    ValidationError {
        code: code.into(),
        message: Some(Cow::Owned(message)),
        params: {
            let mut params = HashMap::new();
            params.insert("value".into(), json!(field_value));
            params
        },
    }
}
