//! Data access layer for patient records.
//!
//! This module handles database operations such as inserting, finding,
//! updating, and deleting patient documents in MongoDB.
//!

use crate::models::{
    ContactInfo, CreatePatientDto, PaginationQuery, PatientDto, PatientResponseDto,
    UpdateInsuranceDto, UpdateMedicalAlertsDto,
};
use crate::utils;
use chrono::Utc;
use common::db::get_collection;
use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::error::Error as MongodbError;
use mongodb::options::FindOptions;
use mongodb::results::InsertOneResult;
use std::time::Duration;
use tracing::{debug, info, instrument};

// this is the Collection name this workspace interacts with.
static PATIENT_COLLECTION: &str = "patients_table";
static PATIENT_DELETED_COLLECTION: &str = "patient_deleted";

#[instrument(name = "db_create_patient", skip(payload))]
pub(crate) async fn create_patient(
    payload: CreatePatientDto,
) -> Result<InsertOneResult, MongodbError> {
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
        is_active: true,
        created_at: Utc::now(),
        updated_at: None,
        deleted_at: None,
    };

    let collection = get_collection::<PatientDto>(PATIENT_COLLECTION);
    info!("Executing MongoDB insert one.");
    collection.insert_one(new_patient).await
}

#[instrument(name = "db_get_patient", skip(patient_id))]
pub(crate) async fn get_single_patient(
    patient_id: String,
) -> Result<Option<PatientResponseDto>, MongodbError> {
    let collection = get_collection::<PatientDto>(PATIENT_COLLECTION);

    let obj_id = match ObjectId::parse_str(&patient_id) {
        Ok(id) => id,
        Err(e) => {
            debug!("Invalid ObjectId format: {}", e);
            return Ok(None);
        }
    };

    let filter = doc! { "_id": obj_id, "is_active": true };

    info!("Executing MongoDB FindOne");
    let patient_doc = collection
        .find_one(filter)
        .max_time(Duration::from_secs(10))
        .await?;

    Ok(patient_doc.map(|p| PatientResponseDto {
        id: p.id.map(|oid| oid.to_hex()).unwrap_or_default(),
        name: p.name,
        age: p.age,
        dob: p.dob,
        gender: p.gender,
        contact: p.contact,
        medical_alerts: p.medical_alerts,
        insurance: p.insurance,
    }))
}

#[instrument(name = "db_list_patients", skip(pagination))]
pub(crate) async fn list_patient(
    pagination: PaginationQuery,
) -> Result<Vec<PatientResponseDto>, MongodbError> {
    let collection = get_collection::<PatientDto>(PATIENT_COLLECTION);

    let limit = pagination.limit.unwrap_or(15);
    let page = pagination.page.unwrap_or(1);
    let skip = (page - 1) * limit;

    let find_options = FindOptions::builder()
        .limit(limit as i64)
        .skip(skip)
        .build();

    info!(
        limit = limit,
        page = page,
        skip = skip,
        "Executing MongoDB Find for patient listing"
    );

    let filter = doc! { "is_active": true };
    let mut cursor = collection.find(filter).with_options(find_options).await?;
    let mut patients = Vec::new();

    while let Some(result) = cursor.next().await {
        let p = result?;
        patients.push(PatientResponseDto {
            id: p.id.map(|oid| oid.to_hex()).unwrap_or_default(),
            name: p.name,
            age: p.age,
            dob: p.dob,
            gender: p.gender,
            contact: p.contact,
            medical_alerts: p.medical_alerts,
            insurance: p.insurance,
        });
    }

    Ok(patients)
}

#[instrument(name = "db_delete_patient", skip(patient_id))]
pub(crate) async fn delete_patient(patient_id: String) -> Result<bool, MongodbError> {
    let collection = get_collection::<PatientDto>(PATIENT_COLLECTION);
    let archive_collection = get_collection::<PatientDto>(PATIENT_DELETED_COLLECTION);

    let obj_id = match ObjectId::parse_str(&patient_id) {
        Ok(id) => id,
        Err(e) => {
            debug!("Invalid ObjectId format for deletion: {}", e);
            return Ok(false);
        }
    };

    let filter = doc! { "_id": obj_id, "is_active": true };

    // 1. Find the patient
    let patient_doc = collection.find_one(filter.clone()).await?;

    if let Some(mut patient) = patient_doc {
        // 2. Update status and set deletion date
        patient.is_active = false;
        patient.deleted_at = Some(Utc::now());

        // 3. Insert into archive collection
        archive_collection.insert_one(&patient).await?;

        // 4. Remove from active collection
        collection.delete_one(filter).await?;

        info!(
            "Patient archived and removed from active collection: {}",
            patient_id
        );
        Ok(true)
    } else {
        debug!("Patient not found or already inactive: {}", patient_id);
        Ok(false)
    }
}

#[instrument(name = "db_update_patient_insurance", skip(patient_id, insurance))]
pub(crate) async fn update_patient_insurance(
    patient_id: String,
    insurance: UpdateInsuranceDto,
) -> Result<bool, MongodbError> {
    let collection = get_collection::<PatientDto>(PATIENT_COLLECTION);

    let obj_id = match ObjectId::parse_str(&patient_id) {
        Ok(id) => id,
        Err(e) => {
            debug!("Invalid ObjectId format: {}", e);
            return Ok(false);
        }
    };

    let mut update_doc = doc! {};

    if let Some(provider) = insurance.provider_name {
        update_doc.insert("insurance.provider_name", provider);
    }
    if let Some(policy) = insurance.policy_number {
        update_doc.insert("insurance.policy_number", policy);
    }
    if let Some(group) = insurance.group_number {
        update_doc.insert("insurance.group_number", group);
    }
    if let Some(holder) = insurance.primary_holder_name {
        update_doc.insert("insurance.primary_holder_name", holder);
    }

    if update_doc.is_empty() {
        return Ok(true);
    }

    update_doc.insert("updated_at", mongodb::bson::DateTime::now());

    let filter = doc! { "_id": obj_id, "is_active": true };

    // Ensure insurance field is an object if it's currently null or missing
    // We use a separate update to ensure the structure is correct before applying dot-notation updates,
    // or we can use a pipeline-style update if supported, or simply check and set.
    // Given the error "Cannot create field 'group_number' in element {insurance: null}",
    // it means insurance exists but is null.

    let unset_null_insurance = doc! { "$set": { "insurance": {} } };
    let filter_null = doc! { "_id": obj_id, "is_active": true, "insurance": null };
    let _ = collection
        .update_one(filter_null, unset_null_insurance)
        .await?;

    let update = doc! { "$set": update_doc };
    let result = collection.update_one(filter, update).await?;

    Ok(result.matched_count > 0)
}

#[instrument(name = "db_update_patient_medical_alerts", skip(patient_id, alerts))]
pub(crate) async fn update_patient_medical_alerts(
    patient_id: String,
    alerts: UpdateMedicalAlertsDto,
) -> Result<bool, MongodbError> {
    let collection = get_collection::<PatientDto>(PATIENT_COLLECTION);

    let obj_id = match ObjectId::parse_str(&patient_id) {
        Ok(id) => id,
        Err(e) => {
            debug!("Invalid ObjectId format: {}", e);
            return Ok(false);
        }
    };

    let mut update_doc = doc! {};

    if let Some(blood_type) = alerts.blood_type {
        update_doc.insert("medical_alerts.blood_type", blood_type);
    }
    if let Some(allergies) = alerts.allergies {
        update_doc.insert("medical_alerts.allergies", allergies);
    }
    if let Some(conditions) = alerts.chronic_conditions {
        update_doc.insert("medical_alerts.chronic_conditions", conditions);
    }
    if let Some(medications) = alerts.current_medications {
        update_doc.insert("medical_alerts.current_medications", medications);
    }

    if update_doc.is_empty() {
        return Ok(true);
    }

    update_doc.insert("updated_at", mongodb::bson::DateTime::now());

    let filter = doc! { "_id": obj_id, "is_active": true };

    // Ensure medical_alerts field is an object if it's currently null or missing
    let unset_null_alerts = doc! { "$set": { "medical_alerts": {} } };
    let filter_null = doc! { "_id": obj_id, "is_active": true, "medical_alerts": null };
    let _ = collection
        .update_one(filter_null, unset_null_alerts)
        .await?;

    let update = doc! { "$set": update_doc };
    let result = collection.update_one(filter, update).await?;

    Ok(result.matched_count > 0)
}
