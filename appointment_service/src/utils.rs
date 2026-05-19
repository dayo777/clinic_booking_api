use mongodb::bson::DateTime as BsonDateTime;
use std::time::{Duration, SystemTime};
use validator::ValidationError;

// check to make sure the booking date is at least 24 hours in the future
#[allow(dead_code)]
pub(crate) fn check_date_is_24hr_in_future(
    booking_date: &BsonDateTime,
) -> Result<(), ValidationError> {
    let twenty_four_hour_from_now =
        BsonDateTime::from_system_time(SystemTime::now() + Duration::from_secs(86400));

    if *booking_date < twenty_four_hour_from_now {
        return Err(ValidationError::new(
            "Booking date must be at least 24 hours in the future",
        ));
    }

    Ok(())
}
