// //! Error types and handling for the patient service.
// //!
// //! This module defines the `PatientError` enum and implements
// //! the necessary traits for Actix-web error responses.
// use actix_web::{
//     HttpResponse, ResponseError,
//     http::{StatusCode, header::ContentType},
// };
// use mongodb::error::Error as MongoError;
// use serde::Serialize;
// // use std::fmt;
// // use thiserror::Error;
//
// #[derive(Debug, Serialize)]
// struct ErrorResponse {
//     pub message: String,
// }
//
// #[derive(thiserror::Error, Debug)]
// pub enum PatientError {
//     #[error("Internal Server Error")]
//     InternalServerError,
//
//     #[error("Database Error: {0}")]
//     DatabaseError(#[from] MongoError),
//
//     #[error("Not Found: {0}")]
//     NotFound(String),
//
//     #[error("Validation Error: {0}")]
//     ValidationError(String),
//
//     #[error("Conflict: {0}")]
//     Conflict(String),
// }
//
// impl ResponseError for PatientError {
//     fn status_code(&self) -> StatusCode {
//         match self {
//             PatientError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
//             PatientError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
//             PatientError::NotFound(_) => StatusCode::NOT_FOUND,
//             PatientError::ValidationError(_) => StatusCode::BAD_REQUEST,
//             PatientError::Conflict(_) => StatusCode::CONFLICT,
//         }
//     }
//
//     fn error_response(&self) -> HttpResponse {
//         HttpResponse::build(self.status_code())
//             .insert_header(ContentType::json())
//             .json(ErrorResponse {
//                 message: self.to_string(),
//             })
//     }
// }
