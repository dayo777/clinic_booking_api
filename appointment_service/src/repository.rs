// Database operations: insert, find_by_id, find_by_email, etc.

use crate::error::AppointmentScheduleError;
use crate::models::{
    AppointmentDto, AppointmentStatus, AppointmentStatusHistoryDto, CreateAppointmentDto,
};
use crate::utils;
use common::db::get_collection;
use common::models::ScheduleSlot;
use doctor_service::repository::{SCHEDULE_COLLECTION, get_active_doctor_schedule, get_doctor};
use mongodb::bson::DateTime as BsonDateTime;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use tracing::{info, instrument};

// declare collections as represented in MongoDB
static APPOINTMENT_COLLECTION: &str = "appointments_collection";

#[instrument(name = "db_create_appointment", skip(payload))]
pub async fn create_appointment(
    payload: CreateAppointmentDto,
) -> Result<String, AppointmentScheduleError> {
    info!(
        "Creating appointment for doctor_id {} for patient_id {}",
        payload.doctor_id.clone(),
        payload.patient_id.clone()
    );

    // retrieve Doctor details
    let doctor = match get_doctor(payload.doctor_id.clone()).await {
        Ok(doc) => match doc {
            Some(d) => d,
            None => return Err(AppointmentScheduleError::InvalidDoctorId),
        },
        Err(e) => {
            return Err(AppointmentScheduleError::Internal(format!(
                "Unable to retrieve Doctor details: {:?}",
                e
            )));
        }
    };

    // verify the Specialty chosen by the patient matches the Doctor specialty
    if !utils::confirm_specialty_exist_for_doctor(&payload.specialty, &doctor.specialties) {
        return Err(AppointmentScheduleError::UnableToScheduleAppointment);
    }

    // call get_active_doctor_schedule to confirm available slots
    let slots = match get_active_doctor_schedule(payload.doctor_id.clone()).await {
        Ok(Some(slots)) => slots,
        Ok(None) => return Err(AppointmentScheduleError::InvalidSlotId),
        Err(e) => return Err(AppointmentScheduleError::Internal(e.to_string())),
    };

    // change the slot_id availability to false
    let allocated_slot = match utils::verify_slot_id_exist(payload.slot_id.clone().as_str(), &slots)
    {
        Ok(slot) => {
            // change slot_id 'is_available' to false and update the Updated_at field
            let schedule_collection = get_collection::<ScheduleSlot>(SCHEDULE_COLLECTION);
            let filtered_slot_id = doc! { "slot_id": payload.slot_id.clone() };
            let slot_update_availability = doc! {
                "$set":
                    doc! {
                        "is_available": false
                    }
            };
            // 4. update the slot_id is_available to false
            schedule_collection
                .update_one(filtered_slot_id, slot_update_availability)
                .await
                .map_err(AppointmentScheduleError::Database)?;

            slot.clone()
        }
        Err(_) => return Err(AppointmentScheduleError::UnableToScheduleAppointment),
    };

    // 5. persist the Appointment
    let notes = Some(payload.notes.unwrap_or_else(|| String::from("")));
    let slot_id = ObjectId::parse_str(payload.slot_id.as_str())
        .map_err(|_| AppointmentScheduleError::InvalidSlotId)?;
    let doctor_id = ObjectId::parse_str(payload.doctor_id.as_str())
        .map_err(|_| AppointmentScheduleError::InvalidDoctorId)?;
    let patient_id = ObjectId::parse_str(payload.patient_id.as_str())
        .map_err(|_| AppointmentScheduleError::InvalidPatientId)?;
    let start_time = allocated_slot.start_time;
    let end_time = allocated_slot.end_time.unwrap_or_else(|| {
        let thirty_minutes_ms = 30 * 60 * 1000; // 30 mins in milliseconds
        BsonDateTime::from_millis(start_time.timestamp_millis() + thirty_minutes_ms)
    });
    let specialty = payload.specialty;
    let status = AppointmentStatus::Scheduled;
    let status_history = vec![AppointmentStatusHistoryDto {
        status: AppointmentStatus::Scheduled,
        changed_at: BsonDateTime::now(),
        reason: None,
    }];
    let created_at = BsonDateTime::now();
    let updated_at = None;

    let new_appointment = AppointmentDto {
        appointment_id: ObjectId::new(),
        slot_id,
        doctor_id,
        patient_id,
        start_time,
        end_time,
        specialty,
        notes,
        status,
        status_history,
        created_at,
        updated_at,
    };

    let appointment_collection = get_collection::<AppointmentDto>(APPOINTMENT_COLLECTION);

    info!(
        "\
        Creating new appointment for SlotID: {}, with DoctorID: {}, with PatientID: {}",
        payload.slot_id.clone(),
        payload.doctor_id.clone(),
        payload.patient_id.clone()
    );

    let inserted_appointment = appointment_collection
        .insert_one(new_appointment)
        .await
        .map_err(AppointmentScheduleError::from)?;

    // NB: Expose endpoint for Doctor to Confirm an appointment

    Ok(inserted_appointment.inserted_id.to_string())
}

// TODO: recommendations to note
// 1. Make status a Rust enum serialized to a string (#[serde(rename_all = "snake_case")]) so it's readable in the DB and index-friendly.
// 2. Add created_at / updated_at and consider a small embedded status_history array if you need an audit trail of transitions.
// 3. Enforce valid transitions in the service layer (e.g., can't go Completed → Scheduled), and use conditional updates (updateOne with a filter on the expected current status) to avoid race conditions.
