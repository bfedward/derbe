use chrono::Datelike;
use chrono::{Duration, NaiveDate, Weekday};

use crate::model::leave_request::HalfDayOptions;

pub fn naive_date_to_yyyy_mm_dd(naive_date: &NaiveDate) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        naive_date.year(),
        naive_date.month(),
        naive_date.day()
    )
}

pub fn naive_date_to_yyyy_mm_dd_this_year_start(naive_date: &NaiveDate) -> String {
    format!("{:04}-{:02}-{:02}", naive_date.year(), 1, 1)
}

pub fn naive_date_to_yyyy_mm_dd_next_year_end(naive_date: &NaiveDate) -> String {
    format!("{:04}-{:02}-{:02}", naive_date.year() + 1, 12, 31)
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
    match half_day_op {
        HalfDayOptions::FullDays => {}
        HalfDayOptions::FirstDayHalf | HalfDayOptions::LastDayHalf => duration_in_days -= 0.5,
        HalfDayOptions::BothHalfDays => duration_in_days -= 1.0,
    }

    // Ensure duration_in_days is non-negative
    if duration_in_days < 0.0 {
        duration_in_days = 0.0;
    }

    duration_in_days
}
