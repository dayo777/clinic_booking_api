// Route handlers: get_me, update_me, etc.

use crate::{models, repository};
use actix_web::{HttpResponse, post, web};
use tracing::{info, instrument};

#[post("")]
#[instrument(name = "create_appointment_request", fields(id = ?_payload.appointment_id))]
pub async fn create_appointment(_payload: web::Json<models::CreateAppointmentDto>) -> HttpResponse {
    info!(
        "creating new Appointment with payload {:?}",
        _payload.appointment_id
    );
    match repository::create_appointment(_payload.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
