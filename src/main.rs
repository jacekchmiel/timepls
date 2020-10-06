#[macro_use]
extern crate clap;

use anyhow::Error;
use chrono::{TimeZone, Utc};
use serde_json::{Number, Value};

fn main() -> Result<(), Error> {
    let _ = clap::app_from_crate!().get_matches();
    let keys = vec!["time", "t", "ts", "timestamp"];
    let value = serde_json::from_reader(std::io::stdin())?;
    let processed = process_rec(&keys, None, &value);
    serde_json::to_writer(std::io::stdout(), &processed)?;
    Ok(())
}

fn process_rec(keys: &[&str], key: Option<&str>, value: &Value) -> Value {
    match (key, value) {
        (Some(key), Value::Number(n)) if keys.contains(&key.to_ascii_lowercase().as_str()) => {
            try_convert_time(n)
        }
        (_, Value::Object(obj)) => obj
            .into_iter()
            .map(|(k, v)| (k.clone(), process_rec(keys, Some(k.as_str()), &v)))
            .collect::<serde_json::Map<String, Value>>()
            .into(),
        _ => value.clone(),
    }
}

const SECONDS_UPPER_LIMIT: i64 = 32503680000; // Values that big are assumed to be in milliseconds

fn try_convert_time(n: &Number) -> Value {
    match n.as_i64() {
        Some(n) if n < SECONDS_UPPER_LIMIT => Utc.timestamp(n, 0).to_string().into(),
        Some(n) => Utc
            .timestamp(n / 1000, (n % 1000) as u32 * 1000_000)
            .to_string()
            .into(),
        None => Value::Number(n.clone()),
    }
}
