use chrono::{NaiveDate, Utc};
use tracing::{debug, instrument};

// calculate Patient Age from DOB -- format: %Y-%m-%d
#[instrument(name = "calc_patient_age", level = "debug")]
pub fn calculate_age(dob: NaiveDate) -> u8 {
    let now = Utc::now().date_naive();
    debug!(?dob, ?now, "Calculating age for medical record");
    let duration = now - dob;
    let age = duration.num_days() / 365;
    age as u8
}
