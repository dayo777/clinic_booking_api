use chrono::{NaiveDate, Utc};
use tracing::{info, instrument};

// calculate Patient Age from DOB -- format: %Y-%m-%d
#[instrument(skip(dob))]
pub fn calculate_age(dob: NaiveDate) -> u8 {
    info!("Calculating patient age");
    let now = Utc::now().date_naive();
    let duration = now - dob;
    let age = duration.num_days() / 365;
    info!("Returning patient age");
    age as u8
}
