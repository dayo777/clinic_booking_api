# Project Endpoints

This document lists the primary endpoints available in the Clinic Booking API.

## Base URL
All API routes are prefixed with `/api`.
Required header: `x-api-version: 1`

## Service Endpoints

### Patient Service
- `GET /api/patient` - List all patients
- `POST /api/patient` - Register a new patient
- `GET /api/patient/{id}` - Get patient details
- `PUT /api/patient/{id}` - Update patient details
- `DELETE /api/patient/{id}` - Delete/Archive patient

### Doctor Service
- `GET /api/doctor` - List all doctors
- `POST /api/doctor` - Create a new doctor
- `GET /api/doctor/{id}` - Get doctor details
- `PUT /api/doctor/{id}` - Update doctor details
- `DELETE /api/doctor/{id}` - Delete/Archive doctor

### Clinic Service
- `GET /api/clinic` - List all clinics
- `POST /api/clinic` - Create a new clinic
- `GET /api/clinic/{id}` - Get clinic details
- `PUT /api/clinic/{id}` - Update clinic details
- `DELETE /api/clinic/{id}` - Delete/Archive clinic

### Appointment Service
- `GET /api/appointments` - List all appointments
