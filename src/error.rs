use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum PerleError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error), // ← this generates From<io::Error>

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error), // ← this generates From<serde_json::Error>

    #[error("Audio error: {0}")]
    Audio(String),

    #[error("Timer error: {0}")]
    Timer(String),
}

pub type Result<T> = std::result::Result<T, PerleError>;
