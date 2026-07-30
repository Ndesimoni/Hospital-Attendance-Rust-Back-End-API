use validator::ValidationError;

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if !password.chars().any(|c| c.is_uppercase()) {
        let mut error = ValidationError::new("password_no_uppercase");
        error.message = Some("Password must contain an uppercase letter".into());
        return Err(error);
    };

    if !password.chars().any(|c| c.is_lowercase()) {
        let mut error = ValidationError::new("password_no_lowercase");
        error.message = Some("Password must contain an lowercase letter".into());
        return Err(error);
    };

    if !password.chars().any(|c| c.is_ascii_digit()) {
        let mut error = ValidationError::new("password_no_numbers");
        error.message = Some("Password must contain a number".into());
        return Err(error);
    };

    if !password.chars().any(|c| c.is_alphanumeric()) {
        let mut error = ValidationError::new("password_no_character");
        error.message = Some("Password must contain a special character".into());
        return Err(error);
    };

    Ok(())
}
