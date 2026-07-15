// test the handler endpoints here

mod setup_env;
// use doctor_service::models;

#[cfg(test)]
mod doctor_service_handler_test {
    use super::setup_env::setup_test_env;
    use actix_web::{
        App,
        http::{self, header::ContentType},
        test,
    };
    use serde_json::json;
    // use super::models; // this is from the Models imported on line 4

    async fn setup_integration_test() {
        // call the setup from the setup_env file
        setup_test_env().await;
        common::db::reset_db_for_test();
        common::db::init_db().await;
    }

    #[actix_web::test]
    async fn test_doctor_post_create_1() {
        setup_integration_test().await;

        let app = test::init_service(App::new().configure(doctor_service::doctor_config_v1)).await;

        let req_data = json!({
            "name": "Dr. Dru",
            "specialties": ["cardio", "derm"],
            "license_num": "LIC12345"
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/doctor")
            .set_json(req_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    // request should fail due to name being less than 3
    #[actix_web::test]
    async fn test_doctor_post_create_invalid_name() {
        setup_integration_test().await;

        let app = test::init_service(App::new().configure(doctor_service::doctor_config_v1)).await;

        let req_data = json!({
            "name": "Dr", // too short, less than 3
            "specialties": ["gp", "neuro"],
            "license_num": "LIC-123"
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/doctor")
            .set_json(req_data)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    // request should fail due to Bad specialty
    #[actix_web::test]
    async fn test_doctor_post_create_invalid_specialty() {
        setup_integration_test().await;

        let app = test::init_service(App::new().configure(doctor_service::doctor_config_v1)).await;

        let req_data = json!({
            "name": "Dr Dru",
            "specialties": ["invalid-specialty"],
            "license_num": "123"
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/doctor")
            .set_json(req_data)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    // request should fail due to License-number being less than 5
    #[actix_web::test]
    async fn test_doctor_post_create_invalid_license() {
        setup_integration_test().await;

        let app = test::init_service(App::new().configure(doctor_service::doctor_config_v1)).await;

        let req_data = json!({
            "name": "Dr Steve",
            "specialties": ["neuro", "derm"],
            "license_num": "123" // too short, min 5
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/doctor")
            .set_json(req_data)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    // ensure you can retrieve a Dcotor object
    #[actix_web::test]
    async fn test_get_doctor_success() {
        setup_integration_test().await;

        let app = test::init_service(App::new().configure(doctor_service::doctor_config_v1)).await;

        // First, create a doctor
        let unique_name = format!(
            "Dr. House {}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );
        let req_data = json!({
            "name": unique_name,
            "specialties": ["gp"],
            "license_num": "LIC99999"
        });

        let create_req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/doctor")
            .set_json(req_data)
            .to_request();

        let create_resp = test::call_service(&app, create_req).await;
        assert_eq!(create_resp.status(), http::StatusCode::CREATED);

        // I will temporarily change the test to use a known license_num to find it in DB.
        let doctor_id = {
            let collection = common::db::get_collection::<doctor_service::models::DoctorResponseDto>(
                "doctors_collection",
            );
            let filter = mongodb::bson::doc! { "name": &unique_name };
            let doctor_doc = collection
                .find_one(filter)
                .await
                .unwrap()
                .expect("Doctor not found in DB");
            doctor_doc.id.to_hex()
        };

        // Enable the doctor
        let enable_req = test::TestRequest::patch()
            .uri(&format!("/doctor/{}/enable", doctor_id))
            .to_request();
        let enable_resp = test::call_service(&app, enable_req).await;
        assert_eq!(enable_resp.status(), http::StatusCode::NO_CONTENT);

        // Now get that doctor
        let get_req = test::TestRequest::get()
            .uri(&format!("/doctor/{}", doctor_id))
            .to_request();

        let get_resp = test::call_service(&app, get_req).await;
        assert_eq!(get_resp.status(), http::StatusCode::OK);

        let doctor: serde_json::Value = test::read_body_json(get_resp).await;
        assert_eq!(doctor["name"], unique_name);
    }

    #[actix_web::test]
    async fn test_list_doctors_with_pagination() {
        setup_integration_test().await;

        let app = test::init_service(App::new().configure(doctor_service::doctor_config_v1)).await;

        let req = test::TestRequest::get()
            .uri("/doctor?page=1&limit=5")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_delete_doctor_success() {
        setup_integration_test().await;

        let app = test::init_service(App::new().configure(doctor_service::doctor_config_v1)).await;

        // Create a doctor to delete
        let req_data = json!({
            "name": "Dru",
            "specialties": ["gp"],
            "license_num": "LIC-DEL"
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/doctor")
            .set_json(&req_data)
            .to_request();

        // call & confirm Post success before proceeding
        let create_resp = test::call_service(&app, req).await;
        assert_eq!(create_resp.status(), http::StatusCode::CREATED);

        // get the DoctorID from the response
        let doctor_id: serde_json::Value = test::read_body_json(create_resp).await;
        let doctor_id = doctor_id
            .as_str()
            .unwrap()
            .trim_start_matches("ObjectId(\"")
            .trim_end_matches("\")");
        // let doctor_id = doctor_id.to_string();

        // run the delete request
        let delete_req = test::TestRequest::delete()
            .insert_header(("x-api-version", "1"))
            .uri(&format!("/doctor/{}", doctor_id))
            .to_request();

        let delete_resp = test::call_service(&app, delete_req).await;
        assert_eq!(delete_resp.status(), http::StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn test_create_doctor_schedule_success() {
        setup_integration_test().await;

        let app = test::init_service(App::new().configure(doctor_service::doctor_config_v1)).await;

        // first create a Doctor so we can have a DoctorId to work with
        let req_data = json!({
            "name": "Dr. House",
            "specialties": ["gp"],
            "license_num": "LIC99999"
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/doctor")
            .set_json(&req_data)
            .to_request();

        // call & confirm Post success before proceeding
        let create_resp = test::call_service(&app, req).await;
        assert_eq!(create_resp.status(), http::StatusCode::CREATED);

        // get the DoctorID from the response
        let doctor_id: serde_json::Value = test::read_body_json(create_resp).await;
        let doctor_id = doctor_id
            .as_str()
            .unwrap()
            .trim_start_matches("ObjectId(\"")
            .trim_end_matches("\")");

        println!("DoctorID: {}", doctor_id); // TODO: remove DoctorID

        // Enable the doctor before creating a schedule
        let enable_req = test::TestRequest::patch()
            .insert_header(("x-api-version", "1"))
            .uri(&format!("/doctor/{}/enable", doctor_id))
            .to_request();
        let enable_resp = test::call_service(&app, enable_req).await;
        assert_eq!(enable_resp.status(), http::StatusCode::NO_CONTENT);

        // create a schedule slot for doctorID
        let req_data2 = json!([
            {
                "start_time": "2028-10-01T08:00:00Z",
                "end_time": "2028-10-01T08:30:00Z"
            },
            {
                "start_time": "2028-10-01T09:00:00Z",
                "end_time": "2028-10-01T09:30:00Z"
            }
        ]);

        let req2 = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri(format!("/doctor/{}/create-schedule", doctor_id).as_str())
            .set_json(&req_data2)
            .to_request();

        // confirm the ScheduleSlot was successfully created
        let resp2 = test::call_service(&app, req2).await;
        eprintln!("Service-Response: {}", resp2.status());
        assert_eq!(resp2.status(), http::StatusCode::OK);
    }
}
