use std::ops::Deref;

use crate::{
    api::{
        api_errors::ApiError,
        employees::get_employee,
        leave_requests::{get_leave_request, update_leave_request},
        leave_types::get_leave_type,
    },
    components::organisms::navbar::Navbar,
    model::{
        employee::Employee,
        leave_request::{HalfDayOptions, LeaveRequest},
        leave_type::LeaveType,
    },
    router::Route,
};
use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveTime, Utc, Weekday};
use gloo::console::log;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component(LeaveRequestDetails)]
pub fn component(props: &Props) -> Html {
    let is_leave_request_loaded = use_state(|| false);
    let is_employee_loaded = use_state(|| false);
    let is_leave_type_loaded = use_state(|| false);
    let leave_request = use_state(LeaveRequest::default);
    let employee = use_state(Employee::default);
    let leave_type = use_state(LeaveType::default);
    let is_form_valid = use_state(|| true);

    let now = Utc::now().with_time(NaiveTime::MIN).unwrap();
    let date_from = use_state(|| now);
    let date_to = use_state(|| now);
    let half_day_opt = use_state(HalfDayOptions::default);
    let notes = use_state(|| None::<String>);

    let is_leave_request_loaded_clone = is_leave_request_loaded.clone();
    let is_employee_loaded_clone = is_employee_loaded.clone();
    let is_leave_type_loaded_clone = is_leave_type_loaded.clone();
    let leave_request_clone = leave_request.clone();
    let employee_clone = employee.clone();
    let leave_type_clone = leave_type.clone();
    let date_from_clone = date_from.clone();
    let date_to_clone = date_to.clone();
    let id = props.id;
    use_effect(move || {
        if !*is_leave_request_loaded_clone
            && !*is_employee_loaded_clone
            && !*is_leave_type_loaded_clone
        {
            wasm_bindgen_futures::spawn_local(async move {
                match get_leave_request(&id).await {
                    Ok(res) => {
                        log!(JsValue::from("Got leave request!"));
                        is_leave_request_loaded_clone.set(true);
                        let res_clone = res.clone();
                        leave_request_clone.set(res_clone.clone());
                        date_from_clone.set(res_clone.date_from);
                        date_to_clone.set(res_clone.date_to);

                        wasm_bindgen_futures::spawn_local(async move {
                            if !*is_leave_type_loaded_clone {
                                match get_leave_type(&res.leave_type_id).await {
                                    Ok(res) => {
                                        log!(JsValue::from("Got leave type!"));
                                        is_leave_type_loaded_clone.set(true);
                                        leave_type_clone.set(res);
                                    }
                                    Err(error) => {
                                        log!(JsValue::from(error.to_string()));
                                    }
                                }
                            }
                            if !*is_employee_loaded_clone {
                                match get_employee(&res.employee_id).await {
                                    Ok(res) => {
                                        log!(JsValue::from("Got employee!"));
                                        is_employee_loaded_clone.set(true);
                                        employee_clone.set(res)
                                    }
                                    Err(error) => {
                                        log!(JsValue::from(error.to_string()));
                                    }
                                }
                            }
                        })
                    }
                    Err(error) => {
                        log!(JsValue::from(error.to_string()));
                    }
                }
            })
        }
        || {}
    });

    let date_to_clone = date_to.clone();
    let date_from_clone = date_from.clone();
    let is_form_valid_clone = is_form_valid.clone();
    let date_from_onchage = {
        Callback::from(move |event: Event| {
            if let Some(input_element) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                let value = input_element.value();
                log!(&value);

                if let Ok(parsed_date) = NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
                    if let Some(naive_dt) = parsed_date.and_hms_opt(0, 0, 0) {
                        let datetime = DateTime::from_naive_utc_and_offset(naive_dt, Utc);
                        date_from_clone.set(datetime);

                        if *date_to_clone < datetime {
                            log!("Dates invalid!");
                            is_form_valid_clone.set(false);
                        } else {
                            is_form_valid_clone.set(true);
                        }
                    }
                }
            }
        })
    };

    let date_from_clone = date_from.clone();
    let date_to_clone = date_to.clone();
    let is_form_valid_clone = is_form_valid.clone();
    let date_to_onchage = {
        Callback::from(move |event: Event| {
            if let Some(input_element) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                let value = input_element.value();
                log!(&value);

                if let Ok(parsed_date) = NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
                    if let Some(naive_dt) = parsed_date.and_hms_opt(0, 0, 0) {
                        let datetime = DateTime::from_naive_utc_and_offset(naive_dt, Utc);
                        date_to_clone.set(datetime);

                        if *date_from_clone > datetime {
                            log!("Dates invalid!");
                            is_form_valid_clone.set(false);
                        } else {
                            is_form_valid_clone.set(true);
                        }
                    }
                }
            }
        })
    };

    let half_day_opt_onchange = {
        let half_day_opt = half_day_opt.clone();
        Callback::from(move |event: Event| {
            let target = event.target().unwrap().unchecked_into::<HtmlInputElement>();
            target.set_checked(true);

            let value = match event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .id()
                .as_str()
            {
                "firstDayHalf" => HalfDayOptions::FirstDayHalf,
                "lastDayHalf" => HalfDayOptions::LastDayHalf,
                "firstAndLastHalfs" => HalfDayOptions::BothHalfDays,
                _ => HalfDayOptions::FullDays,
            };

            half_day_opt.set(value);
        })
    };

    let notes_onchange = {
        let notes = notes.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            notes.set(Some(value));
        })
    };

    let date_from_clone = date_from.clone();
    let date_to_clone = date_to.clone();
    let leave_type_clone = leave_type.clone();
    let employee_clone = employee.clone();
    let leave_request_clone = leave_request.clone();
    let onsubmit = {
        let half_day_opt = half_day_opt.clone();
        let notes = notes;
        let history = use_navigator().unwrap();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let leave_request_clone = leave_request_clone.clone();
            let employee_id = employee_clone.deref().clone().id.unwrap();
            let leave_type_id = leave_type_clone.deref().clone().id.unwrap();
            let date_from_clone = *date_from_clone.deref();
            let date_to_clone = *date_to_clone.deref();
            let half_day_opt = half_day_opt.deref().clone();
            let total_duration =
                calculate_requested_days(&date_from_clone, &date_to_clone, &half_day_opt);
            let notes = notes.deref().clone();
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match update_leave_request(
                    &leave_request_clone.id.unwrap(),
                    &employee_id,
                    &leave_type_id,
                    &leave_request_clone.leave_status,
                    date_from_clone,
                    date_to_clone,
                    half_day_opt,
                    total_duration,
                    &leave_request_clone.date_requested,
                    &leave_request_clone.date_finalised,
                    notes,
                )
                .await
                {
                    Ok(_) => {
                        history.push(&Route::MyLeave);
                    }
                    Err(ApiError::NotAuthenticated) => {
                        history.push(&Route::Home);
                    }
                    Err(error) => {
                        gloo::console::error!(error.to_string());
                        panic!();
                    }
                }
            })
        })
    };

    let leave_type_name = leave_type.name.clone();

    let calculated_leave_duration =
        calculate_requested_days(date_from.deref(), date_to.deref(), &half_day_opt);
    let duration_gt_one = calculated_leave_duration > 1.0;

    let half_day_opt = half_day_opt.deref().clone();

    html! {
        <main class="container">
            <Navbar />
            <h1>{ "Leave Request Details" }</h1>
            if *is_leave_request_loaded.deref() && *is_employee_loaded.deref() && *is_leave_type_loaded.deref() {
                <section id="form">
                    <form {onsubmit}>
                        <label for="employeeName">{ "Employee" }</label>
                        <input type="text" id="employeeName" readonly=true value={format!("{} {}", employee.first_name, employee.last_name)} />

                        <label for="leaveTypeName">{ "Leave Type" }</label>
                        <input type="text" id="leaveTypeName" readonly=true value={leave_type_name} />

                        <p class="grid">
                            <div>
                                <label for="dateFrom">{ "Date From "}</label>
                                <input id="dateFrom" type="date" name="dateFrom" value={ utc_to_yyyy_mm_dd(&leave_request.date_from) } min={ utc_to_yyyy_mm_dd(&now) } max={ utc_to_yyyy_mm_dd_next_year_end(&now)} onchange={ date_from_onchage }/>
                            </div>
                            <div>
                                <label for="dateTo">{ "Date To "}</label>
                                <input id="dateTo" type="date" name="dateTo" value={ utc_to_yyyy_mm_dd(&leave_request.date_to) } min={ utc_to_yyyy_mm_dd(&now) } max={ utc_to_yyyy_mm_dd_next_year_end(&now)} onchange={ date_to_onchage }/>
                            </div>
                        </p>

                        <fieldset>
                            <label>
                                <input type="radio" name="halfDayOps" id="allDaysFull" checked={ half_day_opt == HalfDayOptions::FullDays } onchange={ half_day_opt_onchange.clone() }/>
                                { "All days are full" }
                            </label>
                            <label>
                                <input type="radio" name="halfDayOps" id="firstDayHalf" checked={ half_day_opt == HalfDayOptions::FirstDayHalf } onchange={ half_day_opt_onchange.clone() }/>
                                { "First day is a half day" }
                            </label>
                            <label>
                                <input type="radio" name="halfDayOps" id="lastDayHalf" checked={ half_day_opt == HalfDayOptions::LastDayHalf } disabled={ !duration_gt_one } onchange={ half_day_opt_onchange.clone() }/>
                                { "Last day is a half day" }
                            </label>
                            <label>
                                <input type="radio" name="halfDayOps" id="firstAndLastHalfs" checked={ half_day_opt == HalfDayOptions::BothHalfDays } disabled={ !duration_gt_one } onchange={ half_day_opt_onchange.clone() }/>
                                { "First and last days are half days" }
                            </label>
                        </fieldset>

                        <textarea
                                name="daysRequestNotes"
                                placeholder="Notes..."
                                onchange={ notes_onchange }
                            >
                            </textarea>
                            <small id="notesHelp">
                                <i>{ "Optional" }</i>
                            </small>

                    </form>
                </section>
            }
        </main>
    }
}

fn utc_to_yyyy_mm_dd(utc: &DateTime<Utc>) -> String {
    format!("{:04}-{:02}-{:02}", utc.year(), utc.month(), utc.day())
}

fn utc_to_yyyy_mm_dd_next_year_end(utc: &DateTime<Utc>) -> String {
    format!("{:04}-{:02}-{:02}", utc.year() + 1, 12, 31)
}

fn calculate_requested_days(
    date_from: &DateTime<Utc>,
    date_to: &DateTime<Utc>,
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
