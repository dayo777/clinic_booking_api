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
    use common::models::{CreateScheduleSlot, Specialty};
    use doctor_service::{models::CreateDoctorDto, repository};
    use mongodb::bson::DateTime as BsonDateTime;
    use serde_json::json;
    use std::time::{Duration, SystemTime};
    // use common::utils::generate_id;

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
            doctor_doc.doctor_id
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
            "specialties": ["gp", "derm"],
            "license_num": "LIC-test"
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/doctor")
            .set_json(&req_data)
            .to_request();

        // call & confirm Post success before proceeding
        let create_resp = test::call_service(&app, req).await;
        // assert_eq!(create_resp.status(), http::StatusCode::CREATED);

        // get the DoctorID from the response
        let doctor_id: serde_json::Value = test::read_body_json(create_resp).await;
        let doctor_id = doctor_id
            .as_str()
            .unwrap()
            .trim_start_matches("ObjectId(\"")
            .trim_end_matches("\")");

        // Enable the doctor before creating a schedule
        let enable_req = test::TestRequest::patch()
            .insert_header(("x-api-version", "1"))
            .uri(&format!("/doctor/{}/enable", doctor_id))
            .to_request();
        let _enable_resp = test::call_service(&app, enable_req).await;
        // assert_eq!(enable_resp.status(), http::StatusCode::NO_CONTENT);

        // create a schedule slot for doctorID
        let req_data2 = json!([
            {
                "start_time": "2028-10-01T08:00:00Z",
                "minutes": 30
            },
            {
                "start_time": "2028-10-01T09:00:00Z",
                "minutes": 30
            }
        ]);

        let req2 = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri(format!("/doctor/{}/create-doctor-schedule", doctor_id).as_str())
            .set_json(&req_data2)
            .to_request();

        // confirm the Doctor schedule was successfully created
        let resp2 = test::call_service(&app, req2).await;
        eprintln!("Service-Response: {}", resp2.status());
        assert_eq!(resp2.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_create_doctor_schedule_overlap_returns_bad_request() {
        setup_integration_test().await;

        let app = test::init_service(App::new().configure(doctor_service::doctor_config_v1)).await;
        let doctor_id = repository::create_doctor(CreateDoctorDto {
            name: "Dr. Overlap".to_string(),
            specialties: vec![Specialty::GeneralPractice],
            license_num: "LIC-HANDLER-OVERLAP".to_string(),
        })
        .await
        .unwrap();
        repository::enable_doctor(doctor_id.clone()).await.unwrap();

        let existing_start =
            BsonDateTime::from_system_time(SystemTime::now() + Duration::from_secs(72 * 60 * 60));
        repository::create_doctor_schedule(
            doctor_id.clone(),
            vec![CreateScheduleSlot {
                start_time: existing_start,
                minutes: 30,
            }],
        )
        .await
        .unwrap();

        let overlapping_start = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(
            existing_start.timestamp_millis() + 15 * 60_000,
        )
        .unwrap()
        .to_rfc3339();
        let request = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri(&format!("/doctor/{doctor_id}/create-doctor-schedule"))
            .set_json(json!([{ "start_time": overlapping_start, "minutes": 30 }]))
            .to_request();

        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
        let body: serde_json::Value = test::read_body_json(response).await;
        assert!(body["message"].as_str().unwrap().contains("overlap"));
    }
}
