mod setup_env;

#[cfg(test)]
mod doctor_repository_test {
    use super::setup_env::setup_test_env;
    use doctor_service::models::{CreateDoctorDto, PaginationQuery, Specialty};
    use doctor_service::repository;
    use mongodb::bson::doc;

    async fn setup_integration_test() {
        setup_test_env().await;
        common::db::reset_db_for_test();
        common::db::init_db().await;
        cleanup_db().await;
    }

    async fn cleanup_db() {
        let collection =
            common::db::get_collection::<mongodb::bson::Document>("doctors_collection");
        let _ = collection.delete_many(doc! {}).await;
        let schedule_collection =
            common::db::get_collection::<mongodb::bson::Document>("doctor_schedule_collection");
        let _ = schedule_collection.delete_many(doc! {}).await;
    }

    #[tokio::test]
    async fn test_create_and_get_doctor() {
        setup_integration_test().await;

        let payload = CreateDoctorDto {
            name: "Dr. Doom".to_string(),
            specialties: vec![Specialty::GeneralPractice, Specialty::Dermatology],
            license_num: "DM-REPO-1".to_string(),
        };

        let doctor_id = repository::create_doctor(payload).await.unwrap();

        // New doctors are inactive by default
        let doctor = repository::get_doctor(doctor_id.clone()).await.unwrap();
        assert!(doctor.is_none());

        // Enable doctor
        let enabled = repository::enable_doctor(doctor_id.clone()).await.unwrap();
        assert!(enabled);

        // Now it should be found
        let doctor = repository::get_doctor(doctor_id).await.unwrap().unwrap();
        assert_eq!(doctor.name, "Dr. Doom");
        assert_eq!(doctor.license_num, "DM-REPO-1");
        assert!(doctor.is_active);
    }

    #[tokio::test]
    async fn test_list_doctors() {
        setup_integration_test().await;

        let payload = CreateDoctorDto {
            name: "Dr. List".to_string(),
            specialties: vec![Specialty::Cardiology],
            license_num: "LIC-LIST-1".to_string(),
        };

        let doctor_id = repository::create_doctor(payload).await.unwrap();
        repository::enable_doctor(doctor_id).await.unwrap();

        let pagination = PaginationQuery {
            page: Some(1),
            limit: Some(10),
        };

        let doctors = repository::list_doctor(pagination).await.unwrap();
        assert!(!doctors.is_empty());
        assert!(doctors.iter().any(|d| d.name == "Dr. List"));
    }

    #[tokio::test]
    async fn test_delete_doctor() {
        setup_integration_test().await;

        let payload = CreateDoctorDto {
            name: "Dr. Delete".to_string(),
            specialties: vec![Specialty::Dermatology],
            license_num: "LIC-DEL-1".to_string(),
        };

        let doctor_id = repository::create_doctor(payload).await.unwrap();
        repository::enable_doctor(doctor_id.clone()).await.unwrap();

        let deleted = repository::delete_doctor(doctor_id.clone()).await.unwrap();
        assert!(deleted);

        let doctor = repository::get_doctor(doctor_id).await.unwrap();
        assert!(doctor.is_none());
    }

    #[tokio::test]
    async fn test_create_doctor_invalid_specialty() {
        setup_integration_test().await;

        let payload = CreateDoctorDto {
            name: "Dr. Invalid".to_string(),
            specialties: vec![Specialty::Other("Magic".to_string())],
            license_num: "LIC-INV-1".to_string(),
        };

        let result = repository::create_doctor(payload).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_doctor_exists() {
        setup_integration_test().await;

        let payload = CreateDoctorDto {
            name: "Dr. Exists".to_string(),
            specialties: vec![Specialty::GeneralPractice],
            license_num: "LIC-EX-1".to_string(),
        };

        let doctor_id = repository::create_doctor(payload).await.unwrap();

        // Should not exist yet because it's inactive
        assert!(!repository::doctor_exists(doctor_id.clone()).await);

        repository::enable_doctor(doctor_id.clone()).await.unwrap();

        // Now it should exist
        assert!(repository::doctor_exists(doctor_id).await);

        // Random ID should not exist
        assert!(!repository::doctor_exists(mongodb::bson::oid::ObjectId::new().to_hex()).await);
    }

    #[tokio::test]
    async fn test_create_doctor_schedule() {
        setup_integration_test().await;

        let payload = CreateDoctorDto {
            name: "Dr. Schedule".to_string(),
            specialties: vec![Specialty::GeneralPractice],
            license_num: "LIC-SCH-1".to_string(),
        };

        let doctor_id = repository::create_doctor(payload).await.unwrap();
        repository::enable_doctor(doctor_id.clone()).await.unwrap();

        use doctor_service::models::ScheduleSlot;
        use mongodb::bson::DateTime as BsonDateTime;
        use std::time::{Duration, SystemTime};

        let future_time =
            BsonDateTime::from_system_time(SystemTime::now() + Duration::from_secs(48 * 3600));

        let slots = vec![ScheduleSlot {
            start_time: future_time,
            end_time: None,
            is_available: Some(true),
        }];

        let result = repository::create_doctor_schedule(doctor_id, slots.clone()).await;
        assert!(result.is_ok());
        let returned_slots = result.unwrap();
        assert_eq!(returned_slots.len(), 1);
        assert_eq!(returned_slots[0].is_available, Some(true));
    }

    #[tokio::test]
    async fn test_create_doctor_schedule_invalid_time() {
        setup_integration_test().await;

        let payload = CreateDoctorDto {
            name: "Dr. Time".to_string(),
            specialties: vec![Specialty::GeneralPractice],
            license_num: "LIC-TIME-1".to_string(),
        };

        let doctor_id = repository::create_doctor(payload).await.unwrap();
        repository::enable_doctor(doctor_id.clone()).await.unwrap();

        use doctor_service::models::ScheduleSlot;
        use mongodb::bson::DateTime as BsonDateTime;

        // Past time
        let past_time = BsonDateTime::now();

        let slots = vec![ScheduleSlot {
            start_time: past_time,
            end_time: None,
            is_available: Some(true),
        }];

        let result = repository::create_doctor_schedule(doctor_id, slots).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_doctors_pagination_limits() {
        setup_integration_test().await;

        // Create 2 doctors
        for i in 1..=2 {
            let payload = CreateDoctorDto {
                name: format!("Dr. Pagination {}", i),
                specialties: vec![Specialty::Cardiology],
                license_num: format!("LIC-PAG-{}", i),
            };
            let doctor_id = repository::create_doctor(payload).await.unwrap();
            repository::enable_doctor(doctor_id).await.unwrap();
        }

        // Test with limit 1
        let pagination = PaginationQuery {
            page: Some(1),
            limit: Some(1),
        };
        let doctors = repository::list_doctor(pagination).await.unwrap();
        assert_eq!(doctors.len(), 1);

        // Test with page 2
        let pagination = PaginationQuery {
            page: Some(2),
            limit: Some(1),
        };
        let doctors = repository::list_doctor(pagination).await.unwrap();
        assert_eq!(doctors.len(), 1);

        // Test with high limit (clamped)
        let pagination = PaginationQuery {
            page: Some(1),
            limit: Some(1000),
        };
        let doctors = repository::list_doctor(pagination).await.unwrap();
        assert_eq!(doctors.len(), 2);
    }

    #[tokio::test]
    async fn test_get_doctor_invalid_id() {
        setup_integration_test().await;

        let result = repository::get_doctor("invalid-id".to_string())
            .await
            .unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_inactive_doctor() {
        setup_integration_test().await;

        let payload = CreateDoctorDto {
            name: "Dr. Inactive".to_string(),
            specialties: vec![Specialty::GeneralPractice],
            license_num: "LIC-INACT-1".to_string(),
        };

        let doctor_id = repository::create_doctor(payload).await.unwrap();

        // New doctors are inactive by default, get_doctor should return None
        let doctor = repository::get_doctor(doctor_id).await.unwrap();
        assert!(doctor.is_none());
    }
}
