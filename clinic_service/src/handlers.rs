// Route handlers for the Clinic service
use crate::models;
use actix_web::{Responder, delete, get, post, put, web};
use tracing::{info, info_span};

// existence Check (HEAD): SELECT 1 FROM clinics WHERE id = 123 LIMIT 1;
// (Fast: the database only checks the index and returns a single bit).
// #[head("/{id}")]
// async fn check_clinic_exists(path: web::Path<u32>) -> impl Responder {
//     let id = path.into_inner();
//
//     // Call a specialized "exists" method in repository
//     if repository::clinic_exists(id).await {
//         HttpResponse::Ok().finish() // Returns 200 OK, but NO BODY
//     } else {
//         HttpResponse::NotFound().finish() // Returns 404
//     }
// }

#[post("")]
async fn create_clinic(payload: web::Json<models::CreateClinicDto>) -> impl Responder {
    let span = info_span!("create_clinic", service.name = "clinic_service");
    let _guard = span.enter();

    let _data = payload.into_inner();
    info!("Creating a new clinic");

    "Create clinic handler"
}

#[get("/{id}")]
async fn get_clinic(path: web::Path<u32>) -> impl Responder {
    let span = info_span!("get_clinic", service.name = "clinic_service");
    let _guard = span.enter();

    let id = path.into_inner();
    info!("Retrieving a single clinic with ID: {}", id);

    "get clinic handler"
}

// TODO: put functionality for filtering, limiting
#[get("")]
async fn list_clinics() -> impl Responder {
    let span = info_span!("list_clinics", service.name = "clinic_service");
    let _guard = span.enter();

    info!("Listing the clinics");

    "listing clinic handlers"
}

#[put("{id}")]
async fn update_clinic(path: web::Path<u32>) -> impl Responder {
    let span = info_span!("update_clinic", service.name = "clinic_service");
    let _guard = span.enter();

    let id = path.into_inner();
    info!("Updating clinic handler with id {}", id);

    "Updating clinic handler"
}

// TODO: clinic status is changed to Archived, do not delete
#[delete("/{id}")]
async fn delete_clinic(path: web::Path<u32>) -> impl Responder {
    let span = info_span!("delete_clinic", service.name = "clinic_service");
    let _guard = span.enter();

    let id = path.into_inner();
    info!("Removing clinic handler with ID: {}", id);

    "Removing clinic handler"
}

// other possible endpoints
// /{id}/appointments  -- get all appointments for this clinic
// /{id}/status  -- check if clinic is active or archived (deleted)
