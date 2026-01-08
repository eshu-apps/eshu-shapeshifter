use thiserror::Error;

#[derive(Error, Debug)]
pub enum EshuError {
    #[error("Unsupported distribution: {0}")]
    UnsupportedDistro(String),

    #[error("Package manager error: {0}")]
    PackageManager(String),

    #[error("Snapshot error: {0}")]
    Snapshot(String),

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("File system error: {0}")]
    FileSystem(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

pub type EshuResult<T> = Result<T, EshuError>;
