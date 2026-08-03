use common::models::Specialty;
use mongodb::bson::DateTime as BsonDateTime;
use std::str::FromStr;
use std::time::{Duration, SystemTime};
use validator::ValidationError;

pub fn validate_specialties(values: &Vec<Specialty>) -> Result<(), ValidationError> {
    for value in values {
        match Specialty::from_str(value.to_string().as_str()) {
            Ok(Specialty::Other(_)) | Err(_) => {
                return Err(ValidationError::new("invalid specialty"));
            }
            Ok(_) => {}
        }
    }
    Ok(())
}

pub fn validate_specialty(value: String) -> Result<(), ValidationError> {
    match Specialty::from_str(value.as_str()) {
        Ok(Specialty::Other(_)) | Err(_) => Err(ValidationError::new("invalid specialty")),
        Ok(_) => Ok(()),
    }
}

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

//
// pub fn is_doctor_available(doctor: &DoctorDto, requested_start: DateTime<Utc>, requested_end: DateTime<Utc>) -> bool {
//     // 1. Check if the day of the week is in the schedule
//     let weekday = requested_start.weekday();
//     let day_enum = match weekday {
//         chrono::Weekday::Mon => DayOfWeek::Monday,
//         chrono::Weekday::Tue => DayOfWeek::Tuesday,
//         chrono::Weekday::Wed => DayOfWeek::Wednesday,
//         chrono::Weekday::Thu => DayOfWeek::Thursday,
//         chrono::Weekday::Fri => DayOfWeek::Friday,
//         chrono::Weekday::Sat => DayOfWeek::Saturday,
//         chrono::Weekday::Sun => DayOfWeek::Sunday,
//     };
//
//     let working_hours = doctor.schedule.weekly.iter().find(|wh| wh.day == day_enum);
//
//     if let Some(wh) = working_hours {
//         // 2. Check if the time slot falls within working hours
//         // Simplifying: convert "HH:MM" to minutes from midnight
//         let wh_start_mins = parse_time_to_minutes(&wh.start_time);
//         let wh_end_mins = parse_time_to_minutes(&wh.end_time);
//
//         let req_start_mins = requested_start.hour() * 60 + requested_start.minute();
//         let req_end_mins = requested_end.hour() * 60 + requested_end.minute();
//
//         return req_start_mins >= wh_start_mins && req_end_mins <= wh_end_mins;
//     }
//
//     false
// }
//
// pub fn has_specialty(doctor: &DoctorDto, specialty: &str) -> bool {
//     doctor.specialties.contains(&specialty.to_string())
// }
//
// fn parse_time_to_minutes(time_str: &str) -> u32 {
//     let parts: Vec<&str> = time_str.split(':').collect();
//     if parts.len() == 2 {
//         let hours = parts[0].parse::<u32>().unwrap_or(0);
//         let mins = parts[1].parse::<u32>().unwrap_or(0);
//         return hours * 60 + mins;
//     }
//     0
// }
