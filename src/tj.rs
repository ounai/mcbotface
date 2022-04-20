use chrono::prelude::*;

pub fn get_days_until(date: &str) -> String {
    let now = Utc::now();

    let then = DateTime::parse_from_rfc3339(&[date, "T00:00:00+00:00"].concat())
        .expect("Invalid date string");

    let duration = then.signed_duration_since(now).to_std().unwrap();

    let days = (duration.as_secs() / 60 / 60 / 24) + 1;

    format!("{} days until {}", days, date)
}
