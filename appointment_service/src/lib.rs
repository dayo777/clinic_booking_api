mod error;
mod handlers;
pub mod models;
pub mod repository;
mod utils;

use actix_web::{HttpResponse, guard, web};

pub fn appointment_config_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/appointment")
            .service(handlers::create_appointment)
            .service(handlers::get_appointment)
            .service(handlers::get_all_patient_appointment)
            .service(handlers::get_all_doctor_appointment)
            .service(handlers::confirm_appointment)
            .service(handlers::cancel_appointment)
            .service(handlers::no_show_appointment)
            .service(handlers::complete_appointment)
            .service(handlers::appointment_status_history)
            .default_service(
                web::route()
                    .guard(guard::Head())
                    .to(HttpResponse::MethodNotAllowed),
            ),
    );
}

// TODO: list of other possible endpoints
// - reschedule appointment, one for doctor, one for patient;
// -
