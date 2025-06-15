use crate::prelude::*;
use rayon::prelude::*;
use validator::ValidationError;

use crate::utils::{
    locale_utils::Namespace,
    validation_utils::{add_error, format_error_message},
};

const MIN_NAME_LENGTH: usize = 2;
const MAX_NAME_LENGTH: usize = 100;

fn is_not_empty(name: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    if name.trim().is_empty() {
        Err(messages.get_message(Namespace::Validation, "name.empty"))
    } else {
        Ok(())
    }
}

fn has_min_length(name: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    if name.len() < MIN_NAME_LENGTH {
        Err(messages.get_message(Namespace::Validation, "name.too_short"))
    } else {
        Ok(())
    }
}

fn has_max_length(name: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    if name.len() > MAX_NAME_LENGTH {
        Err(messages.get_message(Namespace::Validation, "name.too_long"))
    } else {
        Ok(())
    }
}

fn has_valid_chars(name: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    if !name.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
        Err(messages.get_message(Namespace::Validation, "name.invalid_chars"))
    } else {
        Ok(())
    }
}

pub fn validate_name(messages: &dyn MessageLookup, name: &str) -> Result<(), ValidationError> {
    let validations = [
        is_not_empty,
        has_min_length,
        has_max_length,
        has_valid_chars,
    ];

    let errors: Vec<String> = validations
        .par_iter()
        .filter_map(|f| f(name, messages).err())
        .collect();

    if errors.is_empty() {
        Ok(())
    } else {
        let raw_errors = errors.join(", ");
        let formatted_error_message = format_error_message(&raw_errors);

        Err(add_error("name.invalid", formatted_error_message, name))
    }
}
