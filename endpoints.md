# Project Endpoints

> **Note:** Before testing the endpoints, ensure that the **Jaeger** endpoint (for logs) and the **MongoDB** database are running. You can refer to `example_settings_dev.toml` for an example of the required settings.

This document lists the currently registered endpoints in the Clinic Booking API.

## Base URL

`http://localhost:8080/api`

All endpoints require the header `x-api-version: 1`.

## Service Endpoints

- `GET /api` - API health/welcome message

### Patient Service

- `HEAD /api/patient/{id}` - Check whether a patient exists
- `GET /api/patient` - List active patients; supports optional `page` and `limit` query parameters
- `POST /api/patient` - Register a patient
- `GET /api/patient/{id}` - Retrieve a patient
- `DELETE /api/patient/{id}` - Archive a patient
- `PUT /api/patient/{id}/insurance` - Update patient insurance information
- `PUT /api/patient/{id}/medical-alerts` - Update patient medical alerts
- `PUT /api/patient/{id}/contact` - Update patient contact information

### Doctor Service

- `HEAD /api/doctor/{id}` - Check whether a doctor exists
- `POST /api/doctor` - Create a doctor
- `GET /api/doctor/{id}` - Retrieve a doctor
- `GET /api/doctor` - List active doctors; supports optional `page` and `limit` query parameters
- `GET /api/doctor/inactive` - List inactive doctors; supports optional `page` and `limit` query parameters
- `PATCH /api/doctor/{id}/enable` - Re-enable an inactive doctor
- `DELETE /api/doctor/{id}` - Archive/deactivate a doctor
- `POST /api/doctor/{id}/create-doctor-schedule` - Create schedule slots for a doctor
- `GET /api/doctor/{id}/active-doctor-schedule` - Retrieve a doctor's active schedule slots

### Appointment Service

- `POST /api/appointment` - Create an appointment
- `GET /api/appointment/{id}` - Retrieve an appointment
- `GET /api/appointment/patient/{patient_id}` - List appointments for a patient; supports optional `page` and `limit` query parameters
- `GET /api/appointment/doctor/{doctor_id}` - List appointments for a doctor; supports optional `page` and `limit` query parameters
- `PATCH /api/appointment/{appointment_id}/confirm` - Confirm an appointment
- `PATCH /api/appointment/{appointment_id}/cancel` - Cancel an appointment
- `PATCH /api/appointment/{appointment_id}/complete` - Mark an appointment as completed
- `PATCH /api/appointment/{appointment_id}/no-show` - Mark an appointment as no-show

## Command-Line Examples

Every example below includes the required API-version header.

### Homepage

```bash
curl -X GET http://localhost:8080/api \
  -H "x-api-version: 1"
```

---

## Patient Service

### Check Patient Existence

Returns `204 No Content` when the patient exists and `404 Not Found` otherwise.

```bash
curl -I http://localhost:8080/api/patient/{id} \
  -H "x-api-version: 1"
```

### Create Patient

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

### Retrieve a Patient

```bash
curl -X GET http://localhost:8080/api/patient/{id} \
  -H "x-api-version: 1"
```

### List Patients with Pagination

```bash
curl -X GET "http://localhost:8080/api/patient?page=1&limit=10" \
  -H "x-api-version: 1"
```

### Update Patient Insurance

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

### Update Patient Medical Alerts

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

### Update Patient Contact Information

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

### Archive Patient

```bash
curl -X DELETE http://localhost:8080/api/patient/{id} \
  -H "x-api-version: 1"
```

---

## Doctor Service

### Check Doctor Existence

Returns `204 No Content` when the doctor exists and `404 Not Found` otherwise.

```bash
curl -I http://localhost:8080/api/doctor/{id} \
  -H "x-api-version: 1"
```

### Create Doctor

Supported specialties are validated by the service. Examples include `gp`, `derm`, `neuro`, and `cardio`.

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

### Retrieve a Doctor

```bash
curl -X GET http://localhost:8080/api/doctor/{id} \
  -H "x-api-version: 1"
```

### List Doctors with Pagination

```bash
curl -X GET "http://localhost:8080/api/doctor?page=1&limit=10" \
  -H "x-api-version: 1"
```

### List Inactive Doctors

```bash
curl -X GET "http://localhost:8080/api/doctor/inactive?page=1&limit=10" \
  -H "x-api-version: 1"
```

### Enable Doctor

```bash
curl -X PATCH http://localhost:8080/api/doctor/{id}/enable \
  -H "x-api-version: 1"
```

### Archive Doctor

```bash
curl -X DELETE http://localhost:8080/api/doctor/{id} \
  -H "x-api-version: 1"
```

### Create Doctor Schedule

The request body is an array of schedule slots.

```bash
curl -X POST http://localhost:8080/api/doctor/{id}/create-doctor-schedule \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '[
    {
      "start_time": "2026-10-01T08:00:00Z",
      "end_time": "2026-10-01T08:30:00Z"
    },
    {
      "start_time": "2026-10-01T09:00:00Z",
      "end_time": "2026-10-01T09:30:00Z"
    }
  ]'
```

### Retrieve Active Doctor Schedule

```bash
curl -X GET http://localhost:8080/api/doctor/{id}/active-doctor-schedule \
  -H "x-api-version: 1"
```

---

## Appointment Service

### Create an Appointment

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

### Retrieve an Appointment

```bash
curl -X GET http://localhost:8080/api/appointment/{id} \
  -H "x-api-version: 1"
```

### List Appointments for a Patient

```bash
curl -X GET "http://localhost:8080/api/appointment/patient/{patient_id}?page=1&limit=10" \
  -H "x-api-version: 1"
```

### List Appointments for a Doctor

```bash
curl -X GET "http://localhost:8080/api/appointment/doctor/{doctor_id}?page=1&limit=10" \
  -H "x-api-version: 1"
```

The appointment list endpoints also work without query parameters.

### Confirm an Appointment

The status update body accepts an optional `reason` field. The current handler expects this field to be present.

```bash
curl -X PATCH http://localhost:8080/api/appointment/{appointment_id}/confirm \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '{"reason": "Appointment confirmed by doctor."}'
```

### Cancel an Appointment

```bash
curl -X PATCH http://localhost:8080/api/appointment/{appointment_id}/cancel \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '{"reason": "Doctor unavailable."}'
```

### Complete an Appointment

```bash
curl -X PATCH http://localhost:8080/api/appointment/{appointment_id}/complete \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '{"reason": "Consultation completed."}'
```

### Mark an Appointment as No-Show

```bash
curl -X PATCH http://localhost:8080/api/appointment/{appointment_id}/no-show \
  -H "Content-Type: application/json" \
  -H "x-api-version: 1" \
  -d '{"reason": "Patient did not attend the appointment."}'
```
