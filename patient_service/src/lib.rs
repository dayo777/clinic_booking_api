mod error;
mod handlers;
mod models;
mod repository;

use actix_web::{HttpResponse, guard, web};

pub fn patient_config_v1(cfg: &mut web::ServiceConfig) {
    tracing::info!("The patient service is starting");
    cfg.service(
        web::scope("/patient")
            .service(handlers::create_patient)
            .service(handlers::get_patient)
            .service(handlers::list_patients)
            .service(handlers::delete_patient)
            .service(handlers::update_patient)
            .default_service(
                web::route()
                    .guard(guard::Head())
                    .to(HttpResponse::MethodNotAllowed),
            ),
    );
}
