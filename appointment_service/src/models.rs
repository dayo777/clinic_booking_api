// who booked, when, conflict-detection, slot-segregation, cancellation/resecheduling
use common::models::Specialty;
use common::utils::validate_specialty;
use mongodb::bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub(crate) struct AppointmentDto {
    #[serde(rename = "_id")]
    pub(crate) appointment_id: String,
    pub(crate) slot_id: String, // linked to the specific slot in DoctorSchedule
    pub(crate) doctor_id: String,
    pub(crate) patient_id: String,
    pub(crate) start_time: BsonDateTime,
    pub(crate) end_time: BsonDateTime,
    pub(crate) specialty: Specialty,
    pub(crate) status: AppointmentStatus,
    pub(crate) status_history: Vec<AppointmentStatusHistoryDto>,
    pub(crate) notes: Option<String>,
    pub(crate) created_at: BsonDateTime,
    pub(crate) updated_at: Option<BsonDateTime>,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AppointmentStatusHistoryDto {
    pub status: AppointmentStatus,
    pub changed_at: BsonDateTime,
    pub reason: String, // optional note by the Doc when changing App-status
}

// separate response DataObject so we can tweak the response format should we need to
#[derive(Serialize, Deserialize)]
pub struct AppointmentResponseDto {
    #[serde(rename = "_id")]
    pub appointment_id: String,
    pub slot_id: String, // linked to the specific slot in DoctorSchedule
    pub doctor_id: String,
    pub patient_id: String,
    pub start_time: BsonDateTime,
    pub end_time: BsonDateTime,
    pub specialty: Specialty,
    pub status: AppointmentStatus,
    pub status_history: Vec<AppointmentStatusHistoryDto>,
    pub notes: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

// use this when confirm/cancel appointment
#[derive(Deserialize, Debug, Validate)]
pub struct UpdateAppointmentStatusDto {
    // pub doctor_id: String,
    pub reason: Option<String>, // optional reason for cancellation or confirmation
}
