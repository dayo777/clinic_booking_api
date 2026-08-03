mod handlers;
mod models;
mod repository;
mod utils;

use actix_web::{HttpResponse, guard, web};

pub fn appointment_config_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/appointments")
            .service(handlers::create_appointment)
            .default_service(
                web::route()
                    .guard(guard::Head())
                    .to(HttpResponse::MethodNotAllowed),
            ),
    );
}
