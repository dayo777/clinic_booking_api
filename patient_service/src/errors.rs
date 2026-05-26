use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header::ContentType},
};
use mongodb::error::Error as MongoError;
use serde::Serialize;
// use std::fmt;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Serialize)]
struct ErrorResponse {
    pub message: String,
}

#[derive(Error, Debug)]
pub enum PatientServiceError {
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Database Error: {0}")]
    DatabaseError(#[from] MongoError),
    #[error("Patient not Found: {0}")]
    PatientNotFound(String),
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors), // for automatic conversion in handlers e.g., using `?`
    #[error("Validation error: {0}")]
    ValidationError(String), // for manual conversion in handlers
    #[error("Conflict: {0}")]
    Conflict(String),
}

impl ResponseError for PatientServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PatientNotFound(_) => StatusCode::NOT_FOUND,
            Self::ValidationError(_) | Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::Conflict(_) => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ErrorResponse {
                message: self.to_string(),
            })
    }
}
