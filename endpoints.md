# Clinic Booking API Endpoint Reference

This reference describes the actual HTTP routes implemented by the Rust Actix API. It is written to be as complete and frontend-friendly as possible.

## Base URL

`http://localhost:8080/api`

All requests require the header:

```http
x-api-version: 1
```

The application mounts all services under `/api`, and the version guard checks the `x-api-version` header before accepting the request.

## API conventions

### Global response behavior

- `200 OK` — successful read or update
- `201 Created` — a resource was created
- `204 No Content` — request succeeded without a response body
- `400 Bad Request` — validation or request-shape error
- `404 Not Found` — resource missing, inactive, or no match
- `500 Internal Server Error` — server-side or database failure

### Common request patterns

- JSON request bodies are used for create/update operations.
- Pagination is supported on list endpoints through `page` and `limit` query parameters.
- IDs are string-based application IDs, not Mongo ObjectId values in the API layer.
- Timestamps use ISO-8601/RFC3339 values when serialized to JSON.

### Pagination

The following endpoints accept optional pagination:

- `GET /patient`
- `GET /doctor`
- `GET /doctor/inactive`
- `GET /appointment/patient/{patient_id}`
- `GET /appointment/doctor/{doctor_id}`

Query parameters:

- `page` — default `1`
- `limit` — default `15`, max `100`

Example:

```http
GET /patient?page=1&limit=10
GET /doctor?page=2&limit=20
GET /appointment/patient/pat_123?page=1&limit=25
```

### Common data types

- `Gender`: `male`, `female`, `other`
- `Specialty`: `gp`, `general practice`, `neuro`, `neuro surgery`, `cardio`, `derm`, `dermatology`
- `AppointmentStatus`: `Scheduled`, `Confirmed`, `Canceled`, `Completed`, `NoShow`

## Route index

### Root

- `GET /` — welcome endpoint

### Patient service

- `HEAD /patient/{id}` — check patient existence
- `POST /patient` — create patient
- `GET /patient/{id}` — get patient by ID
- `GET /patient` — list active patients
- `DELETE /patient/{id}` — archive patient
- `PUT /patient/{id}/insurance` — update insurance details
- `PUT /patient/{id}/medical-alerts` — update medical alerts
- `PUT /patient/{id}/contact` — update contact details

### Doctor service

- `HEAD /doctor/{id}` — check doctor existence
- `POST /doctor` — create doctor
- `GET /doctor/{id}` — get active doctor by ID
- `GET /doctor` — list active doctors
- `GET /doctor/inactive` — list inactive doctors
- `DELETE /doctor/{id}` — deactivate doctor
- `PATCH /doctor/{id}/enable` — re-enable doctor
- `PATCH /doctor/{id}/disable-doctor` — disable doctor
- `POST /doctor/{id}/create-doctor-schedule` — create schedule slots
- `GET /doctor/{id}/schedule` — full doctor schedule
- `GET /doctor/{id}/active-doctor-schedule` — currently available future slots

### Appointment service

- `POST /appointment` — create appointment
- `GET /appointment/{id}` — retrieve appointment
- `GET /appointment/patient/{patient_id}` — list appointments for patient
- `GET /appointment/doctor/{doctor_id}` — list appointments for doctor
- `PATCH /appointment/{appointment_id}/confirm` — confirm appointment
- `PATCH /appointment/{appointment_id}/cancel` — cancel appointment
- `PATCH /appointment/{appointment_id}/complete` — complete appointment
- `PATCH /appointment/{appointment_id}/no-show` — mark no-show
- `GET /appointment/{appointment_id}/status-history` — view status history

## Root endpoint

### GET /

Returns the welcome message for the versioned API.

Example:

```bash
curl -X GET http://localhost:8080/api \
  -H "x-api-version: 1"
```

Response:

```text
Welcome to v1!
```

## Patient service

### HEAD /patient/{id}

Checks whether a patient exists and is active.

Behavior:

- `204 No Content` if the patient exists and is active
- `404 Not Found` if the patient is missing or inactive

Example:

```bash
curl -I http://localhost:8080/api/patient/{id} \
  -H "x-api-version: 1"
```

### POST /patient

Creates a patient.

Request body:

```json
{
  "name": "Dru Oruns",
  "dob": "1960-10-01",
  "gender": "male",
  "contact_info": {
    "phone": "+111111",
    "email": "oruns@outlook.com",
    "address": "House 1A, Jai Crescent",
    "emergency_contact_name": "Flavian Oruns",
    "emergency_contact_phone": "+127833"
  }
}
```

Validation rules:

- `name`: length 3 to 100
- `dob`: valid ISO date (`YYYY-MM-DD`)
- `gender`: `male`, `female`, or `other`
- `contact_info.phone`: minimum length 5
- `contact_info.email`: must be a valid email
- `contact_info.address`: length 1 to 150

Example:

```bash
curl -X POST http://localhost:8080/api/patient \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '{
    "name": "Dru Oruns",
    "dob": "1960-10-01",
    "gender": "male",
    "contact_info": {
      "phone": "+111111",
      "email": "oruns@outlook.com",
      "address": "House 1A, Jai Crescent",
      "emergency_contact_name": "Flavian Oruns",
      "emergency_contact_phone": "+127833"
    }
  }'
```

Success:

- `201 Created`
- Response body: none in the handler implementation

### GET /patient/{id}

Retrieve a single active patient.

Example:

```bash
curl http://localhost:8080/api/patient/{id} \
  -H "x-api-version: 1"
```

Typical response:

```json
{
  "_id": "pat_56df17fc9e814a512552",
  "name": "Dru Oruns",
  "age": 65,
  "dob": "1960-10-01",
  "gender": "male",
  "contact": {
    "phone": "+111111",
    "email": "oruns@outlook.com",
    "address": "House 1A, Jai Crescent",
    "emergency_contact_name": "Flavian Oruns",
    "emergency_contact_phone": "+127833"
  },
  "medical_alerts": null,
  "insurance": null
}
```

### GET /patient

List active patients.

Example:

```bash
curl "http://localhost:8080/api/patient?page=1&limit=10" \
  -H "x-api-version: 1"
```

Typical response:

```json
[
  {
    "_id": "pat_56df17fc9e814a512552",
    "name": "Dru Oruns",
    "age": 65,
    "dob": "1960-10-01",
    "gender": "male",
    "contact": {
      "phone": "+111111",
      "email": "oruns@outlook.com",
      "address": "House 1A, Jai Crescent",
      "emergency_contact_name": "Flavian Oruns",
      "emergency_contact_phone": "+127833"
    },
    "medical_alerts": null,
    "insurance": null
  }
]
```

### DELETE /patient/{id}

Archive/deactivate a patient. The implementation moves the patient to an archive collection rather than hard-deleting the record.

Example:

```bash
curl -X DELETE http://localhost:8080/api/patient/{id} \
  -H "x-api-version: 1"
```

Success:

- `204 No Content`
- `404 Not Found` if the patient is missing or already inactive

### PUT /patient/{id}/insurance

Update patient insurance information.

Request body:

```json
{
  "provider_name": "HealthShield",
  "policy_number": "HS-987654321",
  "group_number": "G-112233",
  "primary_holder_name": "Dru Oruns"
}
```

Example:

```bash
curl -X PUT http://localhost:8080/api/patient/{id}/insurance \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '{
    "provider_name": "HealthShield",
    "policy_number": "HS-987654321",
    "group_number": "G-112233",
    "primary_holder_name": "Dru Oruns"
  }'
```

Success:

- `200 OK`
- `404 Not Found` if patient missing/inactive

### PUT /patient/{id}/medical-alerts

Update patient medical alert data.

Request body:

```json
{
  "blood_type": "O+",
  "allergies": ["Peanuts", "Penicillin"],
  "chronic_conditions": ["Asthma"],
  "current_medications": ["Albuterol"]
}
```

Example:

```bash
curl -X PUT http://localhost:8080/api/patient/{id}/medical-alerts \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '{
    "blood_type": "O+",
    "allergies": ["Peanuts", "Penicillin"],
    "chronic_conditions": ["Asthma"],
    "current_medications": ["Albuterol"]
  }'
```

Success:

- `200 OK`
- `404 Not Found` if patient missing/inactive

### PUT /patient/{id}/contact

Update patient contact information.

Request body:

```json
{
  "phone": "+1222222",
  "email": "updated_oruns@outlook.com",
  "address": "New House 1B, Jai Crescent",
  "emergency_contact_name": "Flavian Oruns",
  "emergency_contact_phone": "+127833"
}
```

Example:

```bash
curl -X PUT http://localhost:8080/api/patient/{id}/contact \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '{
    "phone": "+1222222",
    "email": "updated_oruns@outlook.com",
    "address": "New House 1B, Jai Crescent",
    "emergency_contact_name": "Flavian Oruns",
    "emergency_contact_phone": "+127833"
  }'
```

Success:

- `200 OK`
- `400 Bad Request` if request data is invalid

## Doctor service

### HEAD /doctor/{id}

Checks whether a doctor exists and is active.

Behavior:

- `204 No Content` for an active doctor
- `404 Not Found` when the doctor is missing or inactive
- `500 Internal Server Error` on DB lookup failure

Example:

```bash
curl -I http://localhost:8080/api/doctor/{id} \
  -H "x-api-version: 1"
```

### POST /doctor

Creates a doctor.

Request body:

```json
{
  "name": "Dr. Smith",
  "specialties": ["gp", "derm"],
  "license_num": "MED-12345"
}
```

Validation rules:

- `name`: length 3 to 100
- `specialties`: valid specialties only; custom validation is applied
- `license_num`: length 5 to 20

Example:

```bash
curl -X POST http://localhost:8080/api/doctor \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '{
    "name": "Dr. Smith",
    "specialties": ["gp", "derm"],
    "license_num": "MED-12345"
  }'
```

Success:

- `201 Created`
- body is the generated doctor ID string

Example response:

```json
"doc_5b2e7b0e91be2d22d8c7"
```

### GET /doctor/{id}

Retrieve a single active doctor.

Example:

```bash
curl http://localhost:8080/api/doctor/{id} \
  -H "x-api-version: 1"
```

Typical response:

```json
{
  "_id": "doc_5b2e7b0e91be2d22d8c7",
  "name": "Dr. Smith",
  "specialties": ["gp", "derm"],
  "license_num": "MED-12345",
  "is_active": true
}
```

### GET /doctor

List active doctors.

Example:

```bash
curl "http://localhost:8080/api/doctor?page=1&limit=10" \
  -H "x-api-version: 1"
```

### GET /doctor/inactive

List inactive doctors.

Example:

```bash
curl "http://localhost:8080/api/doctor/inactive?page=1&limit=10" \
  -H "x-api-version: 1"
```

### DELETE /doctor/{id}

Deactivate a doctor without deleting the record.

Example:

```bash
curl -X DELETE http://localhost:8080/api/doctor/{id} \
  -H "x-api-version: 1"
```

Success:

- `204 No Content`
- `404 Not Found` if the doctor does not exist or is already inactive

### PATCH /doctor/{id}/enable

Activate a doctor.

Example:

```bash
curl -X PATCH http://localhost:8080/api/doctor/{id}/enable \
  -H "x-api-version: 1"
```

Success:

- `204 No Content`
- `404 Not Found` if doctor missing or already enabled

### PATCH /doctor/{id}/disable-doctor

Disable a doctor.

Example:

```bash
curl -X PATCH http://localhost:8080/api/doctor/{id}/disable-doctor \
  -H "x-api-version: 1"
```

Success:

- `204 No Content`
- `404 Not Found` if doctor missing or already inactive

### POST /doctor/{id}/create-doctor-schedule

Create one or more doctor schedule slots. This is the schedule creation endpoint used before booking appointments.

Request body:

```json
[
  {
    "start_time": "2028-10-01T08:00:00Z",
    "minutes": 30
  },
  {
    "start_time": "2028-10-01T09:00:00Z",
    "minutes": 30
  }
]
```

Validation and business rules:

- the body must be a non-empty array
- each slot must start at least 24 hours in the future
- `minutes` must be between `10` and `60`
- slots cannot overlap with one another or with existing doctor schedules
- overlap comparison uses half-open intervals, so a slot may start exactly when another ends
- conflicting requests fail with `400 Bad Request` and store none of the slots
- the overlap check is not atomic across simultaneous schedule-creation requests

Example:

```bash
curl -X POST http://localhost:8080/api/doctor/{id}/create-doctor-schedule \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '[
    {"start_time":"2028-10-01T08:00:00Z","minutes":30},
    {"start_time":"2028-10-01T09:00:00Z","minutes":30}
  ]'
```

Success response:

```json
[
  "slot_617c96cff9b9f1ddd65",
  "slot_7c33c75ebd9d1176aa0a"
]
```

The returned IDs are the slot IDs used in appointment creation.

### GET /doctor/{id}/schedule

Retrieve all schedule documents for a doctor, including booked and unbooked slots.

Example:

```bash
curl http://localhost:8080/api/doctor/{id}/schedule \
  -H "x-api-version: 1"
```

Typical response:

```json
[
  {
    "doctor_id": "doc_5b2e7b0e91be2d22d8c7",
    "_id": "slot_617c96cff9b9f1ddd65",
    "start_time": "2028-10-01T08:00:00Z",
    "end_time": "2028-10-01T08:30:00Z",
    "is_available": true,
    "created_at": "2026-11-15T18:00:00Z",
    "updated_at": null
  }
]
```

### GET /doctor/{id}/active-doctor-schedule

Retrieve all future available slots for a doctor.

Example:

```bash
curl http://localhost:8080/api/doctor/{id}/active-doctor-schedule \
  -H "x-api-version: 1"
```

Typical response:

```json
[
  {
    "_id": "slot_617c96cff9b9f1ddd65",
    "start_time": "2028-10-01T08:00:00Z",
    "end_time": "2028-10-01T08:30:00Z",
    "is_available": true,
    "created_at": "2026-11-15T18:00:00Z",
    "updated_at": null
  }
]
```

## Appointment service

### POST /appointment

Create an appointment using an available slot.

Request body:

```json
{
  "slot_id": "slot_617c96cff9b9f1ddd65",
  "doctor_id": "doc_b617df81ec12e9b6a651",
  "patient_id": "pat_56df17fc9e814a512552",
  "specialty": "derm",
  "notes": "Having some skin issues, like eczema."
}
```

Validation rules:

- `slot_id`: required, minimum length 5
- `doctor_id`: required, minimum length 5
- `patient_id`: required, minimum length 5
- `specialty`: must match a specialty on the doctor record
- `notes`: optional

Flow:

1. validates doctor exists and is active
2. validates requested specialty matches the doctor's specialty list
3. loads active available slots for that doctor
4. verifies the given `slot_id` exists and is currently available
5. marks the slot unavailable
6. creates the appointment with initial status `Scheduled`

Example:

```bash
curl -X POST http://localhost:8080/api/appointment \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '{
    "slot_id": "slot_617c96cff9b9f1ddd65",
    "doctor_id": "doc_b617df81ec12e9b6a651",
    "patient_id": "pat_56df17fc9e814a512552",
    "specialty": "derm",
    "notes": "Having some skin issues, like eczema."
  }'
```

Success:

- `201 Created`
- no response body in the handler

### GET /appointment/{id}

Retrieve an appointment by its appointment ID.

Example:

```bash
curl http://localhost:8080/api/appointment/{id} \
  -H "x-api-version: 1"
```

Typical response:

```json
{
  "_id": "app_7f3ca2ee16d70d581f1a",
  "slot_id": "slot_617c96cff9b9f1ddd65",
  "doctor_id": "doc_b617df81ec12e9b6a651",
  "patient_id": "pat_56df17fc9e814a512552",
  "start_time": "2028-10-01T08:00:00Z",
  "end_time": "2028-10-01T08:30:00Z",
  "specialty": "derm",
  "status": "Scheduled",
  "status_history": [
    {
      "status": "Scheduled",
      "changed_at": "2026-11-15T18:00:00Z",
      "reason": "Initial appointment schedule"
    }
  ],
  "notes": "Having some skin issues, like eczema."
}
```

### GET /appointment/patient/{patient_id}

List appointments for a patient.

Example:

```bash
curl "http://localhost:8080/api/appointment/patient/{patient_id}?page=1&limit=10" \
  -H "x-api-version: 1"
```

### GET /appointment/doctor/{doctor_id}

List appointments for a doctor.

Example:

```bash
curl "http://localhost:8080/api/appointment/doctor/{doctor_id}?page=1&limit=10" \
  -H "x-api-version: 1"
```

### PATCH /appointment/{appointment_id}/confirm

Confirm a scheduled appointment.

Request body:

```json
{
  "reason": "Appointment confirmed by doctor."
}
```

Example:

```bash
curl -X PATCH http://localhost:8080/api/appointment/{appointment_id}/confirm \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '{
    "reason": "Appointment confirmed by doctor."
  }'
```

Success:

- `200 OK`
- status changes to `Confirmed`
- a new item is appended to `status_history`

### PATCH /appointment/{appointment_id}/cancel

Cancel an appointment.

Request body:

```json
{
  "canceled_by": "doctor",
  "reason": "Doctor unavailable."
}
```

`canceled_by` must be either:

- `patient`
- `doctor`

Example:

```bash
curl -X PATCH http://localhost:8080/api/appointment/{appointment_id}/cancel \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '{
    "canceled_by": "doctor",
    "reason": "Doctor unavailable."
  }'
```

Special rule:

- if `canceled_by` is `patient`, the appointment must be canceled at least 24 hours before the appointment start time

Success:

- `200 OK`
- status changes to `Canceled`

### PATCH /appointment/{appointment_id}/complete

Mark an appointment as completed.

Request body:

```json
{
  "reason": "Consultation completed."
}
```

Example:

```bash
curl -X PATCH http://localhost:8080/api/appointment/{appointment_id}/complete \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '{
    "reason": "Consultation completed."
  }'
```

Success:

- `200 OK`
- status changes to `Completed`

### PATCH /appointment/{appointment_id}/no-show

Mark an appointment as a no-show.

Request body:

```json
{
  "reason": "Patient did not attend the appointment."
}
```

Example:

```bash
curl -X PATCH http://localhost:8080/api/appointment/{appointment_id}/no-show \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '{
    "reason": "Patient did not attend the appointment."
  }'
```

Success:

- `200 OK`
- status changes to `NoShow`

### GET /appointment/{appointment_id}/status-history

Fetch the status change history for a particular appointment.

Example:

```bash
curl http://localhost:8080/api/appointment/{appointment_id}/status-history \
  -H "x-api-version: 1"
```

Typical response:

```json
[
  {
    "status": "Scheduled",
    "changed_at": "2026-11-15T18:00:00Z",
    "reason": "Initial appointment schedule"
  },
  {
    "status": "Confirmed",
    "changed_at": "2026-11-15T18:30:00Z",
    "reason": "Appointment confirmed by doctor."
  }
]
```

## Notable implementation notes for frontend code

- `x-api-version: 1` is required on every request.
- `DELETE` operations are soft deactivation (not hard deletion) for both doctors and patients.
- A doctor must be enabled before creating a schedule and before accepting appointments.
- Appointment creation requires a valid `slot_id` returned by doctor schedule creation.
- Scheduled slots are `is_available: true` until claimed by an appointment.
- Appointment status transitions are stored in `status_history`, which is the authoritative audit trail for display.
- The doctor schedule API uses `CreateScheduleSlot` objects with `start_time` and `minutes` only.
- Patient and doctor detail endpoints are ID-based and return application IDs as string values.

## Sample frontend payloads

### Create patient

```json
{
  "name": "Dru Oruns",
  "dob": "1960-10-01",
  "gender": "male",
  "contact_info": {
    "phone": "+111111",
    "email": "oruns@outlook.com",
    "address": "House 1A, Jai Crescent",
    "emergency_contact_name": "Flavian Oruns",
    "emergency_contact_phone": "+127833"
  }
}
```

### Create doctor

```json
{
  "name": "Dr. Smith",
  "specialties": ["gp", "derm"],
  "license_num": "MED-12345"
}
```

### Create doctor schedule

```json
[
  {
    "start_time": "2028-10-01T08:00:00Z",
    "minutes": 30
  },
  {
    "start_time": "2028-10-01T09:00:00Z",
    "minutes": 30
  }
]
```

### Create appointment

```json
{
  "slot_id": "slot_617c96cff9b9f1ddd65",
  "doctor_id": "doc_b617df81ec12e9b6a651",
  "patient_id": "pat_56df17fc9e814a512552",
  "specialty": "derm",
  "notes": "Having some skin issues, like eczema."
}
```

### Cancel appointment

```json
{
  "canceled_by": "doctor",
  "reason": "Doctor unavailable."
}
```

### Appointment status update reason

```json
{
  "reason": "Appointment confirmed by doctor."
}
```

This document reflects the actual routes and payload shapes implemented by the backend and is suitable for frontend code generation and UI integration.
