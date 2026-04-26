use actix_web::{App, test};
use doctor_service::doctor_config_v1;

#[actix_web::test]
async fn test_create_doctor_invalid_specialty() {
    let app = test::init_service(App::new().configure(doctor_config_v1)).await;

    let payload = r#"{
        "name": "Dr. Smith",
        "specialties": ["derm1"],
        "license_num": "123456"
    }"#;

    let req = test::TestRequest::post()
        .uri("/doctor")
        .insert_header(("content-type", "application/json"))
        .set_payload(payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();

    println!("Response body: {}", body_str);
    assert!(body_str.contains("invalid specialty"));
}

#[actix_web::test]
async fn test_create_doctor_valid_specialty() {
    let app = test::init_service(App::new().configure(doctor_config_v1)).await;

    let payload = r#"{
        "name": "Dr. Smith",
        "specialties": ["cardio"],
        "license_num": "123456"
    }"#;

    let req = test::TestRequest::post()
        .uri("/doctor")
        .insert_header(("content-type", "application/json"))
        .set_payload(payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // It might fail with InternalServerError because of no DB, but it shouldn't be BadRequest due to validation
    assert_ne!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
}
