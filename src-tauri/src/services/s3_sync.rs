// Stubbed for personal build
#[allow(dead_code)]
pub struct S3SyncService;

pub mod archive {
    use crate::error::AppError;
    use std::path::Path;

    #[allow(dead_code)]
    pub fn zip_skills_ssot(_path: &Path) -> Result<String, AppError> {
        Err(AppError::Config("S3 sync disabled in personal build".into()))
    }

    #[allow(dead_code)]
    pub fn backup_current_skills() -> Result<String, AppError> {
        Err(AppError::Config("S3 sync disabled in personal build".into()))
    }

    #[allow(dead_code)]
    pub fn restore_skills_zip(_zip: &[u8]) -> Result<String, AppError> {
        Err(AppError::Config("S3 sync disabled in personal build".into()))
    }

    #[allow(dead_code)]
    pub fn restore_skills_from_backup(_backup: &str) -> Result<(), AppError> {
        Err(AppError::Config("S3 sync disabled in personal build".into()))
    }
}

use std::sync::OnceLock;
use crate::error::AppError;

pub fn sync_mutex() -> &'static tokio::sync::Mutex<()> {
    static MUTEX: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
    MUTEX.get_or_init(|| tokio::sync::Mutex::new(()))
}

pub async fn run_with_sync_lock<T, Fut>(operation: Fut) -> Result<T, AppError>
where
    Fut: std::future::Future<Output = Result<T, AppError>>,
{
    let _guard = sync_mutex().lock().await;
    operation.await
}

pub async fn check_connection(
    _settings: &crate::settings::S3SyncSettings,
) -> Result<(), AppError> {
    Err(AppError::Config("S3 sync disabled in personal build".into()))
}

pub async fn upload(
    _db: &crate::database::Database,
    _settings: &mut crate::settings::S3SyncSettings,
) -> Result<serde_json::Value, AppError> {
    Err(AppError::Config("S3 sync disabled in personal build".into()))
}

pub async fn download(
    _db: &crate::database::Database,
    _settings: &mut crate::settings::S3SyncSettings,
) -> Result<serde_json::Value, AppError> {
    Err(AppError::Config("S3 sync disabled in personal build".into()))
}

pub async fn fetch_remote_info(
    _settings: &crate::settings::S3SyncSettings,
) -> Result<Option<serde_json::Value>, AppError> {
    Ok(None)
}
