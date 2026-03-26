//! HTTP request handlers for the patient service.
//!
//! module contains the Actix-web route handlers for CRUD operations on patient resources.

use crate::{models, repository};
use actix_web::{HttpResponse, delete, get, post, put, web};
use tracing::{debug, error, info, instrument};

// existence Check (HEAD): SELECT 1 FROM patients WHERE id = 123 LIMIT 1;
// (Fast: the database only checks the index and returns a single bit).
// #[head("/{id}")]
// async fn check_patient_exists(path: web::Path<u32>) -> HttpResponse {
//     let id = path.into_inner();
//
//     // Call a specialized "exists" method in repository
//     if repository::patient_exists(id).await {
//         HttpResponse::Ok().finish() // Returns 200 OK, but no BODY
//     } else {
//         HttpResponse::NotFound().finish() // Returns 404
//     }
// }

#[post("")]
#[instrument(
    name = "create_patient_request",
    skip(payload),
    fields(payload = %payload.name)
)]
async fn create_patient(payload: web::Json<models::CreatePatientDto>) -> HttpResponse {
    info!("Processing new patient registration");

    match repository::create_patient(payload.into_inner()).await {
        Ok(_) => {
            info!("Registration successful");
            HttpResponse::Created().finish()
        }
        Err(e) => {
            error!(error = %e, "Registration failed");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{id}")]
#[instrument(
    name = "get_patient_request",
    fields(payload = %payload)
)]
async fn get_patient(payload: web::Path<String>) -> HttpResponse {
    info!("Retrieving patient information");

    match repository::get_single_patient(payload.into_inner()).await {
        Ok(Some(patient)) => {
            info!("Retrieved patient successful");
            HttpResponse::Ok().json(patient)
        }
        Ok(None) => {
            debug!("Patient not found");
            HttpResponse::NotFound().finish()
        }
        Err(e) => {
            debug!("Database error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// no parameters: /patients
// with parameters: /patients?page=2&limit=10
// partial parameters: /patients?page=3
#[get("")]
#[instrument(name = "list_patients_request", skip(query))]
async fn list_patients(query: web::Query<models::PaginationQuery>) -> HttpResponse {
    info!("Processing list patient");

    match repository::list_patient(query.into_inner()).await {
        Ok(patients) => {
            info!("Successfully retrieved patients list");
            HttpResponse::Ok().json(patients)
        }
        Err(e) => {
            error!(error = %e, error_debug = ?e, "Failed to list patients");
            HttpResponse::InternalServerError().finish()
        }
    }
}

// patient data is never deleted, moved to another Collection
#[delete("/{id}")]
#[instrument(
    name = "delete_patient_request",
    fields(id = %path)
)]
async fn delete_patient(path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    info!("Processing patient deletion for ID: {}", id);

    match repository::delete_patient(id).await {
        Ok(true) => {
            info!("Patient deleted/archived successfully");
            HttpResponse::NoContent().finish()
        }
        Ok(false) => {
            debug!("Patient not found or already deleted");
            HttpResponse::NotFound().finish()
        }
        Err(e) => {
            error!(error = %e, "Failed to delete patient");
            HttpResponse::InternalServerError().finish()
        }
    }
}

// #[put("/{id}")]
// async fn update_patient(path: web::Path<String>) -> HttpResponse {
//     let span = info_span!("update_patient", service.name = "patient_service");
//     let _guard = span.enter();
//
//     let id = path.into_inner();
//     info!("Updating patient handler with id {}", id);
//
//     // TODO: modify
//     HttpResponse::Ok().body("updated patient handler")
// }

// this handler updates the Insurance information for a Patient
#[put("/{id}/insurance")]
async fn update_patient_insurance(
    path: web::Path<String>,
    payload: web::Json<models::UpdateInsuranceDto>,
) -> HttpResponse {
    let id = path.into_inner();
    info!("Updating insurance for patient ID: {}", id);

    match repository::update_patient_insurance(id, payload.into_inner()).await {
        Ok(true) => {
            info!("Insurance updated successfully");
            HttpResponse::Ok().finish()
        }
        Ok(false) => {
            debug!("Patient not found or inactive");
            HttpResponse::NotFound().finish()
        }
        Err(e) => {
            error!(error = %e, "Failed to update insurance");
            HttpResponse::InternalServerError().finish()
        }
    }
}

// this handler updates the Medical Alerts for a Patient
#[put("/{id}/medical-alerts")]
async fn update_patient_medical_alerts(
    path: web::Path<String>,
    payload: web::Json<models::UpdateMedicalAlertsDto>,
) -> HttpResponse {
    let id = path.into_inner();
    info!("Updating medical alerts for patient ID: {}", id);

    match repository::update_patient_medical_alerts(id, payload.into_inner()).await {
        Ok(true) => {
            info!("Medical alerts updated successfully");
            HttpResponse::Ok().finish()
        }
        Ok(false) => {
            debug!("Patient not found or inactive");
            HttpResponse::NotFound().finish()
        }
        Err(e) => {
            error!(error = %e, "Failed to update medical alerts");
            HttpResponse::InternalServerError().finish()
        }
    }
}

// this handler updates the Contact Info for a Patient
#[put("/{id}/contact")]
async fn update_patient_contact_info(
    path: web::Path<String>,
    payload: web::Json<models::UpdateContactInfoDto>,
) -> HttpResponse {
    let id = path.into_inner();
    info!("Updating contact info for patient ID: {}", id);

    match repository::update_patient_contact_info(id, payload.into_inner()).await {
        Ok(true) => {
            info!("Contact info updated successfully");
            HttpResponse::Ok().finish()
        }
        Ok(false) => {
            debug!("Patient not found or inactive");
            HttpResponse::NotFound().finish()
        }
        Err(e) => {
            error!(error = %e, "Failed to update contact info");
            HttpResponse::InternalServerError().finish()
        }
    }
}

// other possible endpoints
// /{id}/appointments  -- get all appointments for this patient
// /{id}/status  -- check if patient is active or archived (deleted)
