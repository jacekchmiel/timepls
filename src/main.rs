#[macro_use]
extern crate clap;

use anyhow::Error;
use chrono::{TimeZone, Utc};
use serde_json::{Number, Value};
use std::io::{BufRead, Write};

fn main() -> Result<(), Error> {
    let _ = clap::app_from_crate!().get_matches();
    let keys = vec!["time", "t", "ts", "timestamp"];
    let output = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap_or_default();
            match serde_json::from_str(&line).ok() {
                Some(value) => process_rec(&keys, None, &value).to_string(),
                None => {
                    let mut space_iter = line.split(' ').into_iter();
                    let header = space_iter
                        .next()
                        .map(|h| try_convert_time_from_str(h).unwrap_or_else(|| h.to_string()));
                    let tail_raw = space_iter.collect::<Vec<_>>().join(" ");
                    let tail = serde_json::from_str(&tail_raw)
                        .ok()
                        .map(|v| process_rec(&keys, None, &v));
                    match (header, tail) {
                        (Some(header), Some(tail)) => format!("{} {}", header, tail),
                        (Some(header), None) if !header.is_empty() => {
                            format!("{} {}", header, tail_raw)
                        }
                        (Some(_), None) => tail_raw,
                        (None, _) => String::new(), // No header means that the input string was empty
                    }
                }
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    write!(std::io::stdout(), "{}", output)?;
    Ok(())
}

fn process_rec(keys: &[&str], key: Option<&str>, value: &Value) -> Value {
    match (key, value) {
        (Some(key), Value::Number(n)) if keys.contains(&key.to_ascii_lowercase().as_str()) => {
            try_convert_time(n)
        }
        (None, Value::Number(n)) => try_convert_time(n),
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
        Some(n) => convert_i64(n).into(),
        None => Value::Number(n.clone()),
    }
}

fn try_convert_time_from_str(s: &str) -> Option<String> {
    match s.parse::<i64>() {
        Ok(n) => Some(format!("\"{}\"", convert_i64(n))),
        Err(_) => None,
    }
}

fn convert_i64(n: i64) -> String {
    if n < SECONDS_UPPER_LIMIT {
        Utc.timestamp(n, 0).to_string()
    } else {
        Utc.timestamp(n / 1000, (n % 1000) as u32 * 1000_000)
            .to_string()
    }
}
