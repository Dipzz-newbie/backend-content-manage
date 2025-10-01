use validator::Validate;
use crate::errors::AppError;

pub fn validate_request<T: Validate>(data: &T) -> Result<(), AppError> {
    data.validate()
        .map_err(|e| AppError::Validation(e.to_string()))
}