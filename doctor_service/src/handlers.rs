// Route handlers for the doctor service

use crate::{models, repository};
use actix_web::{HttpResponse, ResponseError, delete, get, head, patch, post, web};
use tracing::{debug, error, info, instrument};
use validator::Validate;

#[head("/{id}")]
pub(crate) async fn check_doctor_exists(path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();

    // Call a specialized "exists" method in repository
    if repository::doctor_exists(id).await {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish() // Returns 404
    }
}

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

// retrieve single Doctor data
#[get("/{id}")]
#[instrument(
    name = "get_doctor_request",
    fields(payload = %payload)
)]
pub(crate) async fn get_doctor(payload: web::Path<String>) -> HttpResponse {
    info!("Retrieving doctor information");

    match repository::get_doctor(payload.into_inner()).await {
        Ok(Some(doctor)) => {
            info!("Retrieved doctor successful");
            HttpResponse::Ok().json(doctor)
        }
        Ok(None) => {
            info!("Doctor not found");
            HttpResponse::NotFound().finish()
        }
        Err(e) => {
            debug!(cause = %e, "Failed to retrieve doctor data");
            HttpResponse::InternalServerError().finish()
        }
    }
}

// no parameters: /patients
// with parameters: /patients?page=2&limit=10
// partial parameters: /patients?page=3
#[get("")]
#[instrument(name = "list_doctors_request", skip(query))]
pub(crate) async fn list_doctors(query: web::Query<models::PaginationQuery>) -> HttpResponse {
    info!("Processing doctor list request");

    match repository::list_doctor(query.into_inner()).await {
        Ok(doctors) => {
            info!("Successfully retrieved doctor list");
            HttpResponse::Ok().json(doctors)
        }
        Err(e) => {
            debug!(cause = %e, "Failed to retrieve doctor list");
            HttpResponse::InternalServerError().finish()
        }
    }
}

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

// Doctor 'is_active' is set to false & not deleted.
#[delete("/{id}")]
#[instrument(name = "delete_doctor_request", fields(id = %path))]
pub(crate) async fn delete_doctor(path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    info!("Processing doctor deletion handler for ID: {}", &id);

    match repository::delete_doctor(id).await {
        Ok(true) => {
            info!("Doctor set to inactive/deletion");
            HttpResponse::NoContent().finish()
        }
        Ok(false) => {
            info!("Doctor not found or already inactive");
            HttpResponse::NotFound().finish()
        }
        Err(e) => {
            debug!(cause = %e, "Failed to deactivate doctor");
            HttpResponse::InternalServerError().finish()
        }
    }
}

// change the value of 'is_active' to true
#[patch("/{id}/enable")]
pub(crate) async fn enable_doctor(path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    info!("Processing doctor enablement handler for ID: {}", &id);

    match repository::enable_doctor(id).await {
        Ok(true) => {
            info!("Doctor enabled");
            HttpResponse::NoContent().finish()
        }
        Ok(false) => {
            info!("Doctor not found or already enabled");
            HttpResponse::NotFound().finish()
        }
        Err(e) => {
            debug!(cause = %e, "Failed to enable doctor");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/{id}/create-schedule")]
pub(crate) async fn create_doctor_schedule(
    path: web::Path<String>,
    payload: web::Json<Vec<models::ScheduleSlot>>,
) -> HttpResponse {
    let doctor_id = path.into_inner();

    let slots: Vec<models::ScheduleSlot> = payload
        .into_inner()
        .into_iter()
        .map(|s| models::ScheduleSlot {
            start_time: s.start_time,
            end_time: s.end_time,
            is_available: Some(true),
        })
        .collect();

    match repository::create_doctor_schedule(doctor_id, slots).await {
        Ok(slots) => {
            info!("Created Doctor schedule.");
            HttpResponse::Ok().json(slots)
        }
        Err(e) => {
            debug!("Unable to create Doctor Schedule: {:?}", e);
            e.error_response()
        }
    }
}

// other possible endpoints
// /{id}/appointments -- get all appointments for this doctor
// /{id}/status -- check if doctor is active or archived (deleted)
