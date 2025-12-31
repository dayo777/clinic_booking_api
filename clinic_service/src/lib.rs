use actix_web::{HttpResponse, web};

pub fn clinic_config_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/clinic")
            .route(web::get().to(|| async { HttpResponse::Ok().body("clinic endpoint v1") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
