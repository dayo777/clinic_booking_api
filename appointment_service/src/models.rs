// who booked, when, conflict-detection, slot-segregation, cancellation/resecheduling
// use crate::utils::deserialize_specialty_from_string;
use common::models::Specialty;
use common::utils::validate_specialty;
use mongodb::bson::{DateTime as BsonDateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct AppointmentDto {
    #[serde(rename = "_id")]
    pub appointment_id: ObjectId,
    pub slot_id: ObjectId, // linked to the specific slot in DoctorSchedule
    pub doctor_id: ObjectId,
    pub patient_id: ObjectId,
    pub start_time: BsonDateTime,
    pub end_time: BsonDateTime,
    pub specialty: Specialty,
    pub status: AppointmentStatus,
    pub status_history: Vec<AppointmentStatusHistoryDto>,
    pub notes: Option<String>,
    pub created_at: BsonDateTime,
    pub updated_at: Option<BsonDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AppointmentStatus {
    Scheduled, // assigned when an Appointment is created
    Confirmed, // assigned when a Doctor confirms an appointment
    Canceled,  // assigned when a Doctor cancels an appointment
    Completed, // assigned when an appointment is concluded
    NoShow,    // assigned when patient does not show up
}

#[derive(Serialize, Validate, Debug, Deserialize)]
pub struct CreateAppointmentDto {
    #[validate(length(min = 5))]
    pub slot_id: String, // mandatory ref to a Doctor's slot
    #[validate(length(min = 5))]
    pub doctor_id: String,
    #[validate(length(min = 5))]
    pub patient_id: String,
    #[validate(custom(function = "validate_specialty"))]
    pub specialty: Specialty,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppointmentStatusHistoryDto {
    pub status: AppointmentStatus,
    pub changed_at: BsonDateTime,
    pub reason: Option<String>, // optional note e.g. "Patient canceled"
}
