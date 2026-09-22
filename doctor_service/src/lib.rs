/// Doctor Schedule is designed such that a Doctor can only have one ScheduleID
pub(crate) mod error;
pub mod handlers;
pub mod models;
pub mod repository;
pub mod utils;

use actix_web::{HttpResponse, guard, web};

pub fn doctor_config_v1(cfg: &mut web::ServiceConfig) {
    tracing::info!("The doctor service is starting");
    cfg.service(
        web::scope("/doctor")
            .service(handlers::check_doctor_exists)
            .service(handlers::create_doctor)
            .service(handlers::get_doctor)
            .service(handlers::list_doctors)
            .service(handlers::delete_doctor)
            .service(handlers::enable_doctor)
            .service(handlers::create_doctor_schedule)
            .service(handlers::get_active_doctor_schedule)
            // .service(handlers::update_doctor)
            .default_service(
                web::route()
                    .guard(guard::Head())
                    .to(HttpResponse::MethodNotAllowed),
            ),
    );
}

// TODO: test active-doctor-schedule endpoint
// TODO: continue from the repository module on line 347
// TODO: confirm the test for handler_test - Doctor active schedule is working
