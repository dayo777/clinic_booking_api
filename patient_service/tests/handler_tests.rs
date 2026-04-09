// test the handlers endpoints here

mod setup_env;

#[cfg(test)]
mod patient_service_handler_test {
    use super::setup_env::setup_test_env;
    use actix_web::{
        App,
        http::{self, header::ContentType},
        test,
    };
    use patient_service::handlers;
    use serde_json::json;

    async fn setup_integration_test() {
        // call the setup from the setup_env file
        setup_test_env().await;
        common::db::init_db().await;
    }

    #[actix_web::test]
    async fn test_patient_post_create_1() {
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(patient_service::patient_config_v1)).await;

        let req_data = json!({
            "name": "Enya",
            "dob": "1961-05-17",
            "gender": "Female",
            "contact_info": {
                "phone": "+44-348-2992",
                "email": "enya@bookings.com",
                "address": "Manderley Castle, Killiney, County Dublin, Ireland.",
                "emergency_contact_name": "Florence",
                "emergency_contact_phone": "+44-1289-1289"
            }
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/patient")
            .set_json(req_data)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::CREATED);
    }
}
