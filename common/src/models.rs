use crate::utils::{
    deserialize_bson_datetime_or_string, deserialize_option_bson_datetime_or_string,
};
use mongodb::bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{Display, EnumString};
use validator::Validate;

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
        match s.to_ascii_lowercase().as_str() {
            "gp" | "general practice" | "generalpractice" => Ok(Self::GeneralPractice),
            "neuro" | "neuro surgery" | "neurosurgery" => Ok(Self::Neurosurgery),
            "cardio" | "cardiology" => Ok(Self::Cardiology),
            "derm" | "dermatology" => Ok(Self::Dermatology),
            _ => match Specialty::from_str(&s) {
                Ok(specialty) => Ok(specialty),
                Err(_) => Ok(Specialty::Other(s)),
            },
        }
    }
}

// A single schedule slot the Doctor creates
// Doctor create their own schedule which patient can book for.
#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct ScheduleSlot {
    #[serde(rename = "_id")]
    pub slot_id: String,
    #[serde(deserialize_with = "deserialize_bson_datetime_or_string")]
    pub start_time: BsonDateTime,
    // this is an Option so we can make every Schedule 30min long by default unless otherwise set
    #[serde(deserialize_with = "deserialize_option_bson_datetime_or_string")]
    pub end_time: Option<BsonDateTime>,
    // make this an Option so `ViewDoctorSchedule` can peek a ScheduleSlot
    #[serde(default)]
    pub is_available: bool, // changes to `False` once booked by a patient
    pub created_at: BsonDateTime,
    #[serde(
        default,
        deserialize_with = "deserialize_option_bson_datetime_or_string"
    )]
    pub updated_at: Option<BsonDateTime>,
}

// this is used to Create the ScheduleSlot
#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct CreateScheduleSlot {
    #[serde(deserialize_with = "deserialize_bson_datetime_or_string")]
    pub start_time: BsonDateTime,
    // this is an Option so we can make every Schedule 30min long by default unless otherwise set
    #[serde(deserialize_with = "deserialize_option_bson_datetime_or_string")]
    pub end_time: Option<BsonDateTime>,
}
