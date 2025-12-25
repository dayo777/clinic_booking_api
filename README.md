# Clinic Booking API

A modular, high-performance backend API for a hospital/clinic appointment booking system, built in Rust using `actix-web` and MongoDB.

This project is organized as a Cargo workspace with clean separation of concerns. Each crate represents a bounded context or shared utility.

## Workspace Structure

### `clinic_core` (binary crate)
- The main entry point of the application.
- Responsible for:
    - Starting the Actix-web HTTP server.
    - Loading configuration (environment variables, etc.).
    - Initializing shared resources (MongoDB connection pool).
    - Registering all service routes under `/api/v1`.
- Does **not** contain business logic — only orchestration and setup.

### `patient_service` (library crate)
- Handles everything related to patients.
- Features:
    - Patient registration and authentication.
    - Profile retrieval and updates.
    - Listing a patient's own appointments.
- Contains:
    - Models, validation, handlers, and repository functions specific to patients.

### `doctor_service` (library crate)
- Manages doctor-related operations.
- Features:
    - Listing doctors (with optional filters by specialty or clinic).
    - Retrieving doctor details.
    - Checking doctor availability for a given date/clinic.
- Contains domain logic and data access for doctors.

### `clinic_service` (library crate)
- Handles clinic data.
- Features:
    - Listing all clinics.
    - Retrieving clinic details.
- Simple CRUD-like read operations (clinics are likely semi-static data).

### `appointment_service` (library crate)
- Core business logic for booking and managing appointments.
- Features:
    - Creating new appointments (with conflict checking).
    - Listing appointments (for patients or doctors).
    - Rescheduling, cancelling, and retrieving appointment details.
    - Availability slot calculation.
- Enforces rules like no double-booking and future-only appointments.

### `common` (library crate)
- Shared utilities and infrastructure used across all other crates.
- Contains:
    - Database connection and client wrappers.
    - Shared error types and result handling.
    - Common models (e.g., ObjectId wrappers, timestamps).
    - Authentication middleware and JWT utilities.
    - Reusable traits, macros, or configuration structs.
- **No business logic** — only technical concerns.

## Architecture Principles
- **Modular monolith**: All services live in one repository and binary but are strictly separated.
- **Dependency direction**: Services depend only on `common`. No circular dependencies.
- **Scalability path**: Individual crates can later be extracted into microservices if needed.

## Running the Project
```bash
cargo run -p clinic_core