// Database operations: insert, find_by_id, find_by_email, etc.
use crate::models::{CreateDoctorDto, DoctorDto};
use crate::utils;
use common::db::get_collection;
use mongodb::error::Error as MongodbError;
use mongodb::results::InsertOneResult;
use tracing::{info, instrument};

// declare collections as represented in MongoDB here
static DOCTOR_COLLECTION: &str = "doctors_collection"; //used `collection` instead of table

#[allow(clippy::redundant_pattern_matching)]
#[instrument(name = "db_create_doctor", skip(payload))]
pub async fn create_doctor(payload: CreateDoctorDto) -> Result<InsertOneResult, MongodbError> {
    info!(
        "db_create_doctor: Creating doctor with license: {}",
        payload.license_num
    );

    // confirm the Doctor specialties are valid
    let specialties = payload.specialties.clone();
    for specialty in specialties.iter() {
        if let Err(_) = utils::validate_specialty(specialty.to_string()) {
            return Err(MongodbError::from(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid specialty",
            )));
        }
    }

    let name = payload.name.clone();
    let license_num = payload.license_num.clone();

    let new_doctor = DoctorDto {
        id: None,
        name,
        specialties,
        license_num,
        schedule: None,
        created_at: mongodb::bson::DateTime::now(),
        updated_at: None,
        active: false,
    };

    let collection = get_collection::<DoctorDto>(DOCTOR_COLLECTION);
    info!(
        "Inserting new doctor into DB for license_num: {}",
        payload.license_num
    );
    collection.insert_one(new_doctor).await
}
