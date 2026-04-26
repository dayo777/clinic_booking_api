mod error;
pub mod handlers;
pub mod models;
pub mod repository;
pub mod utils;

use actix_web::{HttpResponse, guard, web};

pub fn doctor_config_v1(cfg: &mut web::ServiceConfig) {
    tracing::info!("The doctor service is starting");
    cfg.service(
        web::scope("/doctor")
            .service(handlers::create_doctor)
            // .service(handlers::get_doctor)
            // .service(handlers::list_doctors)
            // .service(handlers::delete_doctor)
            // .service(handlers::update_doctor)
            .default_service(
                web::route()
                    .guard(guard::Head())
                    .to(HttpResponse::MethodNotAllowed),
            ),
    );
}
