use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum RepoError {
    #[error("Database insertion error: {0}")]
    InsertionError(String),

    #[error("Update error: {0}")]
    UpdateError(String),

    #[error("Extraction error: {0}")]
    ExtractionError(String),

    #[error("Deletion error: {0}")]
    DeletionError(String),

    #[error("Not found error: {0}")]
    NotFound(String),
}
