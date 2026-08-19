// Data models and Data Transfer Objects (DTOs) for the patient service.

// This module contains the structs used for serializing and deserializing
// request and response bodies, as well as validation logic.

use chrono::NaiveDate;
use common::utils::{
    deserialize_bson_datetime_or_string, deserialize_option_bson_datetime_or_string,
};
use mongodb::bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use validator::Validate;

// Dto = Data Object
#[derive(Serialize, Deserialize, Debug, Validate)]
pub(crate) struct PatientDto {
    #[serde(rename = "_id")]
    pub(crate) patient_id: String,

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

// use this to return calls for Patient Data
#[derive(Serialize, Debug)]
pub struct PatientResponseDto {
    #[serde(rename = "_id")]
    pub patient_id: String,
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

#[derive(Serialize, Deserialize, Debug, EnumString, Display, Clone, Copy, PartialEq)]
pub enum Gender {
    #[strum(serialize = "male")]
    #[serde(rename = "male", alias = "Male")]
    Male,
    #[strum(serialize = "female")]
    #[serde(rename = "female", alias = "Female")]
    Female,
    #[strum(serialize = "other")]
    #[serde(rename = "other", alias = "Other")]
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
    #[validate(length(min = 5))]
    pub phone: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 1, max = 150))]
    pub address: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
}
