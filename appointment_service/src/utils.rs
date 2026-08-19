use crate::error::AppointmentScheduleError;
use common::models::ScheduleSlot;
use mongodb::bson::oid::ObjectId;

// check to make sure the booking date is at least 24 hours in the future
// pub(crate) fn check_date_is_24hr_in_future(
//     booking_date: &BsonDateTime,
// ) -> Result<(), ValidationError> {
//     let twenty_four_hour_from_now =
//         BsonDateTime::from_system_time(SystemTime::now() + Duration::from_secs(86400));
//
//     if *booking_date < twenty_four_hour_from_now {
//         return Err(ValidationError::new(
//             "Booking date must be at least 24 hours in the future",
//         ));
//     }
//
//     Ok(())
// }

// verify that the SlotID exist in the Doctor ScheduleSlot, and is_available is set to true
pub(crate) fn verify_slot_id_exist<'a>(
    slot_id: &str,
    available_slots: &'a [ScheduleSlot],
) -> Result<&'a ScheduleSlot, AppointmentScheduleError> {
    let obj_id =
        ObjectId::parse_str(slot_id).map_err(|_| AppointmentScheduleError::InvalidSlotId)?;

    let slot = available_slots
        .iter()
        .find(|slot| slot.slot_id == Some(obj_id))
        .ok_or(AppointmentScheduleError::AppointmentNotFound)?;

    // confirm the slot is_available
    if slot.is_available != Some(true) {
        return Err(AppointmentScheduleError::UnableToScheduleAppointment);
    }

    Ok(slot)
}

// confirms that the patient's requested specialty matches one of the doctor's available specialties.
pub(crate) fn confirm_specialty_exist_for_doctor<T: PartialEq>(a: &T, b: &[T]) -> bool {
    b.iter().any(|item| item == a)
}

// Custom deserializer to convert String to Specialty
// pub fn deserialize_specialty_from_string<'de, D>(deserializer: D) -> Result<Specialty, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let specialty_str = String::deserialize(deserializer)?;
//     Specialty::from_str(&specialty_str).map_err(serde::de::Error::custom)
// }
