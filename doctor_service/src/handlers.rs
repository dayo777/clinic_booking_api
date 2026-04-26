// Route handlers for the doctor service

use crate::{models, repository};
use actix_web::{HttpResponse, post, web};
use tracing::{error, info, instrument};
use validator::Validate;

// existence Check (HEAD): SELECT 1 FROM doctors WHERE id = 123 LIMIT 1;
// (Fast: the database only checks the index and returns a single bit).
// #[head("/{id}")]
// async fn check_doctor_exists(path: web::Path<u32>) -> HttpResponse {
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
#[instrument(
    name = "create_doctor_request",
    skip(payload),
    fields(payload = %payload.name)
)]
pub(crate) async fn create_doctor(payload: web::Json<models::CreateDoctorDto>) -> HttpResponse {
    info!(
        "Processing new doctor registration for: {}",
        payload.license_num
    );

    if let Err(e) = payload.validate() {
        error!(
            "Validation checks failed for creation for Doctor {}: {:?}",
            payload.name, e
        );

        let error_message = if e.to_string().contains("invalid specialty") {
            "invalid specialty"
        } else {
            "Error saving Doctor details."
        };

        return HttpResponse::BadRequest().body(error_message);
    }

    let dto = payload.into_inner();
    match repository::create_doctor(dto).await {
        Ok(_) => {
            info!("Doctor registration successful");
            HttpResponse::Created().finish()
        }
        Err(e) => {
            error!(error =%e, "Doctor registration failed");
            HttpResponse::InternalServerError().finish()
        }
    }
}

// #[get("/{id}")]
// async fn get_doctor(path: web::Path<u32>) -> HttpResponse {
//     let span = span!("get_doctor", service.name = "doctor_service");
//     let _guard = span.enter();
//
//     let id = path.into_inner();
//     info!("Retrieving a single doctor with ID: {}", id);
//
//     "get doctor handler"
// }

// TODO: put functionality for filtering, limiting
// #[get("")]
// async fn list_doctors() -> HttpResponse {
//     let span = span!("list_doctors", service.name = "doctor_service");
//     let _guard = span.enter();
//
//     info!("Listing the doctors");
//
//     "listing doctor handlers"
// }

// #[put("{id}")]
// async fn update_doctor(path: web::Path<u32>) -> HttpResponse {
//     let span = span!("update_doctor", service.name = "doctor_service");
//     let _guard = span.enter();
//
//     let id = path.into_inner();
//     info!("Updating doctor handler with id {}", id);
//
//     "Updating doctor handler"
// }

// // TODO: doctor status is changed to Archived, do not delete
// #[delete("/{id}")]
// async fn delete_doctor(path: web::Path<u32>) -> HttpResponse {
//     let span = span!("delete_doctor", service.name = "doctor_service");
//     let _guard = span.enter();
//
//     let id = path.into_inner();
//     info!("Removing doctor handler with ID: {}", id);
//
//     "Removing doctor handler"
// }

// other possible endpoints
// /{id}/appointments -- get all appointments for this doctor
// /{id}/status -- check if doctor is active or archived (deleted)
