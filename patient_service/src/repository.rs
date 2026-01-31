//! Data access layer for patient records.
//!
//! This module handles database operations such as inserting, finding,
//! updating, and deleting patient documents in MongoDB.
//!

use crate::models::{ContactInfo, CreatePatientDto, PatientDto};
use crate::utils;
use actix_web::web;
use chrono::Utc;
use common::db::get_collection;
use mongodb::error::Error as MongodbError;
use mongodb::results::InsertOneResult;
use tracing::{info, instrument};

// this is the Collection name this workspace interacts with.
static PATIENT_COLLECTION: &str = "patients_table";

#[instrument(skip(payload))]
pub(crate) async fn create_patient(
    payload: web::Json<CreatePatientDto>,
) -> Result<InsertOneResult, MongodbError> {
    info!("Entering the create patient repository");
    let payload = payload.into_inner();

    let dob = payload.dob;
    let age = utils::calculate_age(payload.dob);
    let contact_info = ContactInfo {
        phone: payload.contact_info.phone,
        email: payload.contact_info.email,
        address: payload.contact_info.address,
        emergency_contact_name: payload.contact_info.emergency_contact_name,
        emergency_contact_phone: payload.contact_info.emergency_contact_phone,
    };
    let gender = payload.gender;

    let new_patient = PatientDto {
        id: None,
        name: payload.name,
        age,
        dob,
        gender,
        contact: contact_info,
        medical_alerts: None,
        insurance: None,
        is_active: false,
        created_at: Utc::now(),
        updated_at: None,
    };

    let collection = get_collection::<PatientDto>(PATIENT_COLLECTION);
    info!("Inserting patient data into the Patient collection...");
    collection.insert_one(new_patient).await
}
