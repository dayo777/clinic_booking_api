use crate::error::DoctorServiceError;
use crate::models::{DoctorDto, DoctorResponseDto};
use common::db::get_collection;
use common::models::CreateScheduleSlot;
use futures::TryStreamExt;
use mongodb::bson::{DateTime as BsonDateTime, Document, doc};
use std::time::{Duration, SystemTime};
use validator::ValidationError;

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

impl From<DoctorDto> for DoctorResponseDto {
    fn from(d: DoctorDto) -> Self {
        Self {
            doctor_id: d.doctor_id,
            name: d.name,
            specialties: d.specialties,
            license_num: d.license_num,
            is_active: d.is_active,
        }
    }
}

pub(crate) async fn collect_doctors(
    mut cursor: mongodb::Cursor<DoctorDto>,
) -> Result<Vec<DoctorResponseDto>, DoctorServiceError> {
    let mut doctors = Vec::new();
    while let Some(doctor) = cursor.try_next().await? {
        doctors.push(DoctorResponseDto::from(doctor));
    }
    Ok(doctors)
}

pub fn add_minutes_to_bson_datetime(start_time: BsonDateTime, minutes: i64) -> BsonDateTime {
    let millis_to_add = minutes * 60 * 1000;
    BsonDateTime::from_millis(start_time.timestamp_millis() + millis_to_add)
}

pub(crate) async fn check_schedule_slots_available(
    doctor_id: &str,
    slots: &[CreateScheduleSlot],
) -> Result<Vec<BsonDateTime>, DoctorServiceError> {
    let end_times: Vec<BsonDateTime> = slots
        .iter()
        .map(|slot| {
            slot.minutes
                .checked_mul(60_000)
                .and_then(|duration| slot.start_time.timestamp_millis().checked_add(duration))
                .map(BsonDateTime::from_millis)
                .ok_or_else(|| {
                    DoctorServiceError::ValidationError(
                        "Schedule slot end time exceeds the supported date range".to_string(),
                    )
                })
        })
        .collect::<Result<_, _>>()?;

    for first in 0..slots.len() {
        for second in (first + 1)..slots.len() {
            if slots[first].start_time < end_times[second]
                && slots[second].start_time < end_times[first]
            {
                return Err(DoctorServiceError::ValidationError(
                    "Schedule slots cannot overlap another requested slot".to_string(),
                ));
            }
        }
    }

    let intervals: Vec<Document> = slots
        .iter()
        .zip(&end_times)
        .map(|(slot, end_time)| {
            doc! {
                "start_time": { "$lt": end_time },
                "end_time": { "$gt": slot.start_time },
            }
        })
        .collect();
    let existing = get_collection::<Document>(crate::repository::SCHEDULE_COLLECTION)
        .find_one(doc! {
            "doctor_id": doctor_id,
            "$or": intervals,
        })
        .await?;

    if existing.is_some() {
        return Err(DoctorServiceError::ValidationError(
            "Schedule slots overlap an existing doctor schedule".to_string(),
        ));
    }

    Ok(end_times)
}
