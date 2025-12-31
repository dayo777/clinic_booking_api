use actix_web::{App, test};
use appointment_service::appointment_config_v1;
use clinic_service::clinic_config_v1;
#[actix_web::test]
async fn test_appointment_get() {
    let app = test::init_service(App::new().configure(appointment_config_v1)).await;
    let req = test::TestRequest::get().uri("/appointments").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_clinic_get() {
    let app = test::init_service(App::new().configure(clinic_config_v1)).await;
    let req = test::TestRequest::get().uri("/clinic").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

// this test below is meant to fail
// #[actix_web::test]
// async fn test_clinic_get1() {
//     let app = test::init_service(
//         App::new().configure(clinic_config_v1)
//     ).await;
//     let req = test::TestRequest::get().uri("/clinic1").to_request();
//     let resp = test::call_service(&app, req).await;
//     assert!(resp.status().is_success());
// }
