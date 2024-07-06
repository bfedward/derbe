use chrono::Datelike;
use chrono::NaiveDate;

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

pub fn get_first_and_last_date(year: i32, month: u32) -> (NaiveDate, NaiveDate) {
    // First date of the month
    let first_date = NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid year/month");

    // Last date of the month
    let last_date = {
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };
        NaiveDate::from_ymd_opt(next_year, next_month, 1)
            .expect("Invalid next year/month")
            .pred_opt()
            .expect("Invalid previous day")
    };

    (first_date, last_date)
}
