mod setup_env;

#[cfg(test)]
mod doctor_repository_test {
    use super::setup_env::setup_test_env;
    use doctor_service::models::{CreateDoctorDto, PaginationQuery, Specialty};
    use doctor_service::repository;

    async fn setup_integration_test() {
        setup_test_env().await;
        common::db::reset_db_for_test();
        common::db::init_db().await;
    }

    #[tokio::test]
    async fn test_create_and_get_doctor() {
        setup_integration_test().await;

        let payload = CreateDoctorDto {
            name: "Dr. Repository".to_string(),
            specialties: vec![Specialty::GeneralPractice],
            license_num: "LIC-REPO-1".to_string(),
        };

        let insert_result = repository::create_doctor(payload).await.unwrap();
        let doctor_id = insert_result.inserted_id.as_object_id().unwrap().to_hex();

        // New doctors are inactive by default
        let doctor = repository::get_doctor(doctor_id.clone()).await.unwrap();
        assert!(doctor.is_none());

        // Enable doctor
        let enabled = repository::enable_doctor(doctor_id.clone()).await.unwrap();
        assert!(enabled);

        // Now it should be found
        let doctor = repository::get_doctor(doctor_id).await.unwrap().unwrap();
        assert_eq!(doctor.name, "Dr. Repository");
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

        let insert_result = repository::create_doctor(payload).await.unwrap();
        let doctor_id = insert_result.inserted_id.as_object_id().unwrap().to_hex();
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

        let insert_result = repository::create_doctor(payload).await.unwrap();
        let doctor_id = insert_result.inserted_id.as_object_id().unwrap().to_hex();
        repository::enable_doctor(doctor_id.clone()).await.unwrap();

        let deleted = repository::delete_doctor(doctor_id.clone()).await.unwrap();
        assert!(deleted);

        let doctor = repository::get_doctor(doctor_id).await.unwrap();
        assert!(doctor.is_none());
    }
}
