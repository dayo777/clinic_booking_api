#![allow(dead_code)]
// who booked, when, conflict-detection, slot-segregation, cancellation/resecheduling
use common::models::Specialty;
use mongodb::bson::{DateTime as BsonDateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct Appointment {
    #[serde(rename = "_id")]
    pub appointment_id: ObjectId,
    pub slot_id: ObjectId, // linked to the specific slot in DoctorSchedule
    pub doctor_id: ObjectId,
    pub patient_id: ObjectId,
    pub start_time: BsonDateTime,
    pub end_time: BsonDateTime,
    pub specialty: Specialty,
    pub notes: Option<String>,
    pub status: AppointmentStatus,
    pub created_at: BsonDateTime,
    pub updated_at: Option<BsonDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AppointmentStatus {
    Scheduled, // assigned when an Appointment is created
    Confirmed, // assigned when a Doctor confirms an appointment
    Canceled,  // assigned when a Doctor cancels an appointment
    Completed, // assigned when an appointment is concluded
}

#[derive(Deserialize, Validate, Debug)]
pub struct CreateAppointmentDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub appointment_id: Option<ObjectId>,
    pub slot_id: ObjectId, // Mandatory ref to a Doctor's slot
    pub doctor_id: ObjectId,
    pub patient_id: ObjectId,
    pub specialty: Specialty,
    pub notes: Option<String>,
}
