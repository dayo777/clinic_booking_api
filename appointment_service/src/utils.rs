use crate::error::AppointmentScheduleError;
use crate::models::{AppointmentDto, AppointmentResponseDto};
use common::models::ScheduleSlot;
use mongodb::bson::DateTime as BsonDateTime;
use validator::ValidationError;

const CANCELLATION_NOTICE_MS: i64 = 24 * 60 * 60 * 1000;

// verify that the SlotID exist in the Doctor ScheduleSlot, and is_available is set to true
pub(crate) fn verify_slot_id_exist<'a>(
    slot_id: &str,
    available_slots: &'a [ScheduleSlot],
) -> Result<&'a ScheduleSlot, AppointmentScheduleError> {
    // let obj_id = ObjectId::parse_str(slot_id).map_err(|_| AppointmentScheduleError::InvalidSlotId)?;

    // let slot = available_slots
    //     .iter()
    //     .find(|slot: &&ScheduleSlot| slot.slot_id.as_ref().is_some_and(|id: &String| id == slot_id))
    //     .ok_or(AppointmentScheduleError::AppointmentNotFound)?;

    let slot = available_slots
        .iter()
        .find(|s| s.slot_id.as_str() == slot_id)
        .ok_or(AppointmentScheduleError::AppointmentNotFound)?;

    // confirm the slot is_available
    if !slot.is_available {
        return Err(AppointmentScheduleError::UnableToScheduleAppointment);
    }

    Ok(slot)
}

// confirms that the patient's requested specialty matches one of the doctor's available specialties.
pub(crate) fn confirm_specialty_exist_for_doctor<T: PartialEq>(a: &T, b: &[T]) -> bool {
    b.iter().any(|item| item == a)
}

impl From<AppointmentDto> for AppointmentResponseDto {
    fn from(app: AppointmentDto) -> Self {
        Self {
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
        }
    }
}

pub(crate) fn validate_cancellation_window(
    appointment_start_time: &BsonDateTime,
) -> Result<(), AppointmentScheduleError> {
    let time_until_appointment_ms = appointment_start_time
        .timestamp_millis()
        .saturating_sub(BsonDateTime::now().timestamp_millis());

    if time_until_appointment_ms < CANCELLATION_NOTICE_MS {
        return Err(AppointmentScheduleError::Validation(ValidationError::new(
            "Appointments must be canceled at least 24 hours before their start time",
        )));
    }

    Ok(())
}
