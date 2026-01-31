use chrono::NaiveDate;
use patient_service::utils::calculate_age;

#[test]
fn test_calculate_age() {
    let dob = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    let age = calculate_age(dob);
    assert_eq!(age, 25);
}
