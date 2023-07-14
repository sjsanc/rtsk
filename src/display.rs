use chrono::{DateTime, Duration, Utc};
use colored::Colorize;
use tabled::{
    settings::{style::BorderColor, Color, Style},
    Table,
};

use crate::{
    logic::{Priority, Task},
    utils::format_time_difference,
};

// Display functions for Tabled

pub fn display_due_date(o: &Option<DateTime<Utc>>) -> String {
    match o {
        Some(due_date) => {
            let current_time = Utc::now();
            let duration = due_date.signed_duration_since(current_time);

            if duration < Duration::hours(1) {
                let minutes = duration.num_minutes();
                format!("{}m", minutes)
            } else if duration < Duration::hours(24) {
                let hours = duration.num_hours();
                format!("{}h", hours)
            } else {
                let days = duration.num_days();
                format!("{}d", days)
            }
        }
        None => String::new(),
    }
}

pub fn display_priority(p: &Priority) -> String {
    match p {
        Priority::Now => format!("{}", "Now".bright_red().bold()),
        Priority::High => format!("{}", "High".red()),
        Priority::Low => format!("{}", "Low"),
    }
}

pub fn display_age(t: &DateTime<Utc>) -> String {
    format_time_difference(*t, Utc::now())
}

pub fn display_tags(tags: &Vec<String>) -> String {
    let mut tags_str = String::new();

    for tag in tags {
        tags_str.push_str(&format!("{} ", tag));
    }

    tags_str
}

pub fn print_tasks(tasks: &Vec<Task>, color: Option<Color>) {
    let mut table = Table::new(tasks);
    table.with(Style::rounded());

    match color {
        Some(color) => {
            table.with(BorderColor::default().top(color.clone()));
            table.with(BorderColor::default().left(color.clone()));
            table.with(BorderColor::default().bottom(color.clone()));
            table.with(BorderColor::default().right(color.clone()));
            table.with(BorderColor::default().corner_bottom_left(color.clone()));
            table.with(BorderColor::default().corner_bottom_right(color.clone()));
            table.with(BorderColor::default().corner_top_left(color.clone()));
            table.with(BorderColor::default().corner_top_right(color.clone()));
        }
        None => {}
    }

    println!("{}", table);
}
