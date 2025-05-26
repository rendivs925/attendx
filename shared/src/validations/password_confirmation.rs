use crate::utils::{
    locale_utils::Messages,
    validation_utils::{add_error, format_error_message},
};
use validator::ValidationError;

pub fn validate_password_confirmation(
    messages: &Messages,
    password: &str,
    password_confirmation: Option<&str>,
) -> Result<(), ValidationError> {
    let Some(confirm) = password_confirmation else {
        let error_message = messages.get_validation_message(
            "password_confirmation.required",
            "Password confirmation is required",
        );
        return Err(add_error(
            "password_confirmation.missing",
            error_message,
            "",
        ));
    };

    if password != confirm {
        let error_message = messages.get_validation_message(
            "password.mismatch",
            "Password confirmation does not match password",
        );
        let formatted = format_error_message(&error_message);
        return Err(add_error(
            "password_confirmation.invalid",
            formatted,
            confirm,
        ));
    }

    Ok(())
}
