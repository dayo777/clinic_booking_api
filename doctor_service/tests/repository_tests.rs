mod setup_env;

#[cfg(test)]
mod doctor_repository_test {
    use super::setup_env::setup_test_env;
    use common::models::{CreateScheduleSlot, Specialty};
    use doctor_service::models::{CreateDoctorDto, PaginationQuery};
    use doctor_service::repository;
    use mongodb::bson::DateTime as BsonDateTime;
    use mongodb::bson::doc;
    use std::time::{Duration, SystemTime};

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
        let doctor_id = doctor_id
            .trim_start_matches("ObjectId(\"")
            .trim_end_matches("\")")
            .to_string();

        // Enable doctor cause new Doctors are inactive by default
        let _enabled = repository::enable_doctor(doctor_id.clone()).await.unwrap();

        // Now it should be found
        let doctor = repository::get_doctor(doctor_id).await.unwrap().unwrap();
        assert_eq!(doctor.name, "Dr. Doom");
        assert_eq!(doctor.license_num, "DM-REPO-1");
        assert!(doctor.is_active);
    }

    #[tokio::test]
    async fn test_list_doctors() {
        setup_integration_test().await;

        // Create 3 new Doctors
        let payload1 = CreateDoctorDto {
            name: "Dr. Mira".to_string(),
            specialties: vec![Specialty::Cardiology],
            license_num: "LIC-LIST-1".to_string(),
        };

        let payload2 = CreateDoctorDto {
            name: "Dr. Ma".to_string(),
            specialties: vec![Specialty::Cardiology],
            license_num: "LIC-LIST-1".to_string(),
        };

        let payload3 = CreateDoctorDto {
            name: "Dr. Lira".to_string(),
            specialties: vec![Specialty::Cardiology],
            license_num: "LIC-LIST-1".to_string(),
        };

        let doctor1 = repository::create_doctor(payload1).await.unwrap();
        let doctor2 = repository::create_doctor(payload2).await.unwrap();
        let doctor3 = repository::create_doctor(payload3).await.unwrap();

        // retrieve the DoctorIDs of the Doctors created above
        let doctor1 = doctor1
            .trim_start_matches("ObjectId(\"")
            .trim_end_matches("\")")
            .to_string();

        let doctor2 = doctor2
            .trim_start_matches("ObjectId(\"")
            .trim_end_matches("\")")
            .to_string();

        let doctor3 = doctor3
            .trim_start_matches("ObjectId(\"")
            .trim_end_matches("\")")
            .to_string();

        // Enable the Doctors created above
        repository::enable_doctor(doctor1).await.unwrap();
        repository::enable_doctor(doctor2).await.unwrap();
        repository::enable_doctor(doctor3).await.unwrap();

        let pagination = PaginationQuery {
            page: Some(1),
            limit: Some(10),
        };

        let doctors = repository::list_doctor(pagination).await.unwrap();
        assert!(!doctors.is_empty());
        assert!(doctors.iter().any(|d| d.name == "Dr. Lira"));
        assert!(doctors.iter().any(|d| d.name == "Dr. Mira"));
        assert!(doctors.iter().any(|d| d.name == "Dr. Ma"));
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
        let doctor_id = doctor_id
            .trim_start_matches("ObjectId(\"")
            .trim_end_matches("\")")
            .to_string();
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

        let doctor_id = doctor_id
            .trim_start_matches("ObjectId(\"")
            .trim_end_matches("\")")
            .to_string();

        // Should not exist yet because it's inactive
        assert!(!repository::doctor_exists(doctor_id.clone()).await.unwrap());

        // Enable doctor
        repository::enable_doctor(doctor_id.clone()).await.unwrap();

        // Now it should exist
        assert!(repository::doctor_exists(doctor_id).await.unwrap());
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
        let doctor_id = doctor_id
            .trim_start_matches("ObjectId(\"")
            .trim_end_matches("\")")
            .to_string();

        repository::enable_doctor(doctor_id.clone()).await.unwrap();

        let future_time =
            BsonDateTime::from_system_time(SystemTime::now() + Duration::from_secs(48 * 3600));

        let later_time = BsonDateTime::from_millis(future_time.timestamp_millis() + 3_600_000);
        let slots = vec![
            CreateScheduleSlot {
                start_time: future_time,
                minutes: 20,
            },
            CreateScheduleSlot {
                start_time: later_time,
                minutes: 45,
            },
        ];
        let ids = repository::create_doctor_schedule(doctor_id.clone(), slots)
            .await
            .unwrap();
        assert_eq!(ids.len(), 2);
        assert_ne!(ids[0], ids[1]);
        let schedules = repository::get_doctor_schedule(doctor_id.clone())
            .await
            .unwrap();
        assert_eq!(schedules.len(), 2);
        for (schedule, minutes) in schedules.iter().zip([20, 45]) {
            assert_eq!(schedule.doctor_id, doctor_id);
            assert_eq!(
                schedule.slot.end_time.timestamp_millis()
                    - schedule.slot.start_time.timestamp_millis(),
                minutes * 60_000
            );
        }
        let collection =
            common::db::get_collection::<mongodb::bson::Document>(repository::SCHEDULE_COLLECTION);
        let stored = collection
            .find_one(doc! { "_id": &ids[0] })
            .await
            .unwrap()
            .unwrap();
        assert!(!stored.contains_key("slots"));
        assert_eq!(stored.get_str("doctor_id").unwrap(), doctor_id);
        let active = repository::get_active_doctor_schedule(doctor_id.clone())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(active.len(), 2);
        let claimed =
            repository::update_doctor_schedule_to_false(doctor_id.clone(), active[0].clone())
                .await
                .unwrap();
        assert!(!claimed.is_available);
        assert!(
            repository::update_doctor_schedule_to_false(doctor_id.clone(), active[0].clone())
                .await
                .is_err()
        );
        assert_eq!(
            repository::get_active_doctor_schedule(doctor_id.clone())
                .await
                .unwrap()
                .unwrap()
                .len(),
            1
        );
        // A later batch must remain visible alongside the earlier documents.
        let next_batch_time =
            BsonDateTime::from_millis(future_time.timestamp_millis() + 3 * 3_600_000);
        repository::create_doctor_schedule(
            doctor_id.clone(),
            vec![CreateScheduleSlot {
                start_time: next_batch_time,
                minutes: 30,
            }],
        )
        .await
        .unwrap();
        assert_eq!(
            repository::get_active_doctor_schedule(doctor_id.clone())
                .await
                .unwrap()
                .unwrap()
                .len(),
            2
        );
        assert!(
            repository::create_doctor_schedule(doctor_id.clone(), vec![])
                .await
                .is_err()
        );
        assert!(
            repository::create_doctor_schedule(
                doctor_id.clone(),
                vec![CreateScheduleSlot {
                    start_time: future_time,
                    minutes: 5
                }]
            )
            .await
            .is_err()
        );
        assert_eq!(
            repository::get_doctor_schedule(doctor_id)
                .await
                .unwrap()
                .len(),
            3
        );
    }

    #[tokio::test]
    async fn test_create_schedule_rejects_overlaps_without_partial_inserts() {
        setup_integration_test().await;

        let doctor_id = repository::create_doctor(CreateDoctorDto {
            name: "Dr. Overlap".to_string(),
            specialties: vec![Specialty::GeneralPractice],
            license_num: "LIC-OVERLAP-1".to_string(),
        })
        .await
        .unwrap()
        .trim_start_matches("ObjectId(\"")
        .trim_end_matches("\")")
        .to_string();
        let other_doctor_id = repository::create_doctor(CreateDoctorDto {
            name: "Dr. Different".to_string(),
            specialties: vec![Specialty::GeneralPractice],
            license_num: "LIC-OVERLAP-2".to_string(),
        })
        .await
        .unwrap()
        .trim_start_matches("ObjectId(\"")
        .trim_end_matches("\")")
        .to_string();
        repository::enable_doctor(doctor_id.clone()).await.unwrap();
        repository::enable_doctor(other_doctor_id.clone())
            .await
            .unwrap();

        let start =
            BsonDateTime::from_system_time(SystemTime::now() + Duration::from_secs(72 * 60 * 60));
        let start_at =
            |minutes: i64| BsonDateTime::from_millis(start.timestamp_millis() + minutes * 60_000);
        let existing_id = repository::create_doctor_schedule(
            doctor_id.clone(),
            vec![CreateScheduleSlot {
                start_time: start,
                minutes: 30,
            }],
        )
        .await
        .unwrap()
        .remove(0);

        let schedule_collection =
            common::db::get_collection::<mongodb::bson::Document>(repository::SCHEDULE_COLLECTION);
        schedule_collection
            .update_one(
                doc! { "_id": &existing_id },
                doc! { "$set": { "is_available": false } },
            )
            .await
            .unwrap();

        for (offset, minutes) in [(0, 30), (15, 30), (5, 10)] {
            let result = repository::create_doctor_schedule(
                doctor_id.clone(),
                vec![CreateScheduleSlot {
                    start_time: start_at(offset),
                    minutes,
                }],
            )
            .await;
            assert!(
                result.is_err(),
                "overlap at {offset} minutes should be rejected"
            );
        }

        let other_doctor_ids = repository::create_doctor_schedule(
            other_doctor_id.clone(),
            vec![CreateScheduleSlot {
                start_time: start,
                minutes: 30,
            }],
        )
        .await
        .unwrap();
        assert_eq!(other_doctor_ids.len(), 1);

        let adjacent_ids = repository::create_doctor_schedule(
            doctor_id.clone(),
            vec![CreateScheduleSlot {
                start_time: start_at(30),
                minutes: 30,
            }],
        )
        .await
        .unwrap();
        assert_eq!(adjacent_ids.len(), 1);

        let rejected_batch = repository::create_doctor_schedule(
            doctor_id.clone(),
            vec![
                CreateScheduleSlot {
                    start_time: start_at(180),
                    minutes: 30,
                },
                CreateScheduleSlot {
                    start_time: start_at(10),
                    minutes: 30,
                },
            ],
        )
        .await;
        assert!(rejected_batch.is_err());

        let rejected_internal_overlap = repository::create_doctor_schedule(
            doctor_id.clone(),
            vec![
                CreateScheduleSlot {
                    start_time: start_at(300),
                    minutes: 30,
                },
                CreateScheduleSlot {
                    start_time: start_at(315),
                    minutes: 30,
                },
            ],
        )
        .await;
        assert!(rejected_internal_overlap.is_err());

        let overflow_result = repository::create_doctor_schedule(
            doctor_id.clone(),
            vec![CreateScheduleSlot {
                start_time: BsonDateTime::from_millis(i64::MAX - 60_000),
                minutes: 30,
            }],
        )
        .await;
        assert!(overflow_result.is_err());

        assert_eq!(
            schedule_collection
                .count_documents(doc! { "doctor_id": &doctor_id })
                .await
                .unwrap(),
            2
        );
        assert_eq!(
            schedule_collection
                .count_documents(doc! { "doctor_id": &other_doctor_id })
                .await
                .unwrap(),
            1
        );
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

        let past_time = BsonDateTime::now();

        let schedule = vec![CreateScheduleSlot {
            start_time: past_time,
            minutes: 30,
        }];

        let result = repository::create_doctor_schedule(doctor_id, schedule).await;
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

            let doctor_id = doctor_id
                .trim_start_matches("ObjectId(\"")
                .trim_end_matches("\")")
                .to_string();

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
