// Route handlers: get_me, update_me, etc.

use crate::{models, repository};
use actix_web::{HttpResponse, get, patch, post, web};
use tracing::{debug, error, info, instrument};
use validator::Validate;

#[post("")]
#[instrument(name = "create_appointment_request", fields(id = ?payload.slot_id))]
pub(crate) async fn create_appointment(
    payload: web::Json<models::CreateAppointmentDto>,
) -> HttpResponse {
    info!("creating new Appointment for slot {:?}", payload.slot_id);

    if let Err(e) = payload.validate() {
        error!(
            "Validation checks failed for creation Appointment: {}",
            e.to_string()
        );
        return HttpResponse::BadRequest().body("Unable to create appointment.");
    }

    match repository::create_appointment(payload.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
#[instrument(name = "get_appointment_request", fields(payload = ?payload))]
pub(crate) async fn get_appointment(payload: web::Path<String>) -> HttpResponse {
    info!("retrieving Appointment ID");

    match repository::get_appointment(payload.into_inner()).await {
        Ok(Some(appointment)) => {
            info!("Appointment found");
            HttpResponse::Ok().json(appointment)
        }
        Ok(None) => {
            info!("Appointment not found");
            HttpResponse::NotFound().finish()
        }
        Err(e) => {
            debug!(cause = %e, "Error retrieving Appointment");
            HttpResponse::InternalServerError().finish()
        }
    }
}

// TODO: Pagination and filtering should be added to this
#[get("/patient/{patient_id}")]
#[instrument(name = "get_all_patient_appointments_request", fields(patient_id = ?patient_id))]
pub(crate) async fn get_all_patient_appointment(
    patient_id: web::Path<String>,
    pagination: web::Query<models::PaginationQuery>,
) -> HttpResponse {
    info!("retrieving all appointments for patient");

    match repository::get_all_patient_appointments(patient_id.into_inner(), pagination.into_inner())
        .await
    {
        Ok(appointments) => {
            info!("Successfully retrieved patient appointments");
            HttpResponse::Ok().json(appointments)
        }
        Err(e) => {
            debug!(cause = %e, "Error retrieving appointments");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/doctor/{app_id}")]
#[instrument(name = "get_all_doctor_appointments_request", fields(appoint_id = ?appoint_id))]
pub(crate) async fn get_all_doctor_appointment(
    appoint_id: web::Path<String>,
    pagination: web::Query<models::PaginationQuery>,
) -> HttpResponse {
    info!("retrieving all appointments for doctor");

    match repository::get_all_doctor_appointments(appoint_id.into_inner(), pagination.into_inner())
        .await
    {
        Ok(appointments) => {
            info!("Successfully retrieved doctor appointments");
            HttpResponse::Ok().json(appointments)
        }
        Err(e) => {
            debug!(cause = %e, "Error retrieving appointments");
            HttpResponse::InternalServerError().finish()
        }
    }
}

// endpoint used by the Doctor to confirm an appointment
// changes AppointmentStatus from `Scheduled` to `Confirmed`
#[patch("/{appointment_id}/confirm")]
#[instrument(name = "confirm_doctor_appointment_request", fields(id = ?app_id))]
pub(crate) async fn confirm_appointment(
    payload: web::Json<models::UpdateAppointmentStatusDto>,
    app_id: web::Path<String>,
) -> HttpResponse {
    info!("confirming appointment");
    let appointment_id = app_id.into_inner();
    let payload = payload.into_inner();
    match repository::confirm_appointment(appointment_id, payload.reason.unwrap()).await {
        Ok(_) => {
            info!("Successfully confirmed appointment");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            debug!(cause = %e, "Error confirming appointment");
            HttpResponse::InternalServerError().finish()
        }
    }
}

// endpoint used by the Doctor to cancel an appointment
// changes AppointmentStatus from `Scheduled` to `Canceled`
#[patch("/{appointment_id}/cancel")]
#[instrument(name = "cancel_doctor_appointment_request", fields(id = ?app_id))]
pub(crate) async fn cancel_appointment(
    payload: web::Json<models::UpdateAppointmentStatusDto>,
    app_id: web::Path<String>,
) -> HttpResponse {
    info!("cancelling appointment");
    let appointment_id = app_id.into_inner();
    let payload = payload.into_inner();
    match repository::cancel_appointment(appointment_id, payload.reason.unwrap()).await {
        Ok(_) => {
            info!("Successfully canceled appointment");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            debug!(cause = %e, "Error canceling appointment");
            HttpResponse::InternalServerError().finish()
        }
    }
}
