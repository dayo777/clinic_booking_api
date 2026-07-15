// Database operations: insert, find_by_id, find_by_email, etc.
use crate::error::DoctorServiceError;
use crate::models::{
    CreateDoctorDto, DoctorDto, DoctorResponseDto, DoctorSchedule, PaginationQuery, ScheduleSlot,
};
use crate::utils;
use common::db::get_collection;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::options::FindOptions;
// use mongodb::results::InsertOneResult;
use tracing::{debug, info, instrument};

// declare collections as represented in MongoDB here
static DOCTOR_COLLECTION: &str = "doctors_collection";
// use this to store the Doctor's schedule, prevent endpoint always hitting main Doctor table
static SCHEDULE_COLLECTION: &str = "doctor_schedule_collection";

#[allow(clippy::redundant_pattern_matching)]
#[instrument(name = "db_create_doctor", skip(payload))]
pub async fn create_doctor(payload: CreateDoctorDto) -> Result<String, DoctorServiceError> {
    info!(
        "db_create_doctor: Creating doctor with license: {}",
        payload.license_num
    );

    // confirm the Doctor specialties are valid
    let specialties = payload.specialties.clone();
    for specialty in specialties.iter() {
        if let Err(e) = utils::validate_specialty(specialty.to_string()) {
            return Err(DoctorServiceError::Validation(e));
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
        // new doctor object is auto set 'is_active' to false
        // there is a separate endpoint to make is_active
        is_active: false,
    };

    let collection = get_collection::<DoctorDto>(DOCTOR_COLLECTION);
    info!(
        "Inserting new doctor into DB for license_num: {}",
        payload.license_num
    );

    let id = collection.insert_one(new_doctor).await?;
    let id = id.inserted_id.to_string();
    Ok(id)
}

#[instrument(name = "db_get_doctor", skip(doctor_id))]
pub async fn get_doctor(
    doctor_id: String,
) -> Result<Option<DoctorResponseDto>, DoctorServiceError> {
    let collection = get_collection::<DoctorDto>(DOCTOR_COLLECTION);

    let obj_id = match ObjectId::parse_str(&doctor_id) {
        Ok(id) => id,
        Err(e) => {
            debug!("Invalid ObjectId format: {}", e);
            return Ok(None);
        }
    };

    let filter = doc! { "_id": obj_id, "is_active": true };

    info!("Retrieving a single DoctorID");
    let doctor_doc = collection.find_one(filter).await?;

    Ok(doctor_doc.map(|d| DoctorResponseDto {
        id: d.id.unwrap_or_else(ObjectId::new),
        name: d.name,
        specialties: d.specialties,
        license_num: d.license_num,
        is_active: d.is_active,
    }))
}

#[instrument(name = "db_list_doctors", skip(pagination))]
pub async fn list_doctor(
    pagination: PaginationQuery,
) -> Result<Vec<DoctorResponseDto>, DoctorServiceError> {
    let collection = get_collection::<DoctorDto>(DOCTOR_COLLECTION);
    const DEFAULT_LIMIT: u64 = 15;
    const MAX_LIMIT: u64 = 100;

    let limit = pagination
        .limit
        .unwrap_or(DEFAULT_LIMIT)
        .clamp(1, MAX_LIMIT);
    let page = pagination.page.unwrap_or(1).max(1);
    let skip = (page - 1) * limit;

    let find_options = FindOptions::builder()
        .limit(limit as i64)
        .skip(skip)
        .sort(doc! { "_id": 1 })
        .build();

    info!(
        limit = limit,
        page = page,
        skip = skip,
        "Executing MongoDB Find for doctor list"
    );

    let filter = doc! { "is_active": true};
    let mut cursor = collection.find(filter).with_options(find_options).await?;
    let mut doctors = Vec::new();

    while let Some(d) = cursor.try_next().await? {
        doctors.push(DoctorResponseDto {
            id: d.id.unwrap_or_else(ObjectId::new),
            name: d.name,
            specialties: d.specialties,
            license_num: d.license_num,
            is_active: d.is_active,
        });
    }

    Ok(doctors)
}

#[instrument(name = "db_delete_doctor", fields(doctor_id = %doctor_id))]
pub async fn delete_doctor(doctor_id: String) -> Result<bool, DoctorServiceError> {
    let collection = get_collection::<DoctorDto>(DOCTOR_COLLECTION);

    let obj_id = match ObjectId::parse_str(&doctor_id) {
        Ok(id) => id,
        Err(e) => {
            debug!("Invalid ObjectId format: {}", e);
            return Ok(false);
        }
    };

    // find the data to be deleted
    let filter = doc! { "_id": obj_id };
    let doctor_doc = collection.find_one(filter.clone()).await?;

    // TODO: might have to add 'mut' here
    if let Some(_doctor) = doctor_doc {
        let modified_content = doc! {
            "$set": {
                "is_active": false,
                "updated_at": mongodb::bson::DateTime::now(),
            }
        };

        collection.update_one(filter, modified_content).await?;

        info!("Doctor status changed to in_active: {}", doctor_id);
        Ok(true)
    } else {
        debug!("Doctor not found or inis_active: {}", doctor_id);
        Ok(false)
    }
}

#[instrument(name = "db_enable_doctor", fields(doctor_id = %doctor_id))]
pub async fn enable_doctor(doctor_id: String) -> Result<bool, DoctorServiceError> {
    let collection = get_collection::<DoctorDto>(DOCTOR_COLLECTION);

    let obj_id = match ObjectId::parse_str(&doctor_id) {
        Ok(id) => id,
        Err(e) => {
            debug!("Invalid ObjectId format: {}", e);
            return Ok(false);
        }
    };

    let filter = doc! { "_id": obj_id };
    let doctor_doc = collection.find_one(filter.clone()).await?;

    if let Some(_doctor) = doctor_doc {
        let modified_content = doc! {
            "$set": {
                "is_active": true,
                "updated_at": mongodb::bson::DateTime::now(),
            }
        };

        collection.update_one(filter, modified_content).await?;

        info!("Doctor status changed to enabled: {}", doctor_id);
        Ok(true)
    } else {
        debug!("Doctor not found or already enabled: {}", doctor_id);
        Ok(false)
    }
}

#[instrument(name = "db_doctor_exists", skip(doctor_id))]
pub async fn doctor_exists(doctor_id: String) -> bool {
    let collection = get_collection::<DoctorDto>(DOCTOR_COLLECTION);

    let obj_id = match ObjectId::parse_str(&doctor_id) {
        Ok(id) => id,
        Err(_) => return false,
    };

    let filter = doc! { "_id": obj_id, "is_active": true };

    match collection.count_documents(filter).await {
        Ok(count) => count > 0,
        Err(_) => false,
    }
}

/// -----------------------
/// Below this line contains code for Doctor-schedule creation
// endpoint to create Doctor Schedule
pub async fn create_doctor_schedule(
    doctor_id: String,
    slots: Vec<ScheduleSlot>,
) -> Result<Vec<ScheduleSlot>, DoctorServiceError> {
    // call the Doctor exists to confirm the Doctor is Active,
    // then go ahead with Booking the Slot
    // Write code below

    let doctor_id = match ObjectId::parse_str(&doctor_id) {
        Ok(id) => id,
        Err(e) => {
            debug!("Invalid ObjectId format: {}", e);
            return Err(DoctorServiceError::DoctorNotFound);
        }
    };

    // confirm the DoctorID exist & is active
    if !doctor_exists(doctor_id.to_string()).await {
        debug!(
            "Unable to create Doctor schedule, Invalid Doctor-Id: {}",
            doctor_id
        );
        return Err(DoctorServiceError::DoctorNotFound);
    }

    // confirm the DoctorID exist in the Doctor schedule database
    // If doctor exists, then update the existing schedule
    // else skip this statement and create a new schedule
    if check_if_doctor_exists_in_doctor_schedule(doctor_id.to_string()).await {
        debug!("DoctorID already has existing schedule, update the existing schedule",);
        return Err(DoctorServiceError::DoctorAlreadyExistInScheduleDatabase);
    }

    // confirm the Doctor booking is 24 hours ahead
    // write Slot check for each slot
    for slot in &slots {
        if utils::check_date_is_24hr_in_future(&slot.start_time).is_ok() {
            continue;
        } else {
            return Err(DoctorServiceError::ValidationError(
                "Slot start time is not 24 hours ahead".to_string(),
            ));
        }
    }

    let slots_to_return = slots.clone();
    let booking_collection = get_collection::<DoctorSchedule>(SCHEDULE_COLLECTION);
    let new_booking = DoctorSchedule {
        id: None,
        doctor_id,
        slots,
        created_at: mongodb::bson::DateTime::now(),
        updated_at: None,
    };

    info!("Inserting new booking into DB for doctor_id: {}", doctor_id);
    booking_collection.insert_one(new_booking).await?;

    // front-end can change use this to display Specific date/time information for use
    Ok(slots_to_return)
}

// Confirm Doctor ID exist in Doctor Schedule
// 1. Once a DoctorID exists on this endpoint, other schedules should only be added to ID
// 2. this is to prevent the same Doctor from being booked multiple times
// true = Doctor exist in Doctor Schedule, false = Doctor not exist in Doctor Schedule
#[instrument(name = "check_if_doctor_exists_in_doctor_schedule", skip(doctor_id))]
pub async fn check_if_doctor_exists_in_doctor_schedule(doctor_id: String) -> bool {
    let collection = get_collection::<DoctorDto>(SCHEDULE_COLLECTION);

    let doctor_id = match ObjectId::parse_str(&doctor_id) {
        Ok(id) => id,
        Err(_) => return false,
    };

    let filter = doc! { "doctor_id": doctor_id };

    match collection.count_documents(filter).await {
        Ok(count) => count > 0,
        Err(_) => false,
    }
}

#[instrument(name = "db_get_doctor_schedule", skip(doctor_id))]
pub async fn get_doctor_schedule(
    doctor_id: String,
) -> Result<Option<DoctorSchedule>, DoctorServiceError> {
    let _ = doctor_id;
    todo!("This is a placeholder for get_doctor_schedule")
}
pub async fn list_doctor_schedule() {
    todo!("list the available active schedules for a Particular Doctor")
}

// to retrieve active Doctor schedules for appointment
// this should be called from the Appointment-service
#[instrument(name = "db_get_active_doctor_schedule", skip(doctor_id))]
pub async fn get_active_doctor_schedule(
    doctor_id: String,
) -> Result<Option<Vec<ScheduleSlot>>, DoctorServiceError> {
    let collection = get_collection::<DoctorSchedule>(SCHEDULE_COLLECTION);

    let obj_id = match ObjectId::parse_str(&doctor_id) {
        Ok(id) => id,
        Err(e) => {
            debug!("Invalid ObjectId format: {}", e);
            return Ok(None);
        }
    };

    let filter = doc! { "doctor_id": obj_id };

    info!("Retrieving active Doctor Schedule for doctor_id");
    let schedule = collection.find_one(filter).await?;

    Ok(schedule.map(|s| {
        s.slots
            .into_iter()
            .filter(|slot| slot.is_available == Some(true))
            .collect()
    }))
}

// TODO: You have to ensure the slot exists, and is on True before you can change to false
// // this should be called from the appointments-service after an appointment is made
// // changes the slot "availability" to false; meaning the slot is booked
// #[instrument(name = "db_update_doctor_schedule", skip(doctor_id))]
// pub async fn update_doctor_schedule(doctor_id: String) {
//     let collection = get_collection::<DoctorSchedule>(SCHEDULE_COLLECTION);
//
//     let obj_id = match ObjectId::parse_str(&doctor_id) {
//         Ok(id) => id,
//         Err(e) => {
//             debug!("Invalid ObjectId format: {}", e);
//             return Ok(None);
//         }
//     };
//
// }
