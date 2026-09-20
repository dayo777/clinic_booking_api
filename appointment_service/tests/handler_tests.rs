// test for all Appointment handlers goes here
mod setup_env;

#[cfg(test)]
mod appointment_service_handler_test {
    use super::setup_env::setup_test_env;
    use actix_http;
    use actix_web::{
        App,
        http::{self, header::ContentType},
        test,
    };
    use common::models::{CreateScheduleSlot, Specialty};
    use mongodb::bson::DateTime as BsonDateTime;
    use serde_json::json;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn acquire_test_lock() -> MutexGuard<'static, ()> {
        TEST_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    async fn setup_integration_test() {
        // call the setup_env file for each test
        let test_env = setup_test_env().await;
        // setup_env currently emits `mongodb:://...`; normalize it before the
        // common database initializer parses the URI.
        let mongodb_uri = test_env
            .mongodb_uri
            .replacen("mongodb:://", "mongodb://", 1);
        unsafe {
            std::env::set_var("MONGODB_URI", mongodb_uri);
            std::env::set_var("MONGODB_DATABASE", &test_env.mongodb_database);
        }
        common::db::reset_db_for_test();
        common::db::init_db().await;
    }

    // seed an active Doctor with an active schedule,
    // returns (doctor_id, slot_id) needed to create an appointment
    async fn seed_doctor_with_slot() -> (String, String) {
        let doctor_payload = doctor_service::models::CreateDoctorDto {
            name: String::from("Dr. Stephen Strange"),
            specialties: vec![Specialty::GeneralPractice, Specialty::Neurosurgery],
            license_num: String::from("LIC-12345"),
        };

        let doctor_id = doctor_service::repository::create_doctor(doctor_payload)
            .await
            .expect("Unable to create doctor");

        // doctors are created inactive, enable before scheduling
        let enabled = doctor_service::repository::enable_doctor(doctor_id.clone())
            .await
            .expect("Unable to enable doctor");
        assert!(enabled, "Doctor was not enabled");

        // slot must start more than 24 hours in the future
        let start_time =
            BsonDateTime::from_millis(BsonDateTime::now().timestamp_millis() + 48 * 60 * 60 * 1000);
        let end_time = Some(BsonDateTime::from_millis(
            start_time.timestamp_millis() + 30 * 60 * 1000,
        ));

        let slots = vec![CreateScheduleSlot {
            start_time,
            end_time,
        }];

        doctor_service::repository::create_doctor_schedule(doctor_id.clone(), slots)
            .await
            .expect("Unable to create doctor schedule");

        let active_slots =
            doctor_service::repository::get_active_doctor_schedule(doctor_id.clone())
                .await
                .expect("Unable to retrieve doctor schedule")
                .expect("No active schedule found for doctor");

        let slot_id = active_slots
            .first()
            .expect("No active slots available")
            .slot_id
            .clone();

        (doctor_id, slot_id)
    }

    // create an appointment through the endpoint and return its ID
    async fn create_appointment_via_endpoint(
        app: &impl actix_web::dev::Service<
            actix_http::Request,
            Response = actix_web::dev::ServiceResponse,
            Error = actix_web::Error,
        >,
        doctor_id: &str,
        slot_id: &str,
        patient_id: &str,
    ) -> String {
        let req_data = json!({
            "slot_id": slot_id,
            "doctor_id": doctor_id,
            "patient_id": patient_id,
            "specialty": "gp",
            "notes": "Recurring headaches"
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/appointment")
            .set_json(req_data)
            .to_request();

        let resp = test::call_service(app, req).await;
        assert_eq!(resp.status(), http::StatusCode::CREATED);

        // create returns an empty body, so retrieve the ID via the patient listing
        let list_req = test::TestRequest::get()
            .uri(&format!("/appointment/patient/{}", patient_id))
            .to_request();

        let list_resp = test::call_service(app, list_req).await;
        assert_eq!(list_resp.status(), http::StatusCode::OK);

        let appointments: Vec<serde_json::Value> = test::read_body_json(list_resp).await;
        let appointment = appointments
            .iter()
            .find(|a| a["slot_id"] == slot_id)
            .expect("Appointment not found in patient list");

        appointment["_id"]
            .as_str()
            .expect("Unable to retrieve appointment ID")
            .to_string()
    }

    #[actix_web::test]
    async fn test_appointment_post_create_success() {
        let _test_guard = acquire_test_lock();
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(appointment_service::appointment_config_v1))
                .await;

        let (doctor_id, slot_id) = seed_doctor_with_slot().await;

        let req_data = json!({
            "slot_id": slot_id,
            "doctor_id": doctor_id,
            "patient_id": "pat_integration_test_001",
            "specialty": "gp",
            "notes": "First consultation"
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/appointment")
            .set_json(req_data)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn test_appointment_post_create_invalid_data() {
        let _test_guard = acquire_test_lock();
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(appointment_service::appointment_config_v1))
                .await;

        // all IDs too short, min 5 chars
        let req_data = json!({
            "slot_id": "s1",
            "doctor_id": "d1",
            "patient_id": "p1",
            "specialty": "gp",
            "notes": "Invalid payload"
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/appointment")
            .set_json(req_data)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_appointment_post_create_missing_fields() {
        let _test_guard = acquire_test_lock();
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(appointment_service::appointment_config_v1))
                .await;

        let req_data = json!({
            "slot_id": "slot_00000000000000000001",
            // missing doctor_id, patient_id, specialty
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/appointment")
            .set_json(req_data)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_appointment_post_create_nonexistent_doctor() {
        let _test_guard = acquire_test_lock();
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(appointment_service::appointment_config_v1))
                .await;

        let req_data = json!({
            "slot_id": "slot_00000000000000000001",
            "doctor_id": "doc_00000000000000000001",
            "patient_id": "pat_00000000000000000001",
            "specialty": "gp",
            "notes": "Doctor does not exist"
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/appointment")
            .set_json(req_data)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn test_appointment_post_create_wrong_specialty() {
        let _test_guard = acquire_test_lock();
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(appointment_service::appointment_config_v1))
                .await;

        let (doctor_id, slot_id) = seed_doctor_with_slot().await;

        // doctor only has GeneralPractice & Neurosurgery specialties
        let req_data = json!({
            "slot_id": slot_id,
            "doctor_id": doctor_id,
            "patient_id": "pat_wrong_specialty_001",
            "specialty": "cardio",
            "notes": "Specialty mismatch"
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .insert_header(("x-api-version", "1"))
            .uri("/appointment")
            .set_json(req_data)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn test_get_appointment_success() {
        let _test_guard = acquire_test_lock();
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(appointment_service::appointment_config_v1))
                .await;

        let (doctor_id, slot_id) = seed_doctor_with_slot().await;
        let patient_id = "pat_get_appointment_001";

        let appointment_id =
            create_appointment_via_endpoint(&app, &doctor_id, &slot_id, patient_id).await;

        let get_req = test::TestRequest::get()
            .uri(&format!("/appointment/{}", appointment_id))
            .to_request();

        let get_resp = test::call_service(&app, get_req).await;
        assert_eq!(get_resp.status(), http::StatusCode::OK);

        let appointment: serde_json::Value = test::read_body_json(get_resp).await;
        assert_eq!(appointment["_id"], appointment_id);
        assert_eq!(appointment["doctor_id"], doctor_id);
        assert_eq!(appointment["patient_id"], patient_id);
        assert_eq!(appointment["slot_id"], slot_id);
        assert_eq!(appointment["status"], "Scheduled");
    }

    #[actix_web::test]
    async fn test_get_appointment_not_found() {
        let _test_guard = acquire_test_lock();
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(appointment_service::appointment_config_v1))
                .await;

        let non_existent_id = "app_00000000000000000001";

        let req = test::TestRequest::get()
            .uri(&format!("/appointment/{}", non_existent_id))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_all_patient_appointments_success() {
        let _test_guard = acquire_test_lock();
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(appointment_service::appointment_config_v1))
                .await;

        let (doctor_id, slot_id) = seed_doctor_with_slot().await;
        let patient_id = "pat_list_appointments_001";

        create_appointment_via_endpoint(&app, &doctor_id, &slot_id, patient_id).await;

        let req = test::TestRequest::get()
            .uri(&format!("/appointment/patient/{}", patient_id))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let appointments: Vec<serde_json::Value> = test::read_body_json(resp).await;
        assert_eq!(appointments.len(), 1);
        assert_eq!(appointments[0]["patient_id"], patient_id);
    }

    #[actix_web::test]
    async fn test_get_all_patient_appointments_empty() {
        let _test_guard = acquire_test_lock();
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(appointment_service::appointment_config_v1))
                .await;

        let req = test::TestRequest::get()
            .uri("/appointment/patient/pat_no_appointments_001")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let appointments: Vec<serde_json::Value> = test::read_body_json(resp).await;
        assert!(appointments.is_empty());
    }

    #[actix_web::test]
    async fn test_get_all_patient_appointments_with_pagination() {
        let _test_guard = acquire_test_lock();
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(appointment_service::appointment_config_v1))
                .await;

        let req = test::TestRequest::get()
            .uri("/appointment/patient/pat_paginated_001?page=1&limit=5")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_all_doctor_appointments_success() {
        let _test_guard = acquire_test_lock();
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(appointment_service::appointment_config_v1))
                .await;

        let (doctor_id, slot_id) = seed_doctor_with_slot().await;
        let patient_id = "pat_doctor_list_001";

        create_appointment_via_endpoint(&app, &doctor_id, &slot_id, patient_id).await;

        let req = test::TestRequest::get()
            .uri(&format!("/appointment/doctor/{}", doctor_id))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let appointments: Vec<serde_json::Value> = test::read_body_json(resp).await;
        assert_eq!(appointments.len(), 1);
        assert_eq!(appointments[0]["doctor_id"], doctor_id);
    }

    #[actix_web::test]
    async fn test_get_all_doctor_appointments_empty() {
        let _test_guard = acquire_test_lock();
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(appointment_service::appointment_config_v1))
                .await;

        let req = test::TestRequest::get()
            .uri("/appointment/doctor/doc_no_appointments_001?page=1&limit=10")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let appointments: Vec<serde_json::Value> = test::read_body_json(resp).await;
        assert!(appointments.is_empty());
    }

    #[actix_web::test]
    async fn test_confirm_appointment_success() {
        let _test_guard = acquire_test_lock();
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(appointment_service::appointment_config_v1))
                .await;

        let (doctor_id, slot_id) = seed_doctor_with_slot().await;
        let patient_id = "pat_confirm_appointment_001";

        let appointment_id =
            create_appointment_via_endpoint(&app, &doctor_id, &slot_id, patient_id).await;

        let confirm_req = test::TestRequest::patch()
            .insert_header(ContentType::json())
            .uri(&format!("/appointment/{}/confirm", appointment_id))
            .set_json(json!({ "reason": "Doctor confirmed availability" }))
            .to_request();

        let confirm_resp = test::call_service(&app, confirm_req).await;
        assert_eq!(confirm_resp.status(), http::StatusCode::OK);

        // verify the status changed to Confirmed
        let get_req = test::TestRequest::get()
            .uri(&format!("/appointment/{}", appointment_id))
            .to_request();

        let get_resp = test::call_service(&app, get_req).await;
        assert_eq!(get_resp.status(), http::StatusCode::OK);

        let appointment: serde_json::Value = test::read_body_json(get_resp).await;
        assert_eq!(appointment["status"], "Confirmed");

        // status history should contain the initial Scheduled entry + Confirmed entry
        let history = appointment["status_history"]
            .as_array()
            .expect("status_history should be an array");
        assert_eq!(history.len(), 2);
        assert_eq!(history[1]["status"], "Confirmed");
        assert_eq!(history[1]["reason"], "Doctor confirmed availability");
    }

    #[actix_web::test]
    async fn test_cancel_appointment_success() {
        let _test_guard = acquire_test_lock();
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(appointment_service::appointment_config_v1))
                .await;

        let (doctor_id, slot_id) = seed_doctor_with_slot().await;
        let patient_id = "pat_cancel_appointment_001";

        let appointment_id =
            create_appointment_via_endpoint(&app, &doctor_id, &slot_id, patient_id).await;

        let cancel_req = test::TestRequest::patch()
            .insert_header(ContentType::json())
            .uri(&format!("/appointment/{}/cancel", appointment_id))
            .set_json(json!({ "reason": "Patient requested cancellation" }))
            .to_request();

        let cancel_resp = test::call_service(&app, cancel_req).await;
        assert_eq!(cancel_resp.status(), http::StatusCode::OK);

        // verify the status changed to Canceled
        let get_req = test::TestRequest::get()
            .uri(&format!("/appointment/{}", appointment_id))
            .to_request();

        let get_resp = test::call_service(&app, get_req).await;
        assert_eq!(get_resp.status(), http::StatusCode::OK);

        let appointment: serde_json::Value = test::read_body_json(get_resp).await;
        assert_eq!(appointment["status"], "Canceled");

        let history = appointment["status_history"]
            .as_array()
            .expect("status_history should be an array");
        assert_eq!(history.len(), 2);
        assert_eq!(history[1]["status"], "Canceled");
        assert_eq!(history[1]["reason"], "Patient requested cancellation");
    }

    #[actix_web::test]
    async fn test_confirm_appointment_empty_reason_uses_default() {
        let _test_guard = acquire_test_lock();
        setup_integration_test().await;

        let app =
            test::init_service(App::new().configure(appointment_service::appointment_config_v1))
                .await;

        let (doctor_id, slot_id) = seed_doctor_with_slot().await;
        let patient_id = "pat_confirm_default_reason_001";

        let appointment_id =
            create_appointment_via_endpoint(&app, &doctor_id, &slot_id, patient_id).await;

        // empty reason should fall back to the repository default
        let confirm_req = test::TestRequest::patch()
            .insert_header(ContentType::json())
            .uri(&format!("/appointment/{}/confirm", appointment_id))
            .set_json(json!({ "reason": "" }))
            .to_request();

        let confirm_resp = test::call_service(&app, confirm_req).await;
        assert_eq!(confirm_resp.status(), http::StatusCode::OK);

        let get_req = test::TestRequest::get()
            .uri(&format!("/appointment/{}", appointment_id))
            .to_request();

        let get_resp = test::call_service(&app, get_req).await;
        let appointment: serde_json::Value = test::read_body_json(get_resp).await;
        assert_eq!(appointment["status"], "Confirmed");

        let history = appointment["status_history"]
            .as_array()
            .expect("status_history should be an array");
        assert_eq!(history[1]["reason"], "Appointment confirmed");
    }
}
