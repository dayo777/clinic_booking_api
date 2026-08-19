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
use validator::ValidationError;

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Error, Debug)]
pub enum AppointmentScheduleError {
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError), // for automatic conversion in handlers e.g., using `?`
    #[error("Invalid doctor ID format")]
    InvalidDoctorId,
    #[error("Invalid Patient ID format")]
    InvalidPatientId,
    #[error("Invalid Slot ID format")]
    InvalidSlotId,
    #[error("Unable to find appointment")]
    AppointmentNotFound,
    #[error("Unable to schedule appointment")]
    UnableToScheduleAppointment,
    #[error("Database error: {0}")]
    Database(#[from] MongodbError),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl ResponseError for AppointmentScheduleError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Validation(_)
            | Self::InvalidDoctorId
            | Self::InvalidPatientId
            | Self::InvalidSlotId => StatusCode::BAD_REQUEST,
            Self::Database(ref e) => {
                error!("Database error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Internal(ref e) => {
                error!("Internal error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::AppointmentNotFound => StatusCode::NOT_FOUND,
            Self::UnableToScheduleAppointment => StatusCode::FORBIDDEN,
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
