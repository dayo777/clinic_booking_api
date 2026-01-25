// Route handlers for the Patient service
use crate::models;
use actix_web::{Responder, delete, get, post, put, web};
use tracing::{info, info_span};

// existence Check (HEAD): SELECT 1 FROM patients WHERE id = 123 LIMIT 1;
// (Fast: the database only checks the index and returns a single bit).
// #[head("/{id}")]
// async fn check_patient_exists(path: web::Path<u32>) -> impl Responder {
//     let id = path.into_inner();
//
//     // Call a specialized "exists" method in repository
//     if repository::patient_exists(id).await {
//         HttpResponse::Ok().finish() // Returns 200 OK, but NO BODY
//     } else {
//         HttpResponse::NotFound().finish() // Returns 404
//     }
// }

#[post("")]
async fn create_patient(payload: web::Json<models::CreatePatientDto>) -> impl Responder {
    let span = info_span!("create_patient", service.name = "patient_service");
    let _guard = span.enter();

    let _data = payload.into_inner();
    info!("Creating a new patient");

    "Create patient handler"
}

#[get("/{id}")]
async fn get_patient(path: web::Path<u32>) -> impl Responder {
    let span = info_span!("get_patient", service.name = "patient_service");
    let _guard = span.enter();

    let id = path.into_inner();
    info!("Retrieving a single patient with ID: {}", id);

    "get patient handler"
}

// TODO: put functionality for filtering, limiting
#[get("")]
async fn list_patients() -> impl Responder {
    let span = info_span!("list_patients", service.name = "patient_service");
    let _guard = span.enter();

    info!("Listing the patients");

    "listing patient handlers"
}

#[put("{id}")]
async fn update_patient(path: web::Path<u32>) -> impl Responder {
    let span = info_span!("update_patient", service.name = "patient_service");
    let _guard = span.enter();

    let id = path.into_inner();
    info!("Updating patient handler with id {}", id);

    "Updating patient handler"
}

// TODO: Patient status is changed to Archived, do not delete
#[delete("/{id}")]
async fn delete_patient(path: web::Path<u32>) -> impl Responder {
    let span = info_span!("delete_patient", service.name = "patient_service");
    let _guard = span.enter();

    let id = path.into_inner();
    info!("Removing patient handler with ID: {}", id);

    "Removing patient handler"
}

// other possible endpoints
// /{id}/appointments  -- get all appointments for this patient
// /{id}/status  -- check if patient is active or archived (deleted)
