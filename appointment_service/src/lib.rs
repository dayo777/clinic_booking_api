use actix_web::{HttpResponse, web};

pub fn appointment_config_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/appointments")
            .route(web::get().to(|| async { HttpResponse::Ok().body("appointments endpoint v1") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
