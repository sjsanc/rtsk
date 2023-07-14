use chrono::{DateTime, NaiveDate, Utc};

use crate::logic::Priority;

// =================================================================================
// TIME UTILS
// =================================================================================

// pub fn get_current_time() -> u64 {
//     let now = std::time::SystemTime::now();
//     let timestamp = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
//     timestamp
// }

// pub fn convert_to_unix_timestamp(date_str: &str) -> Result<i64, chrono::ParseError> {
//     let date = NaiveDateTime::parse_from_str(date_str, "%d/%m/%Y")?;
//     let timestamp = date.timestamp();
//     Ok(timestamp)
// }

pub fn format_time_difference(start: DateTime<Utc>, end: DateTime<Utc>) -> String {
    let duration = end.signed_duration_since(start);

    if duration.num_hours() > 0 {
        // Format as hours
        format!("{}h", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        // Format as minutes
        format!("{}m", duration.num_minutes())
    } else {
        // Format as seconds
        format!("{}s", duration.num_seconds())
    }
}

// =================================================================================
// TASK UTILS
// =================================================================================

pub fn get_priority_or_default(priority: &Option<String>) -> Priority {
    match priority {
        Some(s) => match s.as_str() {
            "now" | "Now" | "n" | "N" => Priority::Now,
            "high" | "High" | "h" | "H" => Priority::High,
            "low" | "Low" | "l" | "L" => Priority::Low,
            _ => Priority::Low,
        },
        None => Priority::Low,
    }
}

pub fn get_due_or_default(due: &Option<String>) -> Option<DateTime<Utc>> {
    match due {
        Some(s) => match parse_date_from_str(&s) {
            Ok(d) => Some(d),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn get_tags_or_default(tags: &Option<String>) -> Vec<String> {
    match tags {
        Some(s) => s.split(",").map(|s| s.to_string()).collect(),
        None => Vec::new(),
    }
}

pub fn parse_date_from_str(date_str: &String) -> Result<DateTime<Utc>, chrono::ParseError> {
    let naive_date = NaiveDate::parse_from_str(&date_str, "%d/%m/%Y")?;
    let naive_datetime = naive_date.and_hms_opt(0, 0, 0);
    let datetime = DateTime::<Utc>::from_utc(naive_datetime.unwrap(), Utc);
    Ok(datetime)
}
