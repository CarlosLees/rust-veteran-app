mod http_result;
mod server_config;

use chrono::{DateTime, NaiveDateTime, Utc};
pub use http_result::HttpResult;
use serde::{Deserialize, Deserializer, Serializer};
pub use server_config::app_config::AppConfig;

pub fn serialize_datetime_with_option<S>(
    datetime: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match datetime {
        Some(dt) => serializer.serialize_str(&dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn serialize_datetime<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&datetime.format("%Y-%m-%d %H:%M:%S").to_string())
}

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let datetime_result = Option::<String>::deserialize(deserializer);

    match datetime_result {
        Ok(datetime) => match datetime {
            Some(time) => {
                if !time.is_empty() {
                    let naive_datetime = NaiveDateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S")
                        .map_err(serde::de::Error::custom)?;
                    Ok(Some(DateTime::<Utc>::from_naive_utc_and_offset(
                        naive_datetime,
                        Utc,
                    )))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        },
        Err(_) => Ok(None),
    }
}
