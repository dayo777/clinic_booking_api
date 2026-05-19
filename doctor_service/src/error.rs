// Service-specific errors
use actix_web::{
    HttpResponse,
    error::ResponseError,
    http::{StatusCode, header::ContentType},
};
use mongodb::error::Error as MongodbError;
use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Serialize)]
struct ErrorResponse {
    pub message: String,
}

#[derive(Error, Debug)]
pub enum DoctorServiceError {
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Invalid doctor ID format")]
    InvalidDoctorId,
    #[error("Doctor not found")]
    DoctorNotFound,
    #[error("Doctor is inactive: {0}")]
    DoctorInactive(String),
    #[error("Database error: {0}")]
    Database(#[from] MongodbError),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl ResponseError for DoctorServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Validation(_) | Self::InvalidDoctorId => StatusCode::BAD_REQUEST,
            Self::DoctorNotFound => StatusCode::NOT_FOUND,
            Self::DoctorInactive(_) => StatusCode::FORBIDDEN,
            Self::Database(ref e) => {
                error!("Database error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
