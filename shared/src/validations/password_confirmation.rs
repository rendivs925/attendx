use crate::prelude::*;
use crate::utils::{
    locale_utils::Namespace,
    validation_utils::{add_error, format_error_message},
};
use validator::ValidationError;

pub fn validate_password_confirmation(
    messages: &dyn MessageLookup,
    password: &str,
    password_confirmation: Option<&str>,
) -> Result<(), ValidationError> {
    let Some(confirm) = password_confirmation else {
        let error_message =
            messages.get_message(Namespace::Validation, "password_confirmation.required");
        return Err(add_error(
            "password_confirmation.required",
            error_message,
            "",
        ));
    };

    if password != confirm {
        let error_message =
            messages.get_message(Namespace::Validation, "password_confirmation.mismatch");
        let formatted = format_error_message(&error_message);
        return Err(add_error(
            "password_confirmation.mismatch",
            formatted,
            confirm,
        ));
    }

    Ok(())
}
