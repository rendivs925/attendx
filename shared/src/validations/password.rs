use rayon::prelude::*;
use validator::ValidationError;

use crate::utils::{
    locale_utils::Messages,
    validation_utils::{add_error, format_error_message},
};

const MIN_PASSWORD_LENGTH: usize = 8;
const MAX_PASSWORD_LENGTH: usize = 128;

fn has_min_length(password: &str, messages: &Messages) -> Result<(), String> {
    let length = password.len();
    if length < MIN_PASSWORD_LENGTH {
        return Err(messages.get_validation_message("password.too_short"));
    }
    Ok(())
}

fn has_max_length(password: &str, messages: &Messages) -> Result<(), String> {
    let length = password.len();
    if length > MAX_PASSWORD_LENGTH {
        return Err(messages.get_validation_message("password.too_long"));
    }
    Ok(())
}

fn has_no_space(password: &str, messages: &Messages) -> Result<(), String> {
    if password.contains(' ') {
        return Err(messages.get_validation_message("password.contains_space"));
    }
    Ok(())
}

fn has_uppercase(password: &str, messages: &Messages) -> Result<(), String> {
    if !password.chars().any(|char| char.is_ascii_uppercase()) {
        return Err(messages.get_validation_message("password.missing_uppercase"));
    }
    Ok(())
}

fn has_lowercase(password: &str, messages: &Messages) -> Result<(), String> {
    if !password.chars().any(|char| char.is_ascii_lowercase()) {
        return Err(messages.get_validation_message("password.missing_lowercase"));
    }
    Ok(())
}

fn has_digit(password: &str, messages: &Messages) -> Result<(), String> {
    if !password.chars().any(|char| char.is_ascii_digit()) {
        return Err(messages.get_validation_message("password.missing_digit"));
    }
    Ok(())
}

fn has_special_char(password: &str, messages: &Messages) -> Result<(), String> {
    if !password.chars().any(|char| !char.is_alphanumeric()) {
        return Err(messages.get_validation_message("password.missing_special_char"));
    }
    Ok(())
}

pub fn validate_password(messages: &Messages, password: &str) -> Result<(), ValidationError> {
    let validations = vec![
        has_min_length,
        has_max_length,
        has_no_space,
        has_uppercase,
        has_lowercase,
        has_digit,
        has_special_char,
    ];

    let errors: Vec<String> = validations
        .par_iter()
        .filter_map(|validate_fn| validate_fn(password, messages).err())
        .collect();

    if errors.is_empty() {
        Ok(())
    } else {
        let raw_errors = errors.join(", ");
        let formatted_error_message = format_error_message(&raw_errors);

        Err(add_error(
            "password.invalid",
            formatted_error_message,
            password,
        ))
    }
}
