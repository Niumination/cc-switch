use crate::error::AppError;

pub fn enable_auto_launch() -> Result<(), AppError> {
    Err(AppError::Generic("Auto-launch disabled in personal build".into()))
}

pub fn disable_auto_launch() -> Result<(), AppError> {
    Err(AppError::Generic("Auto-launch disabled in personal build".into()))
}

pub fn is_auto_launch_enabled() -> Result<bool, AppError> {
    Ok(false)
}
