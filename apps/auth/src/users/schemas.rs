use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use common::regex;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(
        must_match(other = "confirm_password", message = "Passwords must match"),
        custom(function = "password_schema", message = "Invalid password format")
    )]
    pub password: String,
    #[validate(
        must_match(other = "password", message = "Passwords must match"),
        custom(function = "password_schema", message = "Invalid password format")
    )]
    pub confirm_password: String,
}

fn password_schema(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new(
            "Password must be at least 8 characters long",
        ));
    }

    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_digit = false;

    for c in password.chars() {
        match c {
            'A'..='Z' => has_uppercase = true,
            'a'..='z' => has_lowercase = true,
            '0'..='9' => has_digit = true,
            _ => {}
        }
    }

    if !has_uppercase || !has_lowercase || !has_digit {
        return Err(ValidationError::new(
            "Password must contain at least one uppercase letter, one lowercase letter, and one digit",
        ));
    }

    if !regex::RE_SPECIAL_CHAR.is_match(password) {
        return Err(ValidationError::new(
            "Password must not contain special characters @$!%*?&",
        ));
    }

    Ok(())
}
