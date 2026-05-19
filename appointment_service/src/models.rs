// // who booked, when, conflict-detection, slot-segregation, cancellation/resecheduling
// use crate::utils;
// use mongodb::bson::{DateTime as BsonDateTime, oid::ObjectId};
// use serde::{Deserialize, Serialize};
// use validator::Validate;
//
// #[derive(Serialize, Deserialize, Debug, Clone, Validate)]
// pub struct Appointment {
//     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
//     pub id: Option<ObjectId>,
//     pub doctor_id: ObjectId,
//     pub patient_id: ObjectId,
//     pub start_time: BsonDateTime,
//     pub end_time: BsonDateTime,
//     pub specialty: String,
//     pub status: AppointmentStatus,
//     pub created_at: BsonDateTime,
//     pub updated_at: Option<BsonDateTime>,
// }
//
// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// pub enum AppointmentStatus {
//     Scheduled,
//     Confirmed,
//     Canceled,
//     Completed,
// }
//
// #[derive(Deserialize, Validate, Debug)]
// pub struct CreateAppointmentDto {
//     pub doctor_id: ObjectId,
//     pub patient_id: ObjectId,
//     #[validate(custom(function = "utils::check_date_is_24hr_in_future"))]
//     pub start_time: BsonDateTime,
//     pub specialty: String,
// }
