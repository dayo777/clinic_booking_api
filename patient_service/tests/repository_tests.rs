// mod setup_env;
//
// #[cfg(test)]
// mod repository_integration_tests {
//     use super::setup_env::setup_test_env;
//     use actix_web::{
//         App,
//         http::{StatusCode, header::ContentType},
//         test,
//     };
//     use common::{db::init_db, logging::init_tracing};
//     use patient_service::patient_config_v1;
//     use serde_json::{Value, json};
//     use std::sync::Once;
//
//     static TRACING_INIT: Once = Once::new();
//
//     async fn init_test_app() -> actix_web::App<
//         impl actix_web::dev::ServiceFactory<
//             actix_web::dev::ServiceRequest,
//             Config = (),
//             Response = actix_web::dev::ServiceResponse,
//             Error = actix_web::Error,
//             InitError = (),
//         >,
//     > {
//         let env = setup_test_env().await;
//         let _ = (&env.mongodb_uri, &env.mongodb_database, &env.otlp_endpoint);
//         init_db().await;
//
//         TRACING_INIT.call_once(|| {
//             init_tracing("patient_service_integration_test");
//         });
//
//         App::new().service(actix_web::web::scope("/api/v1").configure(patient_config_v1))
//     }
//
//     #[actix_web::test]
//     async fn spin_up_mongodb_and_jaeger_and_create_get_patient() {
//         let app = test::init_service(init_test_app().await).await;
//
//         let payload = json!({
//             "name": "Integration Test Patient",
//             "dob": "1999-11-17",
//             "gender": "Female",
//             "contact_info": {
//                 "phone": "+1-202-555-0192",
//                 "email": "integration.patient@clinic.test",
//                 "address": "100 Integration Avenue",
//                 "emergency_contact_name": "Jane Doe",
//                 "emergency_contact_phone": "+1-202-555-0147"
//             }
//         });
//
//         let create_req = test::TestRequest::post()
//             .uri("/api/v1/patient")
//             .insert_header(ContentType::json())
//             .set_json(payload)
//             .to_request();
//         let create_resp = test::call_service(&app, create_req).await;
//         assert_eq!(create_resp.status(), StatusCode::CREATED);
//
//         let list_req = test::TestRequest::get().uri("/api/v1/patient").to_request();
//         let list_resp = test::call_service(&app, list_req).await;
//         assert_eq!(list_resp.status(), StatusCode::OK);
//
//         let body: Value = test::read_body_json(list_resp).await;
//         let first = body
//             .as_array()
//             .and_then(|arr| arr.first())
//             .expect("expected at least one patient in list response");
//
//         let patient_id = first
//             .get("id")
//             .and_then(|id| id.get("$oid"))
//             .and_then(Value::as_str)
//             .expect("expected BSON object id in response payload");
//
//         let get_req = test::TestRequest::get()
//             .uri(&format!("/api/v1/patient/{patient_id}"))
//             .to_request();
//         let get_resp = test::call_service(&app, get_req).await;
//         assert_eq!(get_resp.status(), StatusCode::OK);
//     }
// }
