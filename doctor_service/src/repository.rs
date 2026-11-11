// Database operations: insert, find_by_id, find_by_email, etc.
use crate::error::DoctorServiceError;
use crate::models::{
    CreateDoctorDto, DoctorDto, DoctorResponseDto, DoctorSchedule, PaginationQuery,
};
use crate::utils;
use crate::utils::collect_doctors;
use common::db::get_collection;
use common::models::{CreateScheduleSlot, ScheduleSlot};
use common::utils::{generate_id, validate_specialty};
use futures::TryStreamExt;
use mongodb::bson::DateTime as BsonDateTime;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use tracing::{debug, info, instrument};
use validator::Validate;

// declare collections as represented in MongoDB here
static DOCTOR_COLLECTION: &str = "doctors_collection";
// use this to store the Doctor's schedule, prevent endpoint always hitting main Doctor table
pub static SCHEDULE_COLLECTION: &str = "doctor_schedule_collection";
static ID_LENGTH: u8 = 24; // this is the length of the generated ID

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
        if let Err(e) = validate_specialty(specialty) {
            return Err(DoctorServiceError::Validation(e));
        }
    }

    let name = payload.name.clone();
    let license_num = payload.license_num.clone();

    let doctor_id = generate_id("doc", ID_LENGTH);

    let new_doctor = DoctorDto {
        doctor_id,
        name,
        specialties,
        license_num,
        schedule: None,
        created_at: mongodb::bson::DateTime::now(),
        updated_at: None,
        // the new doctor object is auto-set 'is_active' to false.
        // there is a separate endpoint to enable is_active to true
        is_active: false,
    };

    let collection = get_collection::<DoctorDto>(DOCTOR_COLLECTION);
    info!(
        "Inserting new doctor into DB for license_num: {}",
        payload.license_num
    );

    let id = collection.insert_one(new_doctor).await?;
    let id = id
        .inserted_id
        .as_str()
        .ok_or(DoctorServiceError::Internal(
            "Failed to extract inserted ID".to_string(),
        ))?
        .to_string();
    Ok(id)
}

// rem this only retrieves doctors with 'is_active' = true
#[instrument(name = "db_get_doctor", skip(doctor_id))]
pub async fn get_doctor(
    doctor_id: String,
) -> Result<Option<DoctorResponseDto>, DoctorServiceError> {
    let collection = get_collection::<DoctorDto>(DOCTOR_COLLECTION);

    // confirm the DoctorID exist & is active
    let filter = doc! { "_id": &doctor_id, "is_active": true };

    info!("Retrieving a single DoctorID");
    let doctor_doc = collection.find_one(filter).await?;

    if let Some(d) = doctor_doc {
        Ok(Some(DoctorResponseDto {
            doctor_id: d.doctor_id,
            name: d.name,
            specialties: d.specialties,
            license_num: d.license_num,
            is_active: d.is_active,
        }))
    } else {
        Ok(None)
    }
}

// returns only active Doctors
#[instrument(name = "db_list_active_doctors", skip(pagination))]
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
        "Executing MongoDB Find for active doctor list"
    );

    let filter = doc! { "is_active": true};
    let cursor = collection.find(filter).with_options(find_options).await?;

    collect_doctors(cursor).await
}

// returns only inactive Doctors e.g., is_active: false
#[instrument(name = "db_list_inactive_doctors", skip(pagination))]
pub async fn list_inactive_doctor(
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
        "Executing MongoDB Find for inactive doctor list"
    );

    let filter = doc! { "is_active": false};
    let cursor = collection.find(filter).with_options(find_options).await?;

    collect_doctors(cursor).await
}

#[instrument(name = "db_delete_doctor", fields(doctor_id = %doctor_id))]
pub async fn delete_doctor(doctor_id: String) -> Result<bool, DoctorServiceError> {
    let collection = get_collection::<DoctorDto>(DOCTOR_COLLECTION);

    // find the data to be deleted
    let filter = doc! { "_id": &doctor_id };
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

    let filter = doc! { "_id": &doctor_id };
    let doc_update = doc! { "$set":  doc! {
        "is_active": true,
        "updated_at": mongodb::bson::DateTime::now(),
    }};
    let result = collection
        .find_one_and_update(filter, doc_update)
        .return_document(mongodb::options::ReturnDocument::After)
        .await
        .map_err(|e| DoctorServiceError::Internal(e.to_string()))?;

    if let Some(_d) = result {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[instrument(name = "db_disable_doctor", fields(doctor_id = %doctor_id))]
pub async fn disable_doctor(doctor_id: String) -> Result<bool, DoctorServiceError> {
    let collection = get_collection::<DoctorDto>(DOCTOR_COLLECTION);
    let filter = doc! { "_id": &doctor_id };
    let doc_update = doc! { "$set":  doc! {
        "is_active": false,
        "updated_at": mongodb::bson::DateTime::now(),
    }};
    let result = collection
        .find_one_and_update(filter, doc_update)
        .return_document(mongodb::options::ReturnDocument::After)
        .await
        .map_err(|e| DoctorServiceError::Internal(e.to_string()))?;

    if let Some(_d) = result {
        Ok(true)
    } else {
        Ok(false)
    }
}

// TODO: this doesn't work as intended. Fix this.
#[instrument(name = "db_doctor_exists")]
pub async fn doctor_exists(doctor_id: String) -> Result<bool, DoctorServiceError> {
    let filter = doc! { "_id": doctor_id };
    let collection = get_collection::<DoctorDto>(DOCTOR_COLLECTION);

    let result = collection.find_one(filter).await?;

    match result {
        Some(doctor) if doctor.is_active => Ok(true),
        None => Err(DoctorServiceError::DoctorNotFound),
        _ => Ok(false),
    }
}

// Create one document per slot. The request remains a batch of slots.
pub async fn create_doctor_schedule(
    doctor_id: String,
    slots: Vec<CreateScheduleSlot>,
) -> Result<Vec<String>, DoctorServiceError> {
    if slots.is_empty() {
        return Err(DoctorServiceError::ValidationError(
            "At least one slot is required".into(),
        ));
    }
    if !doctor_exists(doctor_id.clone()).await? {
        return Err(DoctorServiceError::DoctorNotFound);
    }
    // Validate the entire batch before inserting any documents.
    for slot in &slots {
        slot.validate()
            .map_err(|e| DoctorServiceError::ValidationError(e.to_string()))?;
        utils::check_date_is_24hr_in_future(&slot.start_time)?;
    }
    let end_times = utils::check_schedule_slots_available(&doctor_id, &slots).await?;
    let schedules: Vec<DoctorSchedule> = slots
        .into_iter()
        .zip(end_times)
        .map(|(s, end_time)| DoctorSchedule {
            doctor_id: doctor_id.clone(),
            slot: ScheduleSlot {
                slot_id: generate_id("slot", ID_LENGTH),
                start_time: s.start_time,
                end_time,
                is_available: true,
                created_at: BsonDateTime::now(),
                updated_at: None,
            },
        })
        .collect();
    let ids = schedules.iter().map(|s| s.slot.slot_id.clone()).collect();
    get_collection::<DoctorSchedule>(SCHEDULE_COLLECTION)
        .insert_many(schedules)
        .await?;
    Ok(ids)
}

/// Retrieve all schedules for a doctor, including booked schedules.
#[instrument(name = "db_get_doctor_schedule", skip(doctor_id))]
pub async fn get_doctor_schedule(
    doctor_id: String,
) -> Result<Vec<DoctorSchedule>, DoctorServiceError> {
    Ok(get_collection::<DoctorSchedule>(SCHEDULE_COLLECTION)
        .find(doc! { "doctor_id": doctor_id })
        .sort(doc! { "start_time": 1, "_id": 1 })
        .await?
        .try_collect()
        .await?)
}

#[instrument(name = "db_get_active_doctor_schedule", skip(doctor_id))]
pub async fn get_active_doctor_schedule(
    doctor_id: String,
) -> Result<Option<Vec<ScheduleSlot>>, DoctorServiceError> {
    let schedules: Vec<DoctorSchedule> = get_collection::<DoctorSchedule>(SCHEDULE_COLLECTION)
        .find(doc! { "doctor_id": doctor_id, "is_available": true,
        "start_time": { "$gt": BsonDateTime::now() } })
        .sort(doc! { "start_time": 1, "_id": 1 })
        .await?
        .try_collect()
        .await?;
    if schedules.is_empty() {
        return Ok(None);
    }
    Ok(Some(schedules.into_iter().map(|s| s.slot).collect()))
}

/// Claim exactly one available slot atomically, preventing double booking.
#[instrument(name = "db_update_doctor_schedule", skip(doctor_id, schedule_slot))]
pub async fn update_doctor_schedule_to_false(
    doctor_id: String,
    mut schedule_slot: ScheduleSlot,
) -> Result<ScheduleSlot, DoctorServiceError> {
    let updated_at = BsonDateTime::now();
    let result = get_collection::<DoctorSchedule>(SCHEDULE_COLLECTION)
        .update_one(
            doc! { "doctor_id": doctor_id, "_id": &schedule_slot.slot_id,
            "is_available": true, "start_time": { "$gt": updated_at } },
            doc! { "$set": { "is_available": false, "updated_at": updated_at } },
        )
        .await?;
    if result.modified_count != 1 {
        return Err(DoctorServiceError::UnableToRetrieveDoctorSchedule);
    }
    schedule_slot.is_available = false;
    schedule_slot.updated_at = Some(updated_at);
    Ok(schedule_slot)
}
