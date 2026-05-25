// use chrono::NaiveDate;
use common::utils::{
    deserialize_bson_datetime_or_string, deserialize_option_bson_datetime_or_string,
};
use mongodb::bson::{DateTime as BsonDateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{Display, EnumString};
use validator::Validate;

// Dto = Data Object
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DoctorDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<ObjectId>,
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
    #[validate(custom(function = "crate::utils::validate_specialties"))]
    pub specialties: Vec<Specialty>,
    #[validate(length(min = 5, max = 20))]
    pub license_num: String,
}

// use this struct to return calls for Doctor Data
#[derive(Serialize, Deserialize, Debug)]
pub struct DoctorResponseDto {
    #[serde(rename = "_id")]
    pub id: ObjectId,
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

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash, EnumString, Display)]
#[strum(ascii_case_insensitive)]
#[serde(rename_all = "lowercase")]
pub enum Specialty {
    #[strum(
        serialize = "General Practice",
        to_string = "General Practice",
        serialize = "gp",
        serialize = "general practice"
    )]
    #[serde(alias = "general practice", alias = "gp")]
    GeneralPractice,

    #[strum(
        serialize = "Neurosurgery",
        to_string = "Neurosurgery",
        serialize = "neuro surgery",
        serialize = "neuro"
    )]
    #[serde(alias = "neuro surgery", alias = "neuro")]
    Neurosurgery,

    #[strum(
        serialize = "Cardiology",
        to_string = "Cardiology",
        serialize = "cardio"
    )]
    #[serde(alias = "cardio")]
    Cardiology,

    #[strum(
        serialize = "Dermatology",
        to_string = "Dermatology",
        serialize = "derm"
    )]
    #[serde(alias = "derm")]
    Dermatology,

    #[strum(default)]
    Other(String),
}

impl<'de> Deserialize<'de> for Specialty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match Specialty::from_str(&s) {
            Ok(specialty) => Ok(specialty),
            Err(_) => Ok(Specialty::Other(s)),
        }
    }
}

/// All definition for the Doctor Scheduling is defined below this line
/// Trying to keep the Backend structure simple, just using Date & Time
/// The front-end should easily infer the DayOfWeek and other details from the Date&Time

// A single schedule slot the Doctor creates
// Doctor create their own schedule which patient can book for.
#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct ScheduleSlot {
    pub start_time: BsonDateTime,
    // this is an Option so we can make every Schedule 30min long
    pub end_time: Option<BsonDateTime>,
    // make this an Option so `ViewDoctorSchedule` can peek a ScheduleSlot
    pub is_available: Option<bool>, // changes to `False` once booked by a patient
}

// Have a separate structure to store the Doctor schedule to avoid hitting the main DoctorDto
#[derive(Serialize, Deserialize, Debug)]
pub struct DoctorSchedule {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub doctor_id: ObjectId,
    pub slots: Vec<ScheduleSlot>,
    #[serde(deserialize_with = "deserialize_bson_datetime_or_string")]
    pub created_at: BsonDateTime,
    #[serde(deserialize_with = "deserialize_option_bson_datetime_or_string")]
    pub updated_at: Option<BsonDateTime>,
}

// use this to peep if a Doctor is available for a given slot
// appointment endpoint calls this to know if Patient can book that Slot
pub struct ViewDoctorSchedule {
    pub doctor_id: ObjectId,
    pub slots: ScheduleSlot,
}
