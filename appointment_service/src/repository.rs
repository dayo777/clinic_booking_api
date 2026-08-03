// Database operations: insert, find_by_id, find_by_email, etc.

use crate::models::CreateAppointmentDto;

pub async fn create_appointment(_payload: CreateAppointmentDto) -> Result<(), ()> {
    // 1. call get_active_doctor_schedule to confirm available slots
    // 1. verify the requested slot_id exists in the doctor's schedule
    // 2. verify is_available is true
    // 3. verify the Specialty chosen by the patient matches the Doctor specialty
    // 4. update the slot_id is_available to false
    // 5. persist the Appointment
    // NB: Expose endpoint for Doctor to Confirm an appointment

    // TODO: Remove the TODO once done.
    Ok(())
}
