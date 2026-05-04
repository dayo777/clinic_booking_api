mod setup_env;

#[cfg(test)]
mod doctor_specialty_test {
    use super::setup_env::setup_test_env;
    use actix_web::{App, http, test};
    use doctor_service::doctor_config_v1;

    async fn setup_integration_test() {
        setup_test_env().await;
        common::db::reset_db_for_test();
        common::db::init_db().await;
    }

    #[actix_web::test]
    async fn test_create_doctor_invalid_specialty() {
        setup_integration_test().await;
        let app = test::init_service(App::new().configure(doctor_config_v1)).await;

        let payload = serde_json::json!({
            "name": "Dr. Smith",
            "specialties": ["derm1"],
            "license_num": "123456"
        });

        let req = test::TestRequest::post()
            .uri("/doctor")
            .insert_header(("content-type", "application/json"))
            .set_json(payload)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);

        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();

        assert!(body_str.contains("invalid specialty"));
    }

    #[actix_web::test]
    async fn test_create_doctor_valid_specialty() {
        setup_integration_test().await;
        let app = test::init_service(App::new().configure(doctor_config_v1)).await;

        let payload = serde_json::json!({
            "name": "Dr. Smith",
            "specialties": ["cardio"],
            "license_num": "123456"
        });

        let req = test::TestRequest::post()
            .uri("/doctor")
            .insert_header(("content-type", "application/json"))
            .set_json(payload)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::CREATED);
    }
}
