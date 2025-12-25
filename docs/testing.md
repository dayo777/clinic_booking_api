## Testing Strategy

We follow a comprehensive, layered testing approach to ensure reliability, correctness, and maintainability.

Test runs on every push/PR. Include:
- `cargo fmt -- --check`
- `cargo clippy --workspace --all-targets --all-features`

### 1. Unit Tests
- Located in each crate's `src/` directory (using `#[cfg(test)]` modules).
- Test individual functions, handlers, and business logic in isolation.
- Use mocks (via `mockall` or manual mocks in tests) for database dependencies.
- Focus: Pure logic, model validations, error handling, and edge cases.

### 2. Integration Tests
- Placed in a top-level `tests/` directory at the workspace root (e.g., `tests/appointment_booking.rs`).
- Spin up a real Actix-web server with a **test MongoDB instance** (using testcontainers or a dedicated test database).
- Test full HTTP request/response cycles, including:
    - Authentication flows.
    - End-to-end appointment booking (conflict detection, status updates).
    - Cross-service interactions (e.g., booking requires valid patient, doctor, and clinic).
- Use unique database names or cleanup after each test to ensure isolation.

### 3. Repository Tests
- Inside each service crate (e.g., `patient_service::repository::tests`).
- Test database operations directly using a real or in-memory MongoDB instance.
- Verify CRUD operations, queries, and indexing behavior.

### 4. Common Testing Utilities
- Shared test helpers live in `common::test_utils` (if needed):
    - Test database setup/teardown.
    - JWT token generation for authenticated tests.
    - Fixture data (sample patients, doctors, clinics).

### 5. Key Testing Tools & Practices
- **actix-web::test** for HTTP testing.
- **mongodb** crate with test-specific database names.
- **serial_test** crate (optional) for tests requiring exclusive DB access.
- **env_logger** or **tracing** guards to reduce noise in test output.
- Run tests with:
  ```bash
  cargo test --workspace --all-features