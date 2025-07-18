use crate::prelude::*;
use email_address::EmailAddress;
use rayon::prelude::*;
use validator::ValidationError;

use crate::utils::{
    locale_utils::Namespace,
    validation_utils::{add_error, format_error_message},
};

const MIN_EMAIL_LENGTH: usize = 5;
const MAX_EMAIL_LENGTH: usize = 254;
const MIN_DOMAIN_SEGMENT_LENGTH: usize = 2;
const MIN_TLD_LENGTH: usize = 2;

fn has_min_length(email: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    let length = email.len();
    if length < MIN_EMAIL_LENGTH {
        return Err(messages.get_message(Namespace::Validation, "email.too_short"));
    }
    Ok(())
}

fn has_max_length(email: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    let length = email.len();
    if length > MAX_EMAIL_LENGTH {
        return Err(messages.get_message(Namespace::Validation, "email.too_long"));
    }
    Ok(())
}

fn has_at_and_dot(email: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    let has_at = email.contains('@');
    let has_dot = email.contains('.');
    if !has_at || !has_dot {
        let msg_at = messages.get_message(Namespace::Validation, "email.missing_at");
        let msg_dot = messages.get_message(Namespace::Validation, "email.missing_dot");
        let msg = if !msg_at.is_empty() {
            msg_at
        } else if !msg_dot.is_empty() {
            msg_dot
        } else {
            "Email must contain '@' and '.'".to_string()
        };
        Err(msg)
    } else {
        Ok(())
    }
}

fn is_at_before_dot(email: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    if let (Some(at_index), Some(dot_index)) = (email.find('@'), email.rfind('.')) {
        if at_index >= dot_index {
            Err(messages.get_message(Namespace::Validation, "email.at_before_dot"))
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

fn has_no_invalid_chars(email: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    let has_invalid = email.chars().any(|c| c == ' ' || !c.is_ascii());
    if has_invalid {
        Err(messages.get_message(Namespace::Validation, "email.invalid_chars"))
    } else {
        Ok(())
    }
}

fn has_no_consecutive_dots(email: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    let has_consecutive = email.contains("..");
    if has_consecutive {
        Err(messages.get_message(Namespace::Validation, "email.consecutive_dots"))
    } else {
        Ok(())
    }
}

fn has_no_leading_or_trailing_dot(email: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    let starts_with_dot = email.starts_with('.');
    let ends_with_dot = email.ends_with('.');
    if starts_with_dot || ends_with_dot {
        Err(messages.get_message(Namespace::Validation, "email.starts_or_ends_with_dot"))
    } else {
        Ok(())
    }
}

fn domain_starts_without_dot(email: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    if let Some(domain) = get_domain(email) {
        if domain.starts_with('.') {
            return Err(messages.get_message(Namespace::Validation, "email.domain_starts_with_dot"));
        }
    }
    Ok(())
}

fn domain_exists(email: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    if get_domain(email).is_none() {
        Err(messages.get_message(Namespace::Validation, "email.missing_domain"))
    } else {
        Ok(())
    }
}

fn is_structure_valid_domain(email: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    if let Some(domain) = get_domain(email) {
        let has_dot = domain.contains('.');
        let has_space = domain.contains(' ');
        let is_empty = domain.is_empty();
        if !has_dot || has_space || is_empty {
            return Err(messages.get_message(Namespace::Validation, "email.invalid_domain"));
        }
    }
    Ok(())
}

fn has_valid_domain_segment_length(
    email: &str,
    messages: &dyn MessageLookup,
) -> Result<(), String> {
    if let Some(domain) = get_domain(email) {
        if let Some(first_dot_index) = domain.find('.') {
            if first_dot_index < MIN_DOMAIN_SEGMENT_LENGTH {
                return Err(
                    messages.get_message(Namespace::Validation, "email.invalid_domain_length")
                );
            }
        }
    }
    Ok(())
}

fn has_valid_tld_format(email: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    if let Some(domain) = get_domain(email) {
        if let Some(last_dot_index) = domain.rfind('.') {
            let tld = &domain[last_dot_index + 1..];
            let tld_length = tld.len();
            let all_alphabetic = tld.chars().all(|c| c.is_alphabetic());
            if tld_length < MIN_TLD_LENGTH || !all_alphabetic {
                return Err(messages.get_message(Namespace::Validation, "email.invalid_tld"));
            }
        }
    }
    Ok(())
}

fn is_overall_format_valid(email: &str, messages: &dyn MessageLookup) -> Result<(), String> {
    if !EmailAddress::is_valid(email) {
        Err(messages.get_message(Namespace::Validation, "email.invalid"))
    } else {
        Ok(())
    }
}

fn get_domain(email: &str) -> Option<&str> {
    email.split('@').nth(1)
}

pub fn validate_email(messages: &dyn MessageLookup, email: &str) -> Result<(), ValidationError> {
    let validations = vec![
        has_min_length,
        has_max_length,
        has_at_and_dot,
        is_at_before_dot,
        has_no_invalid_chars,
        has_no_consecutive_dots,
        has_no_leading_or_trailing_dot,
        domain_starts_without_dot,
        domain_exists,
        is_structure_valid_domain,
        has_valid_domain_segment_length,
        has_valid_tld_format,
    ];

    let mut errors: Vec<String> = validations
        .par_iter()
        .filter_map(|validate| validate(email, messages).err())
        .collect();

    if errors.is_empty() {
        if let Err(msg) = is_overall_format_valid(email, messages) {
            errors.push(msg);
        }
    }

    if !errors.is_empty() {
        let raw_errors = errors.join(", ");
        let formatted_error_message = format_error_message(&raw_errors);

        return Err(add_error("email.invalid", formatted_error_message, email));
    }

    Ok(())
}
