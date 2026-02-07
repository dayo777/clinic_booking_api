//! Data models and Data Transfer Objects (DTOs) for the patient service.
//!
//! This module contains the structs used for serializing and deserializing
//! request and response bodies, as well as validation logic.

use chrono::{DateTime, NaiveDate, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use validator::Validate;

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
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: Option<DateTime<Utc>>,
    pub(crate) deleted_at: Option<DateTime<Utc>>,
}

// initial Patient registration. Other medical details and Insurance are added later
#[derive(Deserialize, Validate, Debug)]
pub(crate) struct CreatePatientDto {
    #[validate(length(min = 3, max = 100))]
    pub(crate) name: String,
    pub(crate) dob: NaiveDate, // YYYY, MM, DD
    pub(crate) gender: Gender,
    pub(crate) contact_info: ContactInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PatientResponseDto {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) age: u8,
    pub(crate) dob: NaiveDate,
    pub(crate) gender: Gender,
    pub(crate) contact: ContactInfo,
    pub(crate) medical_alerts: Option<MedicalAlerts>,
    pub(crate) insurance: Option<InsuranceInfo>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct PaginationQuery {
    pub(crate) page: Option<u64>,
    pub(crate) limit: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, EnumString, Display)]
pub(crate) enum Gender {
    #[strum(serialize = "male")]
    Male,
    #[strum(serialize = "female")]
    Female,
    #[strum(serialize = "other")]
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
pub(crate) struct UpdateInsuranceDto {
    pub provider_name: Option<String>,
    pub policy_number: Option<String>,
    pub group_number: Option<String>,
    pub primary_holder_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct UpdateMedicalAlertsDto {
    pub blood_type: Option<String>,
    pub allergies: Option<Vec<String>>,
    pub chronic_conditions: Option<Vec<String>>,
    pub current_medications: Option<Vec<String>>,
}
