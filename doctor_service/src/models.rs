// who the doctor is, associated_clinic, doctor-working-schedule
use chrono::{NaiveDate, NaiveTime};
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
    pub schedule: Option<Schedule>,

    // System generated
    #[serde(deserialize_with = "deserialize_bson_datetime_or_string")]
    pub(crate) created_at: BsonDateTime,
    #[serde(deserialize_with = "deserialize_option_bson_datetime_or_string")]
    pub(crate) updated_at: Option<BsonDateTime>,
    pub(crate) is_active: bool, // using bool instead of 'deleted_at' as in Patient-service
}

// full availability stored for a Doctor at the start of the week
// this can change every new week
#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Schedule {
    pub slot_duration_minutes: u32, // slot duration for each patient attended to
    pub weekly: Vec<WeeklySlot>,    // the repeating weekly template
    pub overrides: Vec<ScheduleOverride>, // date-specific exceptions
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeeklySlot {
    pub day_of_week: DayOfWeek,
    pub hours: WorkingHours,
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, Display)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingHours {
    start_time: NaiveTime,
    end_time: NaiveTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScheduleOverride {
    pub date: NaiveDate,
    pub available: bool,
    pub working_hours: Option<WorkingHours>,
    pub reason: Option<String>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDoctorSchedule {
    // TODO: use this to create a Doctor Schedule endpoint
}

pub struct ViewDoctorSchedule {
    // TODO: use this to view a Doctor Schedule, endpoint
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
