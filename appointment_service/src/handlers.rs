// Route handlers: get_me, update_me, etc.

use crate::{models, repository};
use actix_web::{HttpResponse, post, web};
use tracing::{error, info, instrument};
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
