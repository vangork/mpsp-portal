use base64::{Engine as _, engine::general_purpose};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, de::Deserializer};
use std::env;

pub fn js_date_to_naive_datetime<'de, D>(d: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    //Ok(NaiveDateTime::deserialize(d)?.into())
    Ok(DateTime::<Utc>::deserialize(d)?.naive_utc())
}

pub fn to_ascii_lowercase<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.to_ascii_lowercase())
}

pub fn get_secret_key() -> Vec<u8> {
    let secret_key_base64 = env::var("SECRET_KEY").expect("SECRET_KEY is not set in .env file");
    general_purpose::STANDARD
        .decode(secret_key_base64)
        .expect("SECRET_KEY must be valid base64")
}
