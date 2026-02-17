use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::json;
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

    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
}

impl RepoError {
    pub fn to_response(&self) -> HttpResponse {
        match self {
            RepoError::NotFound(_) => HttpResponse::NotFound().json(json!({"error": "not found"})),
            RepoError::ConstraintViolation(_) => {
                HttpResponse::Conflict().json(json!({"error": "constraint violation"}))
            }
            RepoError::InsertionError(_) | RepoError::UpdateError(_) => {
                HttpResponse::UnprocessableEntity().json(json!({"error": "operation failed"}))
            }
            RepoError::ExtractionError(_) | RepoError::DeletionError(_) => {
                HttpResponse::InternalServerError().json(json!({"error": "internal error"}))
            }
        }
    }
}
