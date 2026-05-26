mod setup_env;

#[cfg(test)]
mod patient_repository_test {
    use super::setup_env::setup_test_env;
    use chrono::NaiveDate;
    use mongodb::bson::doc;
    use patient_service::models::{
        ContactInfo, CreatePatientDto, Gender, PaginationQuery, UpdateContactInfoDto,
        UpdateInsuranceDto, UpdateMedicalAlertsDto,
    };
    use patient_service::repository;

    async fn setup_integration_test() {
        setup_test_env().await;
        common::db::reset_db_for_test();
        common::db::init_db().await;
        cleanup_db().await;
    }

    async fn cleanup_db() {
        let collection =
            common::db::get_collection::<mongodb::bson::Document>("patients_collection");
        let _ = collection.delete_many(doc! {}).await;
        let deleted_collection =
            common::db::get_collection::<mongodb::bson::Document>("patient_deleted");
        let _ = deleted_collection.delete_many(doc! {}).await;
    }

    #[tokio::test]
    async fn test_create_and_get_patient() {
        setup_integration_test().await;

        let contact = ContactInfo {
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
            address: "123 Test St".to_string(),
            emergency_contact_name: Some("Emergency".to_string()),
            emergency_contact_phone: Some("0987654321".to_string()),
        };

        let payload = CreatePatientDto {
            name: "John Doe".to_string(),
            dob: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            gender: Gender::Male,
            contact_info: contact,
        };

        let insert_result = repository::create_patient(payload).await.unwrap();
        let patient_id = insert_result.inserted_id.as_object_id().unwrap().to_hex();

        let patient = repository::get_single_patient(patient_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(patient.name, "John Doe");
        assert_eq!(patient.gender, Gender::Male);
    }

    #[tokio::test]
    async fn test_list_patients() {
        setup_integration_test().await;

        let contact = ContactInfo {
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
            address: "123 Test St".to_string(),
            emergency_contact_name: None,
            emergency_contact_phone: None,
        };

        let payload = CreatePatientDto {
            name: "Patient List".to_string(),
            dob: NaiveDate::from_ymd_opt(1985, 5, 5).unwrap(),
            gender: Gender::Female,
            contact_info: contact,
        };

        repository::create_patient(payload).await.unwrap();

        let pagination = PaginationQuery {
            page: Some(1),
            limit: Some(10),
        };

        let patients = repository::list_patient(pagination).await.unwrap();
        assert!(!patients.is_empty());
        assert!(patients.iter().any(|p| p.name == "Patient List"));
    }

    #[tokio::test]
    async fn test_delete_patient() {
        setup_integration_test().await;

        let contact = ContactInfo {
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
            address: "123 Test St".to_string(),
            emergency_contact_name: None,
            emergency_contact_phone: None,
        };

        let payload = CreatePatientDto {
            name: "To Be Deleted".to_string(),
            dob: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            gender: Gender::Other,
            contact_info: contact,
        };

        let insert_result = repository::create_patient(payload).await.unwrap();
        let patient_id = insert_result.inserted_id.as_object_id().unwrap().to_hex();

        let deleted = repository::delete_patient(patient_id.clone())
            .await
            .unwrap();
        assert!(deleted);

        let patient = repository::get_single_patient(patient_id).await.unwrap();
        assert!(patient.is_none());
    }

    #[tokio::test]
    async fn test_patient_exists() {
        setup_integration_test().await;

        let contact = ContactInfo {
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
            address: "123 Test St".to_string(),
            emergency_contact_name: None,
            emergency_contact_phone: None,
        };

        let payload = CreatePatientDto {
            name: "Exists Check".to_string(),
            dob: NaiveDate::from_ymd_opt(1995, 10, 10).unwrap(),
            gender: Gender::Male,
            contact_info: contact,
        };

        let insert_result = repository::create_patient(payload).await.unwrap();
        let patient_id = insert_result.inserted_id.as_object_id().unwrap().to_hex();

        assert!(repository::patient_exists(patient_id).await);
        assert!(!repository::patient_exists(mongodb::bson::oid::ObjectId::new().to_hex()).await);
    }

    #[tokio::test]
    async fn test_update_insurance() {
        setup_integration_test().await;

        let contact = ContactInfo {
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
            address: "123 Test St".to_string(),
            emergency_contact_name: None,
            emergency_contact_phone: None,
        };

        let payload = CreatePatientDto {
            name: "Insurance Test".to_string(),
            dob: NaiveDate::from_ymd_opt(1980, 1, 1).unwrap(),
            gender: Gender::Female,
            contact_info: contact,
        };

        let insert_result = repository::create_patient(payload).await.unwrap();
        let patient_id = insert_result.inserted_id.as_object_id().unwrap().to_hex();

        let insurance_update = UpdateInsuranceDto {
            provider_name: Some("HealthPlus".to_string()),
            policy_number: Some("HP12345".to_string()),
            group_number: Some("G99".to_string()),
            primary_holder_name: Some("John Doe".to_string()),
        };

        let updated = repository::update_patient_insurance(patient_id.clone(), insurance_update)
            .await
            .unwrap();
        assert!(updated);

        let patient = repository::get_single_patient(patient_id)
            .await
            .unwrap()
            .unwrap();
        let insurance = patient.insurance.unwrap();
        assert_eq!(insurance.provider_name, "HealthPlus");
        assert_eq!(insurance.policy_number, "HP12345");
    }

    #[tokio::test]
    async fn test_update_medical_alerts() {
        setup_integration_test().await;

        let contact = ContactInfo {
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
            address: "123 Test St".to_string(),
            emergency_contact_name: None,
            emergency_contact_phone: None,
        };

        let payload = CreatePatientDto {
            name: "Alerts Test".to_string(),
            dob: NaiveDate::from_ymd_opt(1980, 1, 1).unwrap(),
            gender: Gender::Male,
            contact_info: contact,
        };

        let insert_result = repository::create_patient(payload).await.unwrap();
        let patient_id = insert_result.inserted_id.as_object_id().unwrap().to_hex();

        let alerts_update = UpdateMedicalAlertsDto {
            blood_type: Some("A+".to_string()),
            allergies: Some(vec!["Peanuts".to_string()]),
            chronic_conditions: Some(vec!["Asthma".to_string()]),
            current_medications: Some(vec!["Inhaler".to_string()]),
        };

        let updated = repository::update_patient_medical_alerts(patient_id.clone(), alerts_update)
            .await
            .unwrap();
        assert!(updated);

        let patient = repository::get_single_patient(patient_id)
            .await
            .unwrap()
            .unwrap();
        let alerts = patient.medical_alerts.unwrap();
        assert_eq!(alerts.blood_type, "A+");
        assert!(alerts.allergies.contains(&"Peanuts".to_string()));
    }

    #[tokio::test]
    async fn test_update_contact_info() {
        setup_integration_test().await;

        let contact = ContactInfo {
            phone: "1234567890".to_string(),
            email: "test@example.com".to_string(),
            address: "123 Test St".to_string(),
            emergency_contact_name: None,
            emergency_contact_phone: None,
        };

        let payload = CreatePatientDto {
            name: "Contact Update Test".to_string(),
            dob: NaiveDate::from_ymd_opt(1980, 1, 1).unwrap(),
            gender: Gender::Female,
            contact_info: contact,
        };

        let insert_result = repository::create_patient(payload).await.unwrap();
        let patient_id = insert_result.inserted_id.as_object_id().unwrap().to_hex();

        let contact_update = UpdateContactInfoDto {
            phone: Some("0000000000".to_string()),
            email: Some("newemail@example.com".to_string()),
            address: None,
            emergency_contact_name: None,
            emergency_contact_phone: None,
        };

        let updated = repository::update_patient_contact_info(patient_id.clone(), contact_update)
            .await
            .unwrap();
        assert!(updated);

        let patient = repository::get_single_patient(patient_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(patient.contact.phone, "0000000000");
        assert_eq!(patient.contact.email, "newemail@example.com");
    }
}
