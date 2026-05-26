use doctor_service::utils::validate_specialty;

#[test]
fn test_validate_specialty() {
    assert!(validate_specialty(String::from("gp")).is_ok());
    assert!(validate_specialty(String::from("Cardiology")).is_ok());
    assert!(validate_specialty(String::from("general practice")).is_ok());
    assert!(validate_specialty(String::from("derm")).is_ok());
    assert!(validate_specialty(String::from("Neurosurgery")).is_ok());
    assert!(validate_specialty(String::from("neuro")).is_ok());

    assert!(validate_specialty(String::from("unknown")).is_err());
    assert!(validate_specialty(String::from("invalid")).is_err());
}
