use validator::ValidationError;

pub fn validate_email(_email: &str, _: &str) -> Result<(), ValidationError> {
    // Validator custom function signature: fn(&str, &str) -> Result<(), ValidationError>
    // First param is the value, second is the field name
    let email = _email;
    if email.contains('@') && email.len() > 3 && email.len() < 255 {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_email"))
    }
}

pub fn validate_password(_password: &str, _: &str) -> Result<(), ValidationError> {
    let password = _password;
    if password.len() < 8 {
        return Err(ValidationError::new("password_too_short"));
    }
    if password.len() > 128 {
        return Err(ValidationError::new("password_too_long"));
    }
    // At least one number and one letter
    let has_number = password.chars().any(|c| c.is_ascii_digit());
    let has_letter = password.chars().any(|c| c.is_ascii_alphabetic());
    
    if !has_number || !has_letter {
        return Err(ValidationError::new("password_weak"));
    }
    
    Ok(())
}

pub fn validate_phone(_phone: &str, _: &str) -> Result<(), ValidationError> {
    let phone = _phone;
    // Basic phone validation - starts with + and has digits
    if phone.starts_with('+') && phone.len() > 7 && phone.len() < 20 {
        let digits = phone.chars().skip(1).all(|c| c.is_ascii_digit() || c == ' ' || c == '-');
        if digits {
            return Ok(());
        }
    }
    Err(ValidationError::new("invalid_phone"))
}
