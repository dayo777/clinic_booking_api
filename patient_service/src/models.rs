//! Data models and Data Transfer Objects (DTOs) for the patient service.
//!
//! This module contains the structs used for serializing and deserializing
//! request and response bodies, as well as validation logic.

use chrono::{DateTime, NaiveDate, Utc};
use mongodb::bson::{Bson, DateTime as BsonDateTime, oid::ObjectId};
use serde::{Deserialize, Deserializer, Serialize};
use strum::{Display, EnumString};
use validator::Validate;

/// Deserializes a BSON datetime field that may be stored as either BSON DateTime or an ISO string
/// (e.g. from chrono's default serde, which serializes DateTime as string). Accepts both so that
/// existing documents in MongoDB continue to work.
fn deserialize_bson_datetime_or_string<'de, D>(deserializer: D) -> Result<BsonDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let bson_val = Bson::deserialize(deserializer)?;
    match bson_val {
        Bson::DateTime(dt) => Ok(dt),
        Bson::String(s) => {
            let chrono_dt: DateTime<Utc> = s
                .parse()
                .map_err(|e: chrono::ParseError| serde::de::Error::custom(e))?;
            Ok(BsonDateTime::from_millis(chrono_dt.timestamp_millis()))
        }
        other => Err(serde::de::Error::custom(format!(
            "expected DateTime or string, got {:?}",
            other
        ))),
    }
}

/// Same as above for Option<BsonDateTime> (updated_at, deleted_at).
fn deserialize_option_bson_datetime_or_string<'de, D>(
    deserializer: D,
) -> Result<Option<BsonDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<Bson>::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(Bson::DateTime(dt)) => Ok(Some(dt)),
        Some(Bson::String(s)) => {
            let chrono_dt: DateTime<Utc> = s
                .parse()
                .map_err(|e: chrono::ParseError| serde::de::Error::custom(e))?;
            Ok(Some(BsonDateTime::from_millis(
                chrono_dt.timestamp_millis(),
            )))
        }
        Some(other) => Err(serde::de::Error::custom(format!(
            "expected DateTime or string, got {:?}",
            other
        ))),
    }
}

// Dto = Data Object
#[derive(Serialize, Deserialize, Debug, Validate)]
pub(crate) struct PatientDto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<ObjectId>,

    // Identity Information
    pub(crate) name: String,
    #[validate(range(min = 1, max = 100))]
    pub age: u8,
    pub(crate) dob: NaiveDate,
    pub(crate) gender: Gender,

    // Nested Contact Data
    pub(crate) contact: ContactInfo,

    // Clinical Metadata
    pub(crate) medical_alerts: Option<MedicalAlerts>,

    // Insurance (Optional)
    pub(crate) insurance: Option<InsuranceInfo>,

    // System Metadata
    pub(crate) is_active: bool,
    #[serde(deserialize_with = "deserialize_bson_datetime_or_string")]
    pub(crate) created_at: BsonDateTime,
    #[serde(deserialize_with = "deserialize_option_bson_datetime_or_string")]
    pub(crate) updated_at: Option<BsonDateTime>,
    #[serde(deserialize_with = "deserialize_option_bson_datetime_or_string")]
    pub(crate) deleted_at: Option<BsonDateTime>,
}

// initial Patient registration. Other medical details and Insurance are added later
#[derive(Deserialize, Validate, Debug)]
pub struct CreatePatientDto {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    pub dob: NaiveDate, // YYYY, MM, DD
    pub gender: Gender,
    pub contact_info: ContactInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PatientResponseDto {
    pub id: ObjectId,
    pub name: String,
    pub age: u8,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub contact: ContactInfo,
    pub medical_alerts: Option<MedicalAlerts>,
    pub insurance: Option<InsuranceInfo>,
}

#[derive(Deserialize, Debug)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, EnumString, Display, Clone, Copy)]
pub enum Gender {
    #[strum(serialize = "male")]
    #[serde(rename = "male")]
    Male,
    #[strum(serialize = "female")]
    #[serde(rename = "female")]
    Female,
    #[strum(serialize = "other")]
    #[serde(rename = "other")]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct ContactInfo {
    #[validate(length(min = 5))]
    pub phone: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 150))]
    pub address: String,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MedicalAlerts {
    pub blood_type: String,
    pub allergies: Vec<String>,
    pub chronic_conditions: Vec<String>,
    pub current_medications: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct InsuranceInfo {
    pub provider_name: String,
    pub policy_number: String,
    pub group_number: Option<String>,
    pub primary_holder_name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateInsuranceDto {
    pub provider_name: Option<String>,
    pub policy_number: Option<String>,
    pub group_number: Option<String>,
    pub primary_holder_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMedicalAlertsDto {
    pub blood_type: Option<String>,
    pub allergies: Option<Vec<String>>,
    pub chronic_conditions: Option<Vec<String>>,
    pub current_medications: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateContactInfoDto {
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
}
