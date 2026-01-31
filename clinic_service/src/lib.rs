// this should contain the Clinic & their details
mod error;
mod handlers;
mod models;
mod repository;

use actix_web::{HttpResponse, guard, web};

pub fn clinic_config_v1(cfg: &mut web::ServiceConfig) {
    tracing::info!("The clinic service is starting");
    cfg.service(
        web::scope("/clinic")
            .service(handlers::create_clinic)
            .service(handlers::get_clinic)
            .service(handlers::list_clinics)
            .service(handlers::delete_clinic)
            .service(handlers::update_clinic)
            .default_service(
                web::route()
                    .guard(guard::Head())
                    .to(HttpResponse::MethodNotAllowed),
            ),
    );
}
