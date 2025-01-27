use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid task ID")]
    InvalidTaskId,
}

pub type Result<T> = std::result::Result<T, TodoError>;
