use crate::error::AppointmentScheduleError;
use common::models::ScheduleSlot;

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

// Custom deserializer to convert String to Specialty
// pub fn deserialize_specialty_from_string<'de, D>(deserializer: D) -> Result<Specialty, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let specialty_str = String::deserialize(deserializer)?;
//     Specialty::from_str(&specialty_str).map_err(serde::de::Error::custom)
// }
