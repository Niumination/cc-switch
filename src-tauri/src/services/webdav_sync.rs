// Stubbed for personal build
#[allow(dead_code)]
pub struct WebDavSyncService;

pub mod archive {
    use crate::error::AppError;
    use std::path::Path;

    #[allow(dead_code)]
    pub fn zip_skills_ssot(_path: &Path) -> Result<String, AppError> {
        Err(AppError::Config("WebDAV sync disabled in personal build".into()))
    }

    #[allow(dead_code)]
    pub fn backup_current_skills() -> Result<String, AppError> {
        Err(AppError::Config("WebDAV sync disabled in personal build".into()))
    }

    #[allow(dead_code)]
    pub fn restore_skills_zip(_zip: String) -> Result<String, AppError> {
        Err(AppError::Config("WebDAV sync disabled in personal build".into()))
    }

    #[allow(dead_code)]
    pub fn restore_skills_from_backup(_backup: &str) -> Result<(), AppError> {
        Err(AppError::Config("WebDAV sync disabled in personal build".into()))
    }
}
