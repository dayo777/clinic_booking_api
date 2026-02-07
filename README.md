# Clinic Booking API (Test Project)

Note: `settings_dev.toml` is not pushed to VCS, and is required to run this app. I used K8s secret resource to create the file. 
### How to Pull the Image

To pull this test image from Docker Hub, use:

```
docker pull dayo777/clinic-booking-api:main
```

### How to Run the Image

Run the container by mapping port 8080. This service is built with Rust and Actix-web.

```
docker run -d -p 8080:8080 dayo777/clinic-booking-api:main
```


> **NOTE:** Base URL: `http://localhost:8080/api`

### Quick Test Command

```
curl -X GET -H "x-api-version: 1" http://localhost:8080/api/
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