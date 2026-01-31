//! HTTP request handlers for the patient service.
//!
//! This module contains the Actix-web route handlers for CRUD operations
//! on patient resources.
use crate::{models, repository};
use actix_web::{HttpResponse, delete, get, post, put, web};
use tracing::{debug, info, info_span, instrument};

// existence Check (HEAD): SELECT 1 FROM patients WHERE id = 123 LIMIT 1;
// (Fast: the database only checks the index and returns a single bit).
// #[head("/{id}")]
// async fn check_patient_exists(path: web::Path<u32>) -> HttpResponse {
//     let id = path.into_inner();
//
//     // Call a specialized "exists" method in repository
//     if repository::patient_exists(id).await {
//         HttpResponse::Ok().finish() // Returns 200 OK, but no BODY
//     } else {
//         HttpResponse::NotFound().finish() // Returns 404
//     }
// }

#[post("")]
#[instrument(skip(payload), fields(patient_name = ?payload.name))]
async fn create_patient(payload: web::Json<models::CreatePatientDto>) -> HttpResponse {
    let span = info_span!("create_patient", service.name = "patient_service");
    let _guard = span.enter();

    info!("Entering the create patient handler");

    match repository::create_patient(payload).await {
        Ok(_) => {
            info!("Successfully created patient record.");
            HttpResponse::Created().finish()
        }
        Err(_) => {
            debug!("Failed to create patient record.");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{id}")]
async fn get_patient(path: web::Path<u32>) -> HttpResponse {
    let span = info_span!("get_patient", service.name = "patient_service");
    let _guard = span.enter();

    let id = path.into_inner();
    info!("Retrieving a single patient with ID: {}", id);

    // TODO: modify
    HttpResponse::Ok().finish()
}

// TODO: put functionality for filtering, limiting
#[get("")]
async fn list_patients() -> HttpResponse {
    let span = info_span!("list_patients", service.name = "patient_service");
    let _guard = span.enter();

    info!("Listing the patients");

    // TODO: modify
    HttpResponse::Ok().body("listing patients")
}

#[put("{id}")]
async fn update_patient(path: web::Path<u32>) -> HttpResponse {
    let span = info_span!("update_patient", service.name = "patient_service");
    let _guard = span.enter();

    let id = path.into_inner();
    info!("Updating patient handler with id {}", id);

    // TODO: modify
    HttpResponse::Ok().body("updated patient handler")
}

// TODO: Patient status is changed to Archived, do not delete
#[delete("/{id}")]
async fn delete_patient(path: web::Path<u32>) -> HttpResponse {
    let span = info_span!("delete_patient", service.name = "patient_service");
    let _guard = span.enter();

    let id = path.into_inner();
    info!("Removing patient handler with ID: {}", id);

    // TODO: modify
    HttpResponse::Ok().body("removing patient handler")
}

// other possible endpoints
// /{id}/appointments  -- get all appointments for this patient
// /{id}/status  -- check if patient is active or archived (deleted)
