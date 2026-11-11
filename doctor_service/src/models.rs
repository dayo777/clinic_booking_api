// use chrono::NaiveDate;
use common::models::{ScheduleSlot, Specialty};
use common::utils::validate_specialties;
use common::utils::{
    deserialize_bson_datetime_or_string, deserialize_option_bson_datetime_or_string,
};
use mongodb::bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

// Dto = Data Object
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DoctorDto {
    #[serde(rename = "_id")]
    pub(crate) doctor_id: String,
    pub(crate) name: String,

    // specialties
    pub(crate) specialties: Vec<Specialty>,
    pub(crate) license_num: String,

    // availability
    pub schedule: Option<ScheduleSlot>,

    // system generated
    #[serde(deserialize_with = "deserialize_bson_datetime_or_string")]
    pub(crate) created_at: BsonDateTime,
    #[serde(deserialize_with = "deserialize_option_bson_datetime_or_string")]
    pub(crate) updated_at: Option<BsonDateTime>,
    pub(crate) is_active: bool, // using bool instead of 'deleted_at' like in Patient-service
}

#[derive(Deserialize, Validate, Debug)]
pub struct CreateDoctorDto {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    #[validate(custom(function = "validate_specialties"))]
    pub specialties: Vec<Specialty>,
    #[validate(length(min = 5, max = 20))]
    pub license_num: String,
}

// use this struct to return calls for Doctor Data
#[derive(Serialize, Deserialize, Debug)]
pub struct DoctorResponseDto {
    #[serde(rename = "_id")]
    pub doctor_id: String,
    pub name: String,
    pub specialties: Vec<Specialty>,
    pub license_num: String,
    pub is_active: bool,
}

#[derive(Deserialize, Debug)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

// All definition for the Doctor Scheduling is defined below this line
// One bookable schedule per document, linked to its doctor.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DoctorSchedule {
    pub doctor_id: String,
    #[serde(flatten)]
    pub slot: ScheduleSlot,
}
