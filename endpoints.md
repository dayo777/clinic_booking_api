# Project Endpoints

> **Note:** Before testing the endpoints, ensure that the **Jaeger** endpoint (for logs) and the **MongoDB** database are running. You can refer to `settings_dev.toml` for an example of how the configuration should be structured.

This document lists the primary endpoints available in the Clinic Booking API.

## Base URL
All API routes are prefixed with `/api`.
Required header: `x-api-version: 1`

## Service Endpoints

### Patient Service
- `HEAD /api/patient/{id}` - Check if patient exists
- `GET /api/patient` - List all patients (supports `page` and `limit` query params)
- `POST /api/patient` - Register a new patient
- `GET /api/patient/{id}` - Get patient details
- `DELETE /api/patient/{id}` - Delete/Archive patient
- `PUT /api/patient/{id}/insurance` - Update patient insurance information
- `PUT /api/patient/{id}/medical-alerts` - Update patient medical alerts
- `PUT /api/patient/{id}/contact` - Update patient contact information

### Doctor Service
- `HEAD /api/doctor/{id}` - Check if doctor exists
- `GET /api/doctor` - List all doctors (supports `page` and `limit` query params)
- `POST /api/doctor` - Create a new doctor
- `GET /api/doctor/{id}` - Get doctor details
- `DELETE /api/doctor/{id}` - Delete/Archive doctor
- `PATCH /api/doctor/{id}/enable` - Re-enable an inactive doctor

### Clinic Service
- `GET /api/clinic` - List all clinics (Mock)
- `POST /api/clinic` - Create a new clinic (Mock)
- `GET /api/clinic/{id}` - Get clinic details (Mock)
- `PUT /api/clinic/{id}` - Update clinic details (Mock)
- `DELETE /api/clinic/{id}` - Delete/Archive clinic (Mock)

### Appointment Service
- `GET /api/appointments` - List all appointments


## Available Endpoints

#### Create Patient
To create a new patient, use the following `curl` command.

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

#### Retrieve a Single Patient
To return a single Patient data, pass the patient's MongoDB `ObjectId` as a path parameter.

```bash
curl -X GET http://localhost:8080/api/patient/65c2a1b2e4b0a12345678901 \
     -H "x-api-version: 1"
```

#### List Patients with Pagination
To list patients with pagination, use the `page` and `limit` query parameters.

```bash
curl -X GET "http://localhost:8080/api/patient?page=1&limit=10" \
     -H "x-api-version: 1"
```

#### Update Patient Insurance
To update the insurance information for an existing patient.

```bash
curl -X PUT http://localhost:8080/api/patient/65c2a1b2e4b0a12345678901/insurance \
     -H "Content-Type: application/json" \
     -H "x-api-version: 1" \
     -d '{
           "provider_name": "HealthShield",
           "policy_number": "HS-987654321",
           "group_number": "G-112233",
           "primary_holder_name": "Dru Oruns"
         }'
```

#### Update Patient Medical Alerts
To update medical alerts (allergies, chronic conditions, etc.) for a patient.

```bash
curl -X PUT http://localhost:8080/api/patient/65c2a1b2e4b0a12345678901/medical-alerts \
     -H "Content-Type: application/json" \
     -H "x-api-version: 1" \
     -d '{
           "blood_type": "O+",
           "allergies": ["Peanuts", "Penicillin"],
           "chronic_conditions": ["Asthma"],
           "current_medications": ["Albuterol"]
         }'
```

#### Update Patient Contact Info
To update the contact information (phone, email, address, etc.) for an existing patient.

```bash
curl -X PUT http://localhost:8080/api/patient/65c2a1b2e4b0a12345678901/contact \
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

#### Create Doctor
To create a new doctor.

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

#### Retrieve a Single Doctor
To return a single doctor's data.

```bash
curl -X GET http://localhost:8080/api/doctor/65c2a1b2e4b0a12345678901 \
     -H "x-api-version: 1"
```

#### List Doctors with Pagination
To list doctors with pagination, use the `page` and `limit` query parameters.

```bash
curl -X GET "http://localhost:8080/api/doctor?page=1&limit=10" \
     -H "x-api-version: 1"
```

#### Check Doctor Existence
To check if a doctor exists without retrieving the full record.

```bash
curl -I -X HEAD http://localhost:8080/api/doctor/65c2a1b2e4b0a12345678901 \
     -H "x-api-version: 1"
```

#### Delete Doctor
To deactivate/archive a doctor.

```bash
curl -X DELETE http://localhost:8080/api/doctor/65c2a1b2e4b0a12345678901 \
     -H "x-api-version: 1"
```

#### Enable Doctor
To re-activate a doctor who was previously deactivated/deleted.

```bash
curl -X PATCH http://localhost:8080/api/doctor/65c2a1b2e4b0a12345678901/enable \
     -H "x-api-version: 1"
```
