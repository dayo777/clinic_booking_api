# Clinic Booking API (Test Project)
A Rust-based microservice for managing clinic bookings, backed by MongoDB for data storage and Jaeger for distributed tracing.

---

## 🚀 Quick Start

Follow these steps to get the API running on your local machine.

### Prerequisites

Before starting, ensure you have the following installed and running:
* **Rust** (Edition 2021 or later)
* **MongoDB Atlas** account (or a local MongoDB instance) with a database named `clinic_booking_api`.
* **Jaeger** instance running and accessible from your network for tracing.

> ⚠️ **Important Network Access Notes:**
> * **MongoDB Atlas:** Ensure your current IP address is added to the IP Access List in your Atlas cluster dashboard.
> * **Jaeger Endpoint:** Confirm your host machine can reach the Jaeger ingestion endpoint.

---

### Setup & Installation

- git clone https://github.com/dayo777/clinic_booking_api.git
- cd clinic_booking_api
- rename `example_settings_dev.toml` to `settings_dev.toml`, and input the MONGODB URI, DB name, and Jaeger ingestion endpoint

Note: `settings_dev.toml` is not pushed to VCS, and is required to run this app. 

### How to Pull the Image from Docker hub

To pull this test image from Docker Hub, use:

```
docker pull dayo777/clinic-booking-api:main
```

### How to Run the pulled Image locally

Ensure `settings_dev.toml` exist in the same directory you are running this command from

```
docker run -d \
  --name clinic_api \
  -p 8080:8080 \
  -v $(pwd)/settings_dev.toml:/settings_dev.toml \
  dayo777/clinic-booking-api:main
```


> **NOTE:** Base URL: `http://localhost:8080/api`

### Quick Test Command

```
curl -X GET -H "x-api-version: 1" localhost:8080/api/
```

### Available Endpoints
For a detailed list of all available endpoints and their specifications, please refer to [endpoints.md](endpoints.md).

| Endpoint | Method | Description | 
| :--- | :--- | :--- | 
| /api | GET | Test welcome message |
| /api/doctor | GET | Mock doctors endpoint | 
| /api/patient | GET | Mock patients endpoint | 
| /api/clinic | GET | Mock clinic info endpoint |
| /api/appointments | GET | Mock appointments endpoint | 

The API uses header-based versioning. To access any of the endpoints below, you must include the `x-api-version: 1` header in your request.

### Technical Details
- Built with: Rust (Actix-web)
- Port: 8080
- Current Version: Header-based (x-api-version: 1)
- Status: WIP