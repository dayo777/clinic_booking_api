use actix_web::guard::{Guard, GuardContext};
use actix_web::{App, HttpServer, Responder, middleware::NormalizePath, web};

// import Route configs below
use appointment_service::appointment_config_v1;
use clinic_service::clinic_config_v1;
use doctor_service::doctor_config_v1;
use patient_service::patient_config_v1;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().wrap(NormalizePath::default()).service(
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
