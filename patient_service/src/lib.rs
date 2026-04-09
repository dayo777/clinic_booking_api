//! Patient Service
//!
//! This crate provides the patient management functionality for the clinic booking API,
//! including registration, retrieval, and management of patient records.

// To view this service structure, use the cmd `cargo modules structure --package patient_service`

mod error;
pub mod handlers;
pub mod models;
mod repository;
pub mod utils;

use actix_web::{HttpResponse, guard, web};

pub fn patient_config_v1(cfg: &mut web::ServiceConfig) {
    tracing::info!("The patient service is starting");
    cfg.service(
        web::scope("/patient")
            .service(handlers::create_patient)
            .service(handlers::get_patient)
            .service(handlers::list_patients)
            .service(handlers::delete_patient)
            // .service(handlers::update_patient)
            .service(handlers::update_patient_insurance)
            .service(handlers::update_patient_medical_alerts)
            .service(handlers::update_patient_contact_info)
            .default_service(
                web::route()
                    .guard(guard::Head())
                    .to(HttpResponse::MethodNotAllowed),
            ),
    );
}
