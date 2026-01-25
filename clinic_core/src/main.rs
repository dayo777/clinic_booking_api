mod logging;

use actix_web::guard::{Guard, GuardContext};
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{App, HttpServer, Responder, web};
use tracing_actix_web::TracingLogger;

// import Route configs below
use appointment_service::appointment_config_v1;
use clinic_service::clinic_config_v1;
use doctor_service::doctor_config_v1;
use patient_service::patient_config_v1;

#[actix_web::main]
#[tracing::instrument]
async fn main() -> std::io::Result<()> {
    logging::init_tracing("clinic_core");
    tracing::info!("Starting clinic_core service");

    HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(TracingLogger::default())
            .service(
                web::scope("/api")
                    .guard(ApiVersion("1".to_string()))
                    .configure(appointment_config_v1)
                    .configure(doctor_config_v1)
                    .configure(clinic_config_v1)
                    .configure(patient_config_v1)
                    .route("", web::get().to(hello)),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

// TODO: remove line below
async fn hello() -> impl Responder {
    "Welcome to v1!"
}

struct ApiVersion(String);

// confirm the header contains the version
impl Guard for ApiVersion {
    fn check(&self, ctx: &GuardContext) -> bool {
        ctx.head()
            .headers()
            .get("x-api-version")
            .and_then(|v| v.to_str().ok())
            == Some(&self.0)
    }
}
