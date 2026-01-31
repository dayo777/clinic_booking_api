// Route handlers: get_me, update_me, etc.
// Route handlers for the doctor service
use crate::models;
use actix_web::{Responder, delete, get, post, put, web};
use tracing::{info, info_span};

// existence Check (HEAD): SELECT 1 FROM doctors WHERE id = 123 LIMIT 1;
// (Fast: the database only checks the index and returns a single bit).
// #[head("/{id}")]
// async fn check_doctor_exists(path: web::Path<u32>) -> impl Responder {
//     let id = path.into_inner();
//
//     // Call a specialized "exists" method in repository
//     if repository::doctor_exists(id).await {
//         HttpResponse::Ok().finish() // Returns 200 OK, but no BODY
//     } else {
//         HttpResponse::NotFound().finish() // Returns 404
//     }
// }

#[post("")]
async fn create_doctor(payload: web::Json<models::CreateDoctorDto>) -> impl Responder {
    let span = info_span!("create_doctor", service.name = "doctor_service");
    let _guard = span.enter();

    let _data = payload.into_inner();
    info!("Creating a new doctor");

    "Create doctor handler"
}

#[get("/{id}")]
async fn get_doctor(path: web::Path<u32>) -> impl Responder {
    let span = info_span!("get_doctor", service.name = "doctor_service");
    let _guard = span.enter();

    let id = path.into_inner();
    info!("Retrieving a single doctor with ID: {}", id);

    "get doctor handler"
}

// TODO: put functionality for filtering, limiting
#[get("")]
async fn list_doctors() -> impl Responder {
    let span = info_span!("list_doctors", service.name = "doctor_service");
    let _guard = span.enter();

    info!("Listing the doctors");

    "listing doctor handlers"
}

#[put("{id}")]
async fn update_doctor(path: web::Path<u32>) -> impl Responder {
    let span = info_span!("update_doctor", service.name = "doctor_service");
    let _guard = span.enter();

    let id = path.into_inner();
    info!("Updating doctor handler with id {}", id);

    "Updating doctor handler"
}

// TODO: doctor status is changed to Archived, do not delete
#[delete("/{id}")]
async fn delete_doctor(path: web::Path<u32>) -> impl Responder {
    let span = info_span!("delete_doctor", service.name = "doctor_service");
    let _guard = span.enter();

    let id = path.into_inner();
    info!("Removing doctor handler with ID: {}", id);

    "Removing doctor handler"
}

// other possible endpoints
// /{id}/appointments  -- get all appointments for this doctor
// /{id}/status  -- check if doctor is active or archived (deleted)
