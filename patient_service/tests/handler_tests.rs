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
    use serde_json::json;

    async fn setup_integration_test() {
        // call the setup from the setup_env file
        setup_test_env().await;
        common::db::reset_db_for_test();
        common::db::init_db().await;
    }

    #[actix_web::test]
    async fn test_patient_post_create_1() {
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(patient_service::patient_config_v1)).await;

        // request data to create a patient
        let req_data = json!({
            "name": "Dr Renis",
            "dob": "1988-05-17",
            "gender": "female",
            "contact_info": {
                "phone": "+44-348-2992",
                "email": "rh@dragons.com",
                "address": "Dragon Stone.",
                "emergency_contact_name": "Daemon",
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

    #[actix_web::test]
    async fn test_patient_post_create_invalid_data() {
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(patient_service::patient_config_v1)).await;

        let req_data = json!({
            "name": "En", // too short, min 3
            "dob": "1961-05-17",
            "gender": "female",
            "contact_info": {
                "phone": "+44", // too short, min 5
                "email": "invalid-email",
                "address": "", // too short, min 1
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

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_patient_post_create_missing_fields() {
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(patient_service::patient_config_v1)).await;

        let req_data = json!({
            "name": "Enya",
            // missing dob, gender, contact_info
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/patient")
            .set_json(req_data)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_patient_success() {
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(patient_service::patient_config_v1)).await;

        // First, create a patient
        let unique_name = format!(
            "Enya {}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );
        let req_data = json!({
            "name": unique_name,
            "dob": "1961-05-17",
            "gender": "female",
            "contact_info": {
                "phone": "+44-348-2992",
                "email": "enya@bookings.com",
                "address": "Manderley Castle, Killiney, County Dublin, Ireland.",
                "emergency_contact_name": "Florence",
                "emergency_contact_phone": "+44-1289-1289"
            }
        });

        let create_req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/patient")
            .set_json(req_data)
            .to_request();

        let create_resp = test::call_service(&app, create_req).await;
        assert_eq!(create_resp.status(), http::StatusCode::CREATED);

        // Then list to get the ID
        let list_req = test::TestRequest::get().uri("/patient").to_request();

        let list_resp = test::call_service(&app, list_req).await;
        assert_eq!(list_resp.status(), http::StatusCode::OK);

        let patients: Vec<serde_json::Value> = test::read_body_json(list_resp).await;
        let patient = patients
            .iter()
            .find(|p| p["name"] == unique_name)
            .expect("Patient not found in list");
        let patient_id = patient["_id"].as_str().unwrap();

        // Now get that patient
        let get_req = test::TestRequest::get()
            .uri(&format!("/patient/{}", patient_id))
            .to_request();

        let get_resp = test::call_service(&app, get_req).await;
        assert_eq!(get_resp.status(), http::StatusCode::OK);

        let patient: serde_json::Value = test::read_body_json(get_resp).await;
        assert_eq!(patient["name"], unique_name);
    }

    #[actix_web::test]
    async fn test_get_patient_not_found() {
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(patient_service::patient_config_v1)).await;

        // Use a valid ObjectId format that doesn't exist
        let non_existent_id = "507f1f77bcf86cd799439011";

        let req = test::TestRequest::get()
            .uri(&format!("/patient/{}", non_existent_id))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_patient_invalid_id() {
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(patient_service::patient_config_v1)).await;

        let invalid_id = "invalid-id-format";

        let req = test::TestRequest::get()
            .uri(&format!("/patient/{}", invalid_id))
            .to_request();

        let resp = test::call_service(&app, req).await;

        // Depending on implementation, this might be 400 or 500 if it fails to parse ObjectId
        // Let's see what it returns. The handler handles it with Ok(None) or Err(e) if it fails.
        // If repository fails to parse ObjectId, it likely returns Err.
        assert!(resp.status().is_client_error() || resp.status().is_server_error());
    }

    #[actix_web::test]
    async fn test_list_patients_with_pagination() {
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(patient_service::patient_config_v1)).await;

        let req = test::TestRequest::get()
            .uri("/patient?page=1&limit=5")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_delete_patient_success() {
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(patient_service::patient_config_v1)).await;

        let req_data = json!({
            "name": "Dr. To Delete",
            "dob": "1990-01-01",
            "gender": "other",
            "contact_info": {
                "phone": "+1-555-0199",
                "email": "delete@me.com",
                "address": "123 Delete St",
                "emergency_contact_name": "Emergency",
                "emergency_contact_phone": "+1-555-0000"
            }
        });

        let create_req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/patient")
            .set_json(req_data)
            .to_request();

        // confirm the Service was created
        let create_resp = test::call_service(&app, create_req).await;
        assert_eq!(create_resp.status(), http::StatusCode::CREATED);

        let list_req = test::TestRequest::get().uri("/patient").to_request();
        let list_resp = test::call_service(&app, list_req).await;
        // assert_eq!(list_resp.status(), http::StatusCode::OK);
        let patients: Vec<serde_json::Value> = test::read_body_json(list_resp).await;
        let patient = patients
            .iter()
            .find(|p| p["name"] == "Dr. To Delete")
            .expect("Patient not found in list");
        let patient_id = patient["_id"]
            .as_str()
            .expect("Unable to retrieve PatientID.");

        let delete_req = test::TestRequest::delete()
            .uri(&format!("/patient/{}", patient_id))
            .to_request();

        let delete_resp = test::call_service(&app, delete_req).await;
        assert_eq!(delete_resp.status(), http::StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn test_delete_patient_not_found() {
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(patient_service::patient_config_v1)).await;

        let non_existent_id = "507f1f77bcf86cd799439011";

        let req = test::TestRequest::delete()
            .uri(&format!("/patient/{}", non_existent_id))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_update_patient_insurance_success() {
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(patient_service::patient_config_v1)).await;

        // Create a unique patient
        let unique_name = format!(
            "Insurance Test {}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );
        let req_data = json!({
            "name": unique_name,
            "dob": "1980-01-01",
            "gender": "male",
            "contact_info": {
                "phone": "+1-555-0111",
                "email": "insurance@test.com",
                "address": "456 Insurance Ave",
                "emergency_contact_name": "Emergency",
                "emergency_contact_phone": "+1-555-0000"
            }
        });

        let create_req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/patient")
            .set_json(req_data)
            .to_request();

        let create_resp = test::call_service(&app, create_req).await;
        assert_eq!(create_resp.status(), http::StatusCode::CREATED);

        let list_req = test::TestRequest::get().uri("/patient").to_request();
        let list_resp = test::call_service(&app, list_req).await;
        assert_eq!(list_resp.status(), http::StatusCode::OK);
        let patients: Vec<serde_json::Value> = test::read_body_json(list_resp).await;
        let patient = patients
            .iter()
            .find(|p| p["name"] == unique_name)
            .expect("Patient not found in list");
        let patient_id = patient["_id"].as_str().unwrap();

        // Update insurance
        let insurance_data = json!({
            "provider_name": "HealthGuard",
            "policy_number": "HG12345678",
            "primary_holder_name": unique_name
        });

        let update_req = test::TestRequest::put()
            .insert_header(ContentType::json())
            .uri(&format!("/patient/{}/insurance", patient_id))
            .set_json(insurance_data)
            .to_request();

        let update_resp = test::call_service(&app, update_req).await;
        assert_eq!(update_resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_update_patient_medical_alerts_success() {
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(patient_service::patient_config_v1)).await;

        // Create a unique patient
        let unique_name = format!(
            "Medical Test {}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );
        let req_data = json!({
            "name": unique_name,
            "dob": "1985-05-05",
            "gender": "female",
            "contact_info": {
                "phone": "+1-555-0222",
                "email": "medical@test.com",
                "address": "789 Medical Blvd",
                "emergency_contact_name": "Emergency",
                "emergency_contact_phone": "+1-555-0000"
            }
        });

        let create_req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/patient")
            .set_json(req_data)
            .to_request();

        let create_resp = test::call_service(&app, create_req).await;
        assert_eq!(create_resp.status(), http::StatusCode::CREATED);

        let list_req = test::TestRequest::get().uri("/patient").to_request();
        let list_resp = test::call_service(&app, list_req).await;
        assert_eq!(list_resp.status(), http::StatusCode::OK);
        let patients: Vec<serde_json::Value> = test::read_body_json(list_resp).await;
        let patient = patients
            .iter()
            .find(|p| p["name"] == unique_name)
            .expect("Patient not found in list");
        let patient_id = patient["_id"].as_str().unwrap();

        // Update medical alerts
        let medical_data = json!({
            "blood_type": "O+",
            "allergies": ["Peanuts", "Penicillin"],
            "chronic_conditions": ["Asthma"],
            "current_medications": ["Inhaler"]
        });

        let update_req = test::TestRequest::put()
            .insert_header(ContentType::json())
            .uri(&format!("/patient/{}/medical-alerts", patient_id))
            .set_json(medical_data)
            .to_request();

        let update_resp = test::call_service(&app, update_req).await;
        assert_eq!(update_resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_update_patient_contact_info_success() {
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(patient_service::patient_config_v1)).await;

        // Create a unique patient
        let unique_name = format!(
            "Contact Test {}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );
        let req_data = json!({
            "name": unique_name,
            "dob": "1995-10-10",
            "gender": "other",
            "contact_info": {
                "phone": "+1-555-0333",
                "email": "contact@test.com",
                "address": "321 Contact Rd",
                "emergency_contact_name": "Emergency",
                "emergency_contact_phone": "+1-555-0000"
            }
        });

        let create_req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/patient")
            .set_json(req_data)
            .to_request();

        let create_resp = test::call_service(&app, create_req).await;
        assert_eq!(create_resp.status(), http::StatusCode::CREATED);

        let list_req = test::TestRequest::get().uri("/patient").to_request();
        let list_resp = test::call_service(&app, list_req).await;
        assert_eq!(list_resp.status(), http::StatusCode::OK);
        let patients: Vec<serde_json::Value> = test::read_body_json(list_resp).await;
        let patient = patients
            .iter()
            .find(|p| p["name"] == unique_name)
            .expect("Patient not found in list");
        let patient_id = patient["_id"].as_str().unwrap();

        // Update contact info
        let contact_data = json!({
            "phone": "+1-555-9999",
            "email": "new-contact@test.com"
        });

        let update_req = test::TestRequest::put()
            .insert_header(ContentType::json())
            .uri(&format!("/patient/{}/contact", patient_id))
            .set_json(contact_data)
            .to_request();

        let update_resp = test::call_service(&app, update_req).await;
        assert_eq!(update_resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_update_patient_contact_info_invalid() {
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(patient_service::patient_config_v1)).await;

        let non_existent_id = "507f1f77bcf86cd799439011";

        let contact_data = json!({
            "email": "not-an-email"
        });

        let update_req = test::TestRequest::put()
            .insert_header(ContentType::json())
            .uri(&format!("/patient/{}/contact", non_existent_id))
            .set_json(contact_data)
            .to_request();

        let update_resp = test::call_service(&app, update_req).await;

        // It should be 400 Bad Request due to validation failing (in handler)
        assert_eq!(update_resp.status(), http::StatusCode::BAD_REQUEST);
    }
}
