use crate::models::Specialty;
use chrono::{DateTime, Utc};
use mongodb::bson::{Bson, DateTime as BsonDateTime};
use serde::{Deserialize, Deserializer};
use std::fmt::Write;
use std::str::FromStr;
use validator::ValidationError;

// Deserializes a BSON datetime field that may be stored as either BSON DateTime or an ISO string
// (e.g., from Chrono default Serde, which serializes DateTime as String). Accepts both so that
// existing documents in MongoDB continue to work.
pub fn deserialize_bson_datetime_or_string<'de, D>(
    deserializer: D,
) -> Result<BsonDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let bson_val = Bson::deserialize(deserializer)?;
    match bson_val {
        Bson::DateTime(dt) => Ok(dt),
        Bson::String(s) => {
            let chrono_dt: DateTime<Utc> = s
                .parse()
                .map_err(|e: chrono::ParseError| serde::de::Error::custom(e))?;
            Ok(BsonDateTime::from_millis(chrono_dt.timestamp_millis()))
        }
        other => Err(serde::de::Error::custom(format!(
            "expected DateTime or string, got {:?}",
            other
        ))),
    }
}

/// Same as above for Option<BsonDateTime> (updated_at, deleted_at).
pub fn deserialize_option_bson_datetime_or_string<'de, D>(
    deserializer: D,
) -> Result<Option<BsonDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<Bson>::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(Bson::DateTime(dt)) => Ok(Some(dt)),
        Some(Bson::String(s)) => {
            let chrono_dt: DateTime<Utc> = s
                .parse()
                .map_err(|e: chrono::ParseError| serde::de::Error::custom(e))?;
            Ok(Some(BsonDateTime::from_millis(
                chrono_dt.timestamp_millis(),
            )))
        }
        Some(other) => Err(serde::de::Error::custom(format!(
            "expected DateTime or string, got {:?}",
            other
        ))),
    }
}

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

pub fn validate_specialty(value: &Specialty) -> Result<(), ValidationError> {
    match Specialty::from_str(value.to_string().as_str()) {
        Ok(Specialty::Other(_)) | Err(_) => Err(ValidationError::new("invalid specialty")),
        Ok(_) => Ok(()),
    }
}

// use this to generate generic IDs for each collection e.g. doc_182178dh, app_812nd89213
// helps with easy identification from a glance
pub fn generate_id(prefix: &str, total_length: u8) -> String {
    let total_length = total_length as usize;
    let suffix_length = total_length.saturating_sub(prefix.len() + 1);

    if suffix_length == 0 {
        return format!("{}_{}", prefix, "");
    }

    // let bytes_needed = (suffix_length + 1) / 2;
    let bytes_needed = suffix_length.div_ceil(2); // uncomment the top line if this gives an issue
    let mut buf = vec![0u8; bytes_needed];

    rand::fill(&mut buf[..]); // ← free function in 0.10, no imports needed

    let mut hex = String::with_capacity(bytes_needed * 2);
    for b in buf {
        write!(&mut hex, "{:02x}", b).expect("writing to string failed");
    }

    let suffix = &hex[..suffix_length];
    format!("{}_{}", prefix, suffix)
}
