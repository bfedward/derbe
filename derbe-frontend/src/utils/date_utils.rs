use chrono::Datelike;
use chrono::{Duration, NaiveDate, Weekday};

use crate::model::leave_request::HalfDayOptions;

pub fn naive_date_to_yyyy_mm_dd(naive_date: &NaiveDate) -> String {
    naive_date.format("%Y-%m-%d").to_string()
}

pub fn naive_date_to_yyyy_mm_dd_this_year_start(naive_date: &NaiveDate) -> String {
    naive_date_to_yyyy_mm_dd(
        &NaiveDate::from_ymd_opt(naive_date.year(), 1, 1)
            .expect("Could not create 1st of Jan date"),
    )
}

pub fn naive_date_to_yyyy_mm_dd_next_year_end(naive_date: &NaiveDate) -> String {
    naive_date_to_yyyy_mm_dd(
        &NaiveDate::from_ymd_opt(naive_date.year() + 1, 12, 31)
            .expect("Could not create 31st of Dec date"),
    )
}

pub fn calculate_requested_days(
    date_from: &NaiveDate,
    date_to: &NaiveDate,
    half_day_op: &HalfDayOptions,
) -> f64 {
    let mut duration_in_days = 0.0;

    let mut current_date = *date_from;
    while current_date <= *date_to {
        if current_date.weekday() != Weekday::Sat && current_date.weekday() != Weekday::Sun {
            duration_in_days += 1.0;
        }
        current_date += Duration::days(1);
    }

    // Adjust for half day options
    duration_in_days -= half_day_op.to_f64();

    // Ensure duration_in_days is non-negative
    if duration_in_days < 0.0 {
        duration_in_days = 0.0;
    }

    duration_in_days
}

pub fn format_my_leave_date(naive_date: &NaiveDate) -> String {
    let day_ending = match naive_date.day() {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        4..=20 | 24..=30 => "th",
        _ => panic!("Unknown day: {}", naive_date.day()),
    };

    // https://docs.rs/chrono/latest/chrono/format/strftime/index.html

    format!(
        "{}{}{}",
        naive_date.format("%A, %e"),
        day_ending,
        naive_date.format(" of %B, %Y")
    )
}
