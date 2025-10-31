use std::borrow::Cow;

use validator::{ValidateEmail, ValidateLength, ValidationError};

pub fn name_validation(value: &str) -> Result<(), ValidationError> {
    if !value.validate_length(Some(3), Some(255), None) {
        let validation = ValidationError::new("size");
        let validation = validation.with_message(Cow::from("The 'name' field must be between 3 and 255 characters long."));
        return Err(validation);
    }
    
    Ok(())
}

pub fn email_validation(value: &str) -> Result<(), ValidationError> {
    if !value.validate_length(Some(6), Some(255), None) {
        let validation = ValidationError::new("size");
        let validation = validation.with_message(Cow::from("The 'email' field must be between 3 and 255 characters long."));
        return Err(validation);
    }
    if !value.validate_email() {
        let validation = ValidationError::new("email format");
        let validation = validation.with_message(Cow::from("The 'email' format is invalid"));
        return Err(validation);
    }
    
    Ok(())
}

pub fn phone_validation(value: &str) -> Result<(), ValidationError> {
    if !value.validate_length(None, None, Some(13)) {
        let validation = ValidationError::new("size");
        let validation = validation.with_message(Cow::from("The 'phone' field must be equal to 13 characters long."));
        return Err(validation);
    }
    
    Ok(())
}

pub fn cpf_validation(value: &str) -> Result<(), ValidationError> {
    if !value.validate_length(None, None, Some(11)) {
        let validation = ValidationError::new("size");
        let validation = validation.with_message(Cow::from("The 'cpf' field must be equal to 11 characters long without special characters."));
        return Err(validation);
    }
    
    Ok(())
}

pub fn password_validation(value: &str) -> Result<(), ValidationError> {
    if !value.validate_length(Some(8), Some(255), None) {
        let validation = ValidationError::new("size");
        let validation = validation.with_message(Cow::from("The 'password' field must be between 8 and 255 characters long."));
        return Err(validation);
    }
    
    Ok(())
}

pub fn cep_validation(value: &str) -> Result<(), ValidationError> {
    if !value.validate_length(None, None, Some(8)) {
        let validation = ValidationError::new("size");
        let validation = validation.with_message(Cow::from("The 'cep' field must be equal to 8 characters long without special characters."));
        return Err(validation);
    }
    
    Ok(())
}

pub fn number_validation(value: &str) -> Result<(), ValidationError> {
    if !value.validate_length(Some(1), Some(6), None) {
        let validation = ValidationError::new("size");
        let validation = validation.with_message(Cow::from("The 'number' field must be between 1 and 6 digits long."));
        return Err(validation);
    }
    
    Ok(())
}