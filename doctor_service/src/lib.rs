use actix_web::{HttpResponse, web};

pub fn doctor_config_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/doctor")
            .route(web::get().to(|| async { HttpResponse::Ok().body("doctor endpoint v1") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
