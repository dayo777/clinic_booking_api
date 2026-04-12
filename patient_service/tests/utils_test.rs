use chrono::{Duration, NaiveDate, Utc};
use patient_service::utils::calculate_age;

// Tests that calculate_age returns a value for a fixed date in the past
#[test]
fn test_calculate_age() {
    let dob = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    let age = calculate_age(dob).unwrap();
    // As of 2026-02-07, age should be 26 (approx 26.1 years)
    assert!(age >= 26);
}

// Tests that calculate_age returns the correct age when providing a DOB 25 years in the past
#[test]
fn test_calculate_age_success() {
    let now = Utc::now().date_naive();
    let dob = now - Duration::days(365 * 25);
    assert_eq!(calculate_age(dob), Some(25));
}

// Tests that calculate_age returns None when providing a future DOB
#[test]
fn test_calculate_age_future_dob() {
    let now = Utc::now().date_naive();
    let dob = now + Duration::days(1);
    assert_eq!(calculate_age(dob), None);
}
