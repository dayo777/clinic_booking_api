use actix_web::{HttpResponse, web};

pub fn patient_config_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/patient")
            .route(web::get().to(|| async { HttpResponse::Ok().body("patient endpoint v1") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
