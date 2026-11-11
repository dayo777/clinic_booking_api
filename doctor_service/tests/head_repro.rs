mod setup_env;

use actix_web::{App, guard, http::StatusCode, test, web};
use mongodb::{
    Client,
    bson::{doc, oid::ObjectId},
};
use serde_json::json;

#[actix_web::test]
async fn doctor_head_checks_current_legacy_and_error_cases() {
    setup_env::setup_test_env().await;
    common::db::init_db().await;
    let app = test::init_service(
        App::new().service(
            web::scope("/api")
                .guard(guard::Header("x-api-version", "1"))
                .configure(doctor_service::doctor_config_v1),
        ),
    )
    .await;

    let create = test::TestRequest::post()
        .uri("/api/doctor")
        .insert_header(("x-api-version", "1"))
        .set_json(json!({"name": "Dr. Head", "specialties": ["gp"], "license_num": "LIC-HEAD"}))
        .to_request();
    let response = test::call_service(&app, create).await;
    assert_eq!(response.status(), StatusCode::CREATED);
    let id: String = test::read_body_json(response).await;
    let path = format!("/api/doctor/{id}");

    let inactive_head = test::TestRequest::with_uri(&path)
        .method(actix_web::http::Method::HEAD)
        .insert_header(("x-api-version", "1"))
        .to_request();
    assert_eq!(
        test::call_service(&app, inactive_head).await.status(),
        StatusCode::NOT_FOUND
    );

    let enable = test::TestRequest::patch()
        .uri(&format!("{path}/enable"))
        .insert_header(("x-api-version", "1"))
        .to_request();
    assert_eq!(
        test::call_service(&app, enable).await.status(),
        StatusCode::NO_CONTENT
    );

    let get = test::TestRequest::get()
        .uri(&path)
        .insert_header(("x-api-version", "1"))
        .to_request();
    assert_eq!(test::call_service(&app, get).await.status(), StatusCode::OK);
    let head = test::TestRequest::with_uri(&path)
        .method(actix_web::http::Method::HEAD)
        .insert_header(("x-api-version", "1"))
        .to_request();
    let response = test::call_service(&app, head).await;
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let collection = common::db::get_collection::<mongodb::bson::Document>("doctors_collection");
    let legacy_id = ObjectId::new();
    collection
        .insert_one(doc! { "_id": legacy_id, "is_active": true })
        .await
        .unwrap();
    let legacy_head = test::TestRequest::with_uri(&format!("/api/doctor/{legacy_id}"))
        .method(actix_web::http::Method::HEAD)
        .insert_header(("x-api-version", "1"))
        .to_request();
    assert_eq!(
        test::call_service(&app, legacy_head).await.status(),
        StatusCode::NO_CONTENT
    );

    let inactive_legacy_id = ObjectId::new();
    collection
        .insert_one(doc! { "_id": inactive_legacy_id, "is_active": false })
        .await
        .unwrap();
    let inactive_legacy_head =
        test::TestRequest::with_uri(&format!("/api/doctor/{inactive_legacy_id}"))
            .method(actix_web::http::Method::HEAD)
            .insert_header(("x-api-version", "1"))
            .to_request();
    assert_eq!(
        test::call_service(&app, inactive_legacy_head)
            .await
            .status(),
        StatusCode::NOT_FOUND
    );

    let missing_id = ObjectId::new();
    let missing_head = test::TestRequest::with_uri(&format!("/api/doctor/{missing_id}"))
        .method(actix_web::http::Method::HEAD)
        .insert_header(("x-api-version", "1"))
        .to_request();
    assert_eq!(
        test::call_service(&app, missing_head).await.status(),
        StatusCode::NOT_FOUND
    );

    let precedence_id = ObjectId::new();
    collection
        .insert_many([
            doc! { "_id": precedence_id, "is_active": true },
            doc! { "_id": precedence_id.to_hex(), "is_active": false },
        ])
        .await
        .unwrap();
    let precedence_head = test::TestRequest::with_uri(&format!("/api/doctor/{precedence_id}"))
        .method(actix_web::http::Method::HEAD)
        .insert_header(("x-api-version", "1"))
        .to_request();
    assert_eq!(
        test::call_service(&app, precedence_head).await.status(),
        StatusCode::NOT_FOUND
    );

    let test_env = setup_env::setup_test_env().await;
    let client_options = mongodb::options::ClientOptions::parse(&test_env.mongodb_uri)
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();
    let database = client.database(&test_env.mongodb_database);
    database
        .collection::<mongodb::bson::Document>("doctor_lookup_error_source")
        .insert_one(doc! { "_id": "source", "value": 1 })
        .await
        .unwrap();
    collection.drop().await.unwrap();
    database
        .run_command(doc! {
            "create": "doctors_collection",
            "viewOn": "doctor_lookup_error_source",
            "pipeline": [
                { "$project": { "_id": 1, "is_active": { "$divide": [1, 0] } } }
            ],
        })
        .await
        .unwrap();
    let database_error_head = test::TestRequest::with_uri(&path)
        .method(actix_web::http::Method::HEAD)
        .insert_header(("x-api-version", "1"))
        .to_request();
    assert_eq!(
        test::call_service(&app, database_error_head).await.status(),
        StatusCode::INTERNAL_SERVER_ERROR
    );
}
