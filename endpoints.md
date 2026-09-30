# Project Endpoints

> Before testing the endpoints, ensure that Jaeger and MongoDB are running. See `example_settings_dev.toml` for configuration.

## Base URL

`http://localhost:8080/api`

All endpoints require the header `x-api-version: 1`.

## Table of Contents

- [Homepage](#homepage)
- [Patient Service](#patient-service)
- [Doctor Service](#doctor-service)
  - [List Inactive Doctors](#list-inactive-doctors)
- [Appointment Service](#appointment-service)

## Endpoint Summary

### Homepage

- [`GET /api`](#homepage) — API health/welcome message

### Patient Service

- [`HEAD /api/patient/{id}`](#check-patient-existence) — Check whether a patient exists
- [`GET /api/patient`](#list-patients) — List active patients; supports `page` and `limit`
- [`POST /api/patient`](#create-patient) — Register a patient
- [`GET /api/patient/{id}`](#retrieve-patient) — Retrieve a patient
- [`DELETE /api/patient/{id}`](#archive-patient) — Archive a patient
- [`PUT /api/patient/{id}/insurance`](#update-patient-insurance) — Update insurance information
- [`PUT /api/patient/{id}/medical-alerts`](#update-patient-medical-alerts) — Update medical alerts
- [`PUT /api/patient/{id}/contact`](#update-patient-contact) — Update contact information

### Doctor Service

- [`HEAD /api/doctor/{id}`](#check-doctor-existence) — Check whether a doctor exists
- [`POST /api/doctor`](#create-doctor) — Create a doctor
- [`GET /api/doctor/{id}`](#retrieve-doctor) — Retrieve a doctor
- [`GET /api/doctor`](#list-active-doctors) — List active doctors; supports `page` and `limit`
- [`GET /api/doctor/inactive`](#list-inactive-doctors) — List inactive doctors; supports `page` and `limit`
- [`PATCH /api/doctor/{id}/enable`](#enable-doctor) — Re-enable an inactive doctor
- [`DELETE /api/doctor/{id}`](#archive-doctor) — Archive/deactivate a doctor
- [`POST /api/doctor/{id}/create-doctor-schedule`](#create-doctor-schedule) — Create schedule slots
- [`GET /api/doctor/{id}/active-doctor-schedule`](#retrieve-active-doctor-schedule) — Retrieve active schedule slots

### Appointment Service

- [`POST /api/appointment`](#create-appointment) — Create an appointment
- [`GET /api/appointment/{id}`](#retrieve-appointment) — Retrieve an appointment
- [`GET /api/appointment/patient/{patient_id}`](#list-patient-appointments) — List patient appointments; supports `page` and `limit`
- [`GET /api/appointment/doctor/{doctor_id}`](#list-doctor-appointments) — List doctor appointments; supports `page` and `limit`
- [`PATCH /api/appointment/{appointment_id}/confirm`](#confirm-appointment) — Confirm an appointment
- [`PATCH /api/appointment/{appointment_id}/cancel`](#cancel-appointment) — Cancel an appointment
- [`PATCH /api/appointment/{appointment_id}/complete`](#complete-appointment) — Complete an appointment
- [`PATCH /api/appointment/{appointment_id}/no-show`](#mark-appointment-no-show) — Mark an appointment as no-show

## Command-Line Examples

All examples include the required API-version header. Replace path placeholders with actual IDs.

## Homepage

### Homepage

```bash
curl -X GET http://localhost:8080/api \
  -H "x-api-version: 1"
```

## Patient Service

### Check Patient Existence

```bash
curl -I http://localhost:8080/api/patient/{id} \
  -H "x-api-version: 1"
```

### Create Patient

```bash
curl -X POST http://localhost:8080/api/patient \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '{"name":"Dru Oruns","dob":"1960-10-01","gender":"male","contact_info":{"phone":"+111111","email":"oruns@outlook.com","address":"House 1A, Jai Crescent","emergency_contact_name":"Flavian Oruns","emergency_contact_phone":"+127833"}}'
```

### Retrieve Patient

```bash
curl http://localhost:8080/api/patient/{id} -H "x-api-version: 1"
```

### List Patients

```bash
curl "http://localhost:8080/api/patient?page=1&limit=10" -H "x-api-version: 1"
```

### Archive Patient

```bash
curl -X DELETE http://localhost:8080/api/patient/{id} -H "x-api-version: 1"
```

### Update Patient Insurance

```bash
curl -X PUT http://localhost:8080/api/patient/{id}/insurance \
  -H "Content-Type: application/json" -H "x-api-version: 1" \
  -d '{"provider_name":"HealthShield","policy_number":"HS-987654321","group_number":"G-112233","primary_holder_name":"Dru Oruns"}'
```

### Update Patient Medical Alerts

```bash
curl -X PUT http://localhost:8080/api/patient/{id}/medical-alerts \
  -H "Content-Type: application/json" -H "x-api-version: 1" \
  -d '{"blood_type":"O+","allergies":["Peanuts","Penicillin"],"chronic_conditions":["Asthma"],"current_medications":["Albuterol"]}'
```

### Update Patient Contact

```bash
curl -X PUT http://localhost:8080/api/patient/{id}/contact \
  -H "Content-Type: application/json" -H "x-api-version: 1" \
  -d '{"phone":"+1222222","email":"updated_oruns@outlook.com","address":"New House 1B, Jai Crescent","emergency_contact_name":"Flavian Oruns","emergency_contact_phone":"+127833"}'
```

## Doctor Service

### Check Doctor Existence

```bash
curl -I http://localhost:8080/api/doctor/{id} -H "x-api-version: 1"
```

### Create Doctor

```bash
curl -X POST http://localhost:8080/api/doctor \
  -H "Content-Type: application/json" -H "x-api-version: 1" \
  -d '{"name":"Dr. Smith","specialties":["gp","derm"],"license_num":"MED-12345"}'
```

### Retrieve Doctor

```bash
curl http://localhost:8080/api/doctor/{id} -H "x-api-version: 1"
```

### List Active Doctors

```bash
curl "http://localhost:8080/api/doctor?page=1&limit=10" -H "x-api-version: 1"
```

### List Inactive Doctors

Returns doctors whose `is_active` value is false. Supports optional `page` and `limit` query parameters.

```bash
curl "http://localhost:8080/api/doctor/inactive?page=1&limit=10" \
  -H "x-api-version: 1"
```

### Enable Doctor

```bash
curl -X PATCH http://localhost:8080/api/doctor/{id}/enable -H "x-api-version: 1"
```

### Archive Doctor

```bash
curl -X DELETE http://localhost:8080/api/doctor/{id} -H "x-api-version: 1"
```

### Create Doctor Schedule

```bash
curl -X POST http://localhost:8080/api/doctor/{id}/create-doctor-schedule \
  -H "Content-Type: application/json" -H "x-api-version: 1" \
  -d '[{"start_time":"2026-10-01T08:00:00Z","end_time":"2026-10-01T08:30:00Z"},{"start_time":"2026-10-01T09:00:00Z","end_time":"2026-10-01T09:30:00Z"}]'
```

### Retrieve Active Doctor Schedule

```bash
curl http://localhost:8080/api/doctor/{id}/active-doctor-schedule -H "x-api-version: 1"
```

## Appointment Service

The status-update endpoints currently expect a JSON body containing `reason`.

### Create Appointment

```bash
curl -X POST http://localhost:8080/api/appointment \
  -H "Content-Type: application/json" -H "x-api-version: 1" \
  -d '{"slot_id":"slot_617c96cff9b9f1ddd65","doctor_id":"doc_b617df81ec12e9b6a651","patient_id":"pat_56df17fc9e814a512552","specialty":"derm","notes":"Having some skin issues, like eczema."}'
```

### Retrieve Appointment

```bash
curl http://localhost:8080/api/appointment/{id} -H "x-api-version: 1"
```

### List Patient Appointments

```bash
curl "http://localhost:8080/api/appointment/patient/{patient_id}?page=1&limit=10" -H "x-api-version: 1"
```

### List Doctor Appointments

```bash
curl "http://localhost:8080/api/appointment/doctor/{doctor_id}?page=1&limit=10" -H "x-api-version: 1"
```

### Confirm Appointment

```bash
curl -X PATCH http://localhost:8080/api/appointment/{appointment_id}/confirm \
  -H "Content-Type: application/json" -H "x-api-version: 1" \
  -d '{"reason":"Appointment confirmed by doctor."}'
```

### Cancel Appointment

```bash
curl -X PATCH http://localhost:8080/api/appointment/{appointment_id}/cancel \
  -H "Content-Type: application/json" -H "x-api-version: 1" \
  -d '{"reason":"Doctor unavailable."}'
```

### Complete Appointment

```bash
curl -X PATCH http://localhost:8080/api/appointment/{appointment_id}/complete \
  -H "Content-Type: application/json" -H "x-api-version: 1" \
  -d '{"reason":"Consultation completed."}'
```

### Mark Appointment No-Show

```bash
curl -X PATCH http://localhost:8080/api/appointment/{appointment_id}/no-show \
  -H "Content-Type: application/json" -H "x-api-version: 1" \
  -d '{"reason":"Patient did not attend the appointment."}'
```
