use chrono::{DateTime, Utc};
use mongodb::bson::{Bson, DateTime as BsonDateTime};
use serde::{Deserialize, Deserializer};

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
