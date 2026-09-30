// Database operations: insert, find_by_id, find_by_email, etc.

use crate::error::AppointmentScheduleError;
use crate::models::{
    AppointmentDto, AppointmentResponseDto, AppointmentStatus, AppointmentStatusHistoryDto,
    CreateAppointmentDto, PaginationQuery,
};
use crate::utils;
use common::models::ScheduleSlot;
use common::{db::get_collection, utils::generate_id};
use doctor_service::repository::{SCHEDULE_COLLECTION, get_active_doctor_schedule, get_doctor};
use futures::stream::TryStreamExt;
use mongodb::bson::DateTime as BsonDateTime;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use tracing::{info, instrument};

// declare collections as represented in MongoDB
static APPOINTMENT_COLLECTION: &str = "appointments_collection";
static ID_LENGTH: u8 = 24; // this is the length of the generated ID

#[instrument(name = "db_create_appointment", skip(payload))]
pub async fn create_appointment(
    payload: CreateAppointmentDto,
) -> Result<String, AppointmentScheduleError> {
    info!(
        "Creating appointment for doctor_id {} for patient_id {}",
        payload.doctor_id, payload.patient_id
    );

    // retrieve Doctor details
    let doctor = match get_doctor(payload.doctor_id.clone()).await {
        Ok(Some(d)) => d,
        Ok(None) => return Err(AppointmentScheduleError::InvalidDoctorId),
        Err(e) => {
            return Err(AppointmentScheduleError::Internal(format!(
                "Unable to retrieve Doctor details: {:?}",
                e
            )));
        }
    };

    // verify specialty matches doctor
    if !utils::confirm_specialty_exist_for_doctor(&payload.specialty, &doctor.specialties) {
        return Err(AppointmentScheduleError::UnableToScheduleAppointment);
    }

    // retrieve active doctor schedule
    let slots = match get_active_doctor_schedule(payload.doctor_id.clone()).await {
        Ok(Some(slots)) => {
            info!(
                "Retrieved {} active slots for doctor_id: {}",
                slots.len(),
                payload.doctor_id
            );
            slots
        }
        Ok(None) => {
            info!(
                "No active schedule found for doctor_id: {}",
                payload.doctor_id
            );
            return Err(AppointmentScheduleError::InvalidSlotId);
        }
        Err(e) => return Err(AppointmentScheduleError::Internal(e.to_string())),
    };

    // verify slot exists and is available
    let allocated_slot = match utils::verify_slot_id_exist(payload.slot_id.as_str(), &slots) {
        Ok(slot) => {
            let schedule_collection = get_collection::<ScheduleSlot>(SCHEDULE_COLLECTION);

            // filter by _id not slot_id
            let filter = doc! {
                "doctor_id": &payload.doctor_id,
                "slots._id": &payload.slot_id,
            };

            let update = doc! {
                "$set": {
                    "slots.$.is_available": false   // ← $ positional operator updates matched slot
                }
            };

            schedule_collection
                .update_one(filter, update)
                .await
                .map_err(AppointmentScheduleError::Database)?;

            slot.clone()
        }
        Err(_) => return Err(AppointmentScheduleError::UnableToScheduleAppointment),
    };

    // build appointment — use String IDs directly, no ObjectId parsing
    let notes = Some(payload.notes.unwrap_or_default());
    let start_time = allocated_slot.start_time;
    let end_time = allocated_slot.end_time.unwrap_or_else(|| {
        BsonDateTime::from_millis(start_time.timestamp_millis() + (30 * 60 * 1000))
    });

    let appointment_id = generate_id("app", ID_LENGTH);

    let new_appointment = AppointmentDto {
        appointment_id,
        slot_id: payload.slot_id.clone(),       // ← String directly
        doctor_id: payload.doctor_id.clone(),   // ← String directly
        patient_id: payload.patient_id.clone(), // ← String directly
        start_time,
        end_time,
        specialty: payload.specialty,
        notes,
        status: AppointmentStatus::Scheduled,
        status_history: vec![AppointmentStatusHistoryDto {
            status: AppointmentStatus::Scheduled,
            changed_at: BsonDateTime::now(),
            reason: String::from("Initial appointment schedule"),
        }],
        created_at: BsonDateTime::now(),
        updated_at: None,
    };

    let appointment_collection = get_collection::<AppointmentDto>(APPOINTMENT_COLLECTION);

    info!(
        "Creating new appointment for SlotID: {}, DoctorID: {}, PatientID: {}",
        payload.slot_id, payload.doctor_id, payload.patient_id
    );

    let inserted = appointment_collection
        .insert_one(new_appointment)
        .await
        .map_err(AppointmentScheduleError::from)?;

    Ok(inserted
        .inserted_id
        .as_object_id()
        .map(|id| id.to_hex())
        .unwrap_or_else(|| inserted.inserted_id.to_string()))
}

// retrieve a Single appointment by appointment-id
#[instrument(name = "db_get_appointment", skip(appointment_id))]
pub async fn get_appointment(
    appointment_id: String,
) -> Result<Option<AppointmentResponseDto>, AppointmentScheduleError> {
    let collection = get_collection::<AppointmentDto>(APPOINTMENT_COLLECTION);
    let filter = doc! {"_id": &appointment_id};

    // retrieving Appointment by ID
    info!("Retrieving Appointment by ID: {}", appointment_id.clone());
    let result = collection
        .find_one(filter)
        .await
        .map_err(|_| AppointmentScheduleError::AppointmentNotFound)?;

    if let Some(appointment) = result {
        Ok(Some(AppointmentResponseDto {
            appointment_id: appointment.appointment_id,
            slot_id: appointment.slot_id,
            doctor_id: appointment.doctor_id,
            patient_id: appointment.patient_id,
            start_time: appointment.start_time,
            end_time: appointment.end_time,
            specialty: appointment.specialty,
            status: appointment.status,
            status_history: appointment.status_history,
            notes: appointment.notes,
        }))
    } else {
        Ok(None)
    }
}

// get all appointments for a patient
#[instrument(name = "db_get_all_patient_appointments", skip(patient_id))]
pub async fn get_all_patient_appointments(
    patient_id: String,
    pagination_query: PaginationQuery,
) -> Result<Vec<AppointmentResponseDto>, AppointmentScheduleError> {
    const DEFAULT_LIMIT: u64 = 15;
    const MAX_LIMIT: u64 = 100;

    let limit = pagination_query
        .limit
        .unwrap_or(DEFAULT_LIMIT)
        .clamp(1, MAX_LIMIT);

    let page = pagination_query.page.unwrap_or(1).max(1);
    let skip = (page - 1) * limit;

    let find_options = FindOptions::builder()
        .limit(limit as i64)
        .skip(skip)
        .sort(doc! { "_id": 1 })
        .build();

    let filter = doc! {"patient_id": &patient_id};

    info!(
        "Retrieving all appointments for patient_id: {}",
        patient_id.clone()
    );
    let collection = get_collection::<AppointmentDto>(APPOINTMENT_COLLECTION);
    let mut cursor = collection.find(filter).with_options(find_options).await?;
    let mut patient_appointments = Vec::new();

    while let Some(app) = cursor.try_next().await? {
        patient_appointments.push(AppointmentResponseDto {
            appointment_id: app.appointment_id,
            slot_id: app.slot_id,
            doctor_id: app.doctor_id,
            patient_id: app.patient_id,
            start_time: app.start_time,
            end_time: app.end_time,
            specialty: app.specialty,
            status: app.status,
            status_history: app.status_history,
            notes: app.notes,
        });
    }

    Ok(patient_appointments)
}

// get all appointments for a doctor
#[instrument(name = "db_get_all_doctor_appointments", skip(doctor_id))]
pub async fn get_all_doctor_appointments(
    doctor_id: String,
    pagination_query: PaginationQuery,
) -> Result<Vec<AppointmentResponseDto>, AppointmentScheduleError> {
    const DEFAULT_LIMIT: u64 = 15;
    const MAX_LIMIT: u64 = 100;

    let limit = pagination_query
        .limit
        .unwrap_or(DEFAULT_LIMIT)
        .clamp(1, MAX_LIMIT);

    let page = pagination_query.page.unwrap_or(1).max(1);
    let skip = (page - 1) * limit;

    let find_options = FindOptions::builder()
        .limit(limit as i64)
        .skip(skip)
        .sort(doc! { "_id": 1 })
        .build();

    let filter = doc! {"doctor_id": &doctor_id};

    info!(
        "Retrieving all appointments for doctor_id: {}",
        doctor_id.clone()
    );
    let collection = get_collection::<AppointmentDto>(APPOINTMENT_COLLECTION);
    let mut cursor = collection.find(filter).with_options(find_options).await?;
    let mut doctor_appointments = Vec::new();

    while let Some(app) = cursor.try_next().await? {
        doctor_appointments.push(AppointmentResponseDto {
            appointment_id: app.appointment_id,
            slot_id: app.slot_id,
            doctor_id: app.doctor_id,
            patient_id: app.patient_id,
            start_time: app.start_time,
            end_time: app.end_time,
            specialty: app.specialty,
            status: app.status,
            status_history: app.status_history,
            notes: app.notes,
        });
    }

    Ok(doctor_appointments)
}

// TODO: should only be able to confirm appointment if appointment is in `Scheduled` status
// change AppointmentStatus to `confirmed` & update status-history
#[instrument(name = "db_confirm_appointment", fields(appointment_id = ?appointment_id))]
pub async fn confirm_appointment(
    appointment_id: String,
    reason: String,
) -> Result<(), AppointmentScheduleError> {
    let filter = doc! {"_id": &appointment_id};

    let reason = if reason.is_empty() {
        String::from("Appointment confirmed")
    } else {
        reason
    };

    let appointment_status_update = AppointmentStatusHistoryDto {
        status: AppointmentStatus::Confirmed,
        changed_at: mongodb::bson::DateTime::now(),
        reason,
    };

    let status_bson = mongodb::bson::to_bson(&AppointmentStatus::Confirmed).map_err(|_| {
        AppointmentScheduleError::Internal(String::from(
            "Unable to convert Appointment status to BSON",
        ))
    })?;
    let status_history_bson = mongodb::bson::to_bson(&appointment_status_update).map_err(|_| {
        AppointmentScheduleError::Internal(String::from(
            "Unable to convert Appointment history status to BSON",
        ))
    })?;

    let modified_content = doc! {
        "$set": {
            "status": status_bson,
            "updated_at": mongodb::bson::DateTime::now(),
        },
        "$push": {
            "status_history": status_history_bson,
        }
    };

    let collection = get_collection::<AppointmentDto>(APPOINTMENT_COLLECTION);
    info!("Updating appointment status to confirmed");
    collection.update_one(filter, modified_content).await?;

    Ok(())
}

// TODO: should only be able to cancel appointment if appointment is in `Scheduled` OR `Confirmed` status
// change AppointmentStatus to `canceled` & update status-history
#[instrument(name = "db_cancel_appointment", fields(appointment_id = ?appointment_id))]
pub async fn cancel_appointment(
    appointment_id: String,
    reason: String,
) -> Result<(), AppointmentScheduleError> {
    let filter = doc! {"_id": &appointment_id};

    let reason = if reason.is_empty() {
        String::from("Appointment Cancelled")
    } else {
        reason
    };

    let appointment_status_update = AppointmentStatusHistoryDto {
        status: AppointmentStatus::Canceled,
        changed_at: mongodb::bson::DateTime::now(),
        reason,
    };

    let status_bson = mongodb::bson::to_bson(&AppointmentStatus::Canceled).map_err(|_| {
        AppointmentScheduleError::Internal(String::from(
            "Unable to convert Appointment status to BSON",
        ))
    })?;
    let status_history_bson = mongodb::bson::to_bson(&appointment_status_update).map_err(|_| {
        AppointmentScheduleError::Internal(String::from(
            "Unable to convert Appointment history status to BSON",
        ))
    })?;

    let modified_content = doc! {
        "$set": {
            "status": status_bson,
            "updated_at": mongodb::bson::DateTime::now(),
        },
        "$push": {
            "status_history": status_history_bson,
        }
    };

    let collection = get_collection::<AppointmentDto>(APPOINTMENT_COLLECTION);
    info!("Updating appointment status to canceled");
    collection.update_one(filter, modified_content).await?;

    Ok(())
}

// TODO: should only be able to complete appointment if appointment is in `Confirmed` status
// change AppointmentStatus to `Completed` & update status-history
#[instrument(name = "db_complete_appointment", fields(appointment_id = ?appointment_id))]
pub async fn complete_appointment(
    appointment_id: String,
    reason: String,
) -> Result<(), AppointmentScheduleError> {
    let filter = doc! {"_id": &appointment_id};

    let reason = if reason.is_empty() {
        String::from("Appointment Completed successfully.")
    } else {
        reason
    };

    let appointment_status_update = AppointmentStatusHistoryDto {
        status: AppointmentStatus::Completed,
        changed_at: mongodb::bson::DateTime::now(),
        reason,
    };

    let status_bson = mongodb::bson::to_bson(&AppointmentStatus::Completed).map_err(|_| {
        AppointmentScheduleError::Internal(String::from(
            "Unable to convert Appointment status to BSON",
        ))
    })?;
    let status_history_bson = mongodb::bson::to_bson(&appointment_status_update).map_err(|_| {
        AppointmentScheduleError::Internal(String::from(
            "Unable to convert Appointment history status to BSON",
        ))
    })?;

    let modified_content = doc! {
        "$set": {
            "status": status_bson,
            "updated_at": mongodb::bson::DateTime::now(),
        },
        "$push": {
            "status_history": status_history_bson,
        }
    };

    let collection = get_collection::<AppointmentDto>(APPOINTMENT_COLLECTION);
    info!("Updating appointment status to completed");
    collection.update_one(filter, modified_content).await?;

    Ok(())
}

// TODO: should only be able to complete appointment if appointment is in `Confirmed` status
// change AppointmentStatus to `NoShow` & update status-history
#[instrument(name = "db_no_show_appointment", fields(appointment_id = ?appointment_id))]
pub async fn no_show_appointment(
    appointment_id: String,
    reason: String,
) -> Result<(), AppointmentScheduleError> {
    let filter = doc! {"_id": &appointment_id};

    let reason = if reason.is_empty() {
        String::from("Patient did not show up for this appointment session.")
    } else {
        reason
    };

    let appointment_status_update = AppointmentStatusHistoryDto {
        status: AppointmentStatus::NoShow,
        changed_at: mongodb::bson::DateTime::now(),
        reason,
    };

    let status_bson = mongodb::bson::to_bson(&AppointmentStatus::NoShow).map_err(|_| {
        AppointmentScheduleError::Internal(String::from(
            "Unable to convert Appointment status to BSON",
        ))
    })?;
    let status_history_bson = mongodb::bson::to_bson(&appointment_status_update).map_err(|_| {
        AppointmentScheduleError::Internal(String::from(
            "Unable to convert Appointment history status to BSON",
        ))
    })?;

    let modified_content = doc! {
        "$set": {
            "status": status_bson,
            "updated_at": mongodb::bson::DateTime::now(),
        },
        "$push": {
            "status_history": status_history_bson,
        }
    };

    let collection = get_collection::<AppointmentDto>(APPOINTMENT_COLLECTION);
    info!("Updating appointment status to NoShow");
    collection.update_one(filter, modified_content).await?;

    Ok(())
}

// TODO: recommendations to note
// 1. Make status a Rust enum serialized to a string (#[serde(rename_all = "snake_case")]) so it's readable in the DB and index-friendly.
// 2. Add created_at / updated_at and consider a small embedded status_history array if you need an audit trail of transitions.
// 3. Enforce valid transitions in the service layer (e.g., can't go Completed → Scheduled), and use conditional updates (updateOne with a filter on the expected current status) to avoid race conditions.

// TODO: Other handlers to add
// 8. Get an appointment status history #[get("/{id}/history")]
