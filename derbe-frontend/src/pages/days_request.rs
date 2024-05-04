use std::ops::Deref;

use chrono::{DateTime, Datelike, NaiveDate, Utc};
use gloo::console::log;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use wasm_bindgen::{JsCast, JsValue};
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use crate::{api::{self, api_errors::ApiError, get_employees, get_leave_types}, components::organisms::navbar::Navbar, model::{employee::Employee, leave_request::HalfDayOptions, leave_type::LeaveType}, router::Route};

#[function_component(DaysRequest)]
pub fn component() -> Html {
    let is_leave_types_loaded = use_state(|| false);
    let is_employees_loaded = use_state(|| false);
    let leave_types = use_state(Vec::new);
    let selected_leave_type = use_state(|| None);
    let employees = use_state(Vec::new);

    let is_leave_types_loaded_clone = is_leave_types_loaded.clone();
    let is_employees_loaded_clone = is_employees_loaded.clone();
    let leave_types_clone = leave_types.clone();
    let selected_leave_type_clone = selected_leave_type.clone();
    let employees_clone = employees.clone();

    use_effect(move || {
        let leave_types_clone = leave_types_clone.clone();
        let employees_clone = employees_clone.clone();
        if !*is_leave_types_loaded_clone && !*is_employees_loaded_clone {
            let leave_types_clone = leave_types_clone.clone();
            let employees_clone = employees_clone.clone();
            let is_leave_types_loaded_clone = is_leave_types_loaded_clone.clone();
            let is_employees_loaded_clone = is_employees_loaded_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let leave_types_clone = leave_types_clone.clone();
                let employees_clone = employees_clone.clone();
                match get_leave_types().await {
                    Ok(res) => {
                        let leave_types_clone = leave_types_clone.clone();
                        log!(JsValue::from("Got leave types!"));
            
                        if let Some(selected_leave_type) = res.iter()
                                    .find(|&leave_type| leave_type.default_in_others_days == true) {
                            selected_leave_type_clone.set(Some(selected_leave_type.clone()));
                        } else {
                            log!("No default leave!!");
                            selected_leave_type_clone.set(None);
                        }

                        leave_types_clone.set(res);
                        is_leave_types_loaded_clone.set(true);
                    }
                    Err(error) => {
                        log!(JsValue::from(error.to_string()));
                    }
                }
                match get_employees().await {
                    Ok(res) => {
                        let employees_clone = employees_clone.clone();
                        log!(JsValue::from("Got employees!"));
                        let mut res = res;
                        res.sort_by_key(|employee| employee.last_name.clone());
                        employees_clone.set(res);
                        is_employees_loaded_clone.set(true);
                    }
                    Err(error) => {
                        log!(JsValue::from(error.to_string()));
                    }
                }
            });
        }
        || {}
    });
    

    let now = Utc::now();

    let employee = use_state(|| 0);
    let date_from = use_state(|| now);
    let date_to = use_state(|| now);
    let half_day_opt = use_state(|| HalfDayOptions::FullDays);
    let notes = use_state(|| None::<String>);

    let employee_onchange = {
        let employee = employee.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();
            log!(&value);

            let value = value.parse::<i32>().unwrap();

            employee.set(value);
        })
    };
    
    let selected_leave_type_clone = selected_leave_type.clone();
    let leave_types_clone = leave_types.clone();
    let leave_types_onchange = {
        Callback::from(move |event: Event| {
            if let Some(select_element) = event.target().and_then(|t| t.dyn_into::<HtmlSelectElement>().ok()) {
                let value = select_element.value();
                log!(&value);

                if let Ok(value) = value.parse::<i32>() {
                    if let Some(selected_leave_type) = leave_types_clone.iter()
                                    .find(|&leave_type| leave_type.id == Some(value)) {
                        selected_leave_type_clone.set(Some(selected_leave_type.clone()));
                    } else {
                        selected_leave_type_clone.set(None);
                    }
                } else {
                    log!("Error parsing value as i32");
                }
            } else {
                log!("Error getting select element");
            }
        })
    };


    let date_from_clone = date_from.clone();
    let date_from_onchage = {
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            log!(&value);

            let parsed_date = NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap();
            let datetime = DateTime::from_naive_utc_and_offset(parsed_date.and_hms_opt(0, 0, 0).unwrap(), Utc);

            date_from_clone.set(datetime);
        })
    };

    let date_to_clone = date_to.clone();
    let date_to_onchage = {
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            log!(&value);
            let parsed_date = NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap();
            let datetime = DateTime::from_naive_utc_and_offset(parsed_date.and_hms_opt(0, 0, 0).unwrap(), Utc);

            date_to_clone.set(datetime);
        })
    };

    let half_day_opt_onchage = {
        let half_day_opt = half_day_opt.clone();
        Callback::from(move |event: Event| {
            let target = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>();
            target.set_checked(true);

            let value = match event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .id()
                .as_str() {
                    "firstDayHalf" => HalfDayOptions::FirstDayHalf,
                    "lastDayHalf" => HalfDayOptions::LastDayHalf,
                    "firstAndLastHalfs" => HalfDayOptions::BothHalfDays,
                    _ => HalfDayOptions::FullDays
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
    let selected_leave_type_clone = selected_leave_type.clone();
    let onsubmit = {
        let employee = employee;
        let half_day_opt = half_day_opt.clone();
        let notes = notes;
        let history = use_navigator().unwrap();
        
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let employee = employee.deref().clone();
            let leave_type = selected_leave_type_clone.deref().clone();
            let date_from_clone = *date_from_clone.deref();
            let date_to = *date_to_clone.deref();
            let half_day_opt = half_day_opt.deref().clone();
            let notes = notes.deref().clone();
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::create_leave_request(employee, 
                                                leave_type.unwrap().id.unwrap(), 
                                                date_from_clone, 
                                                date_to, 
                                                half_day_opt, 
                                                notes).await {
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

    let cancel_onclick = {
        let history = use_navigator().unwrap();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            history.push(&Route::MyLeave);
        })
    };

    let half_day_opt = half_day_opt.deref().clone();
    let date_from_clone = date_from.deref().clone();
    let date_to_clone = date_to.deref().clone();

    let leave_type_name = if let Some(leave_type) = selected_leave_type.deref() {
        leave_type.name.clone()
    } else {
        String::new()
    };

    let leave_type_quota = if let Some(leave_type) = selected_leave_type.deref() {
        leave_type.default_quota
    } else {
        None
    };

    html! {
        <main class="container">
            <Navbar />
            if *is_leave_types_loaded.deref() && *is_employees_loaded.deref() {
                <section id="form">
                    <form {onsubmit}>
                        <h1>{ "Days Off Request" }</h1>

                        <label for="employeeSelect">{ "Employee "}</label>
                        <select id="employeeSelect" name="employeeSelect" required=true onchange={employee_onchange}>
                            { for employee_select_options_html(&employees.deref()) }
                        </select>

                        <label for="leaveTypeSelect">{ " Leave Type "}</label>
                        <select id="leaveTypeSelect" name="leaveTypeSelect" required=true onchange={leave_types_onchange}>
                            { for leave_type_options_html(&leave_types.deref())}
                        </select>
                    
                        <p class="grid">
                            <div>
                                <label for="dateFrom">{ "Date From "}</label>
                                <input id="dateFrom" type="date" name="dateFrom" value={ utc_to_yyyy_mm_dd(&date_from_clone) } min={ utc_to_yyyy_mm_dd(&now) } onchange={ date_from_onchage }/>
                            </div>
                            <div>
                                <label for="dateTo">{ "Date To "}</label>
                                <input id="dateTo" type="date" name="dateTo" value={ utc_to_yyyy_mm_dd(&date_to_clone) } min={ utc_to_yyyy_mm_dd(&now) } onchange={ date_to_onchage }/>
                            </div>
                        </p>
                        

                        <fieldset>
                            <label>
                                <input type="radio" name="halfDayOps" id="allDaysFull" checked={ half_day_opt == HalfDayOptions::FullDays } onchange={ half_day_opt_onchage.clone() }/>
                                { "All days are full" }
                            </label>
                            <label>
                                <input type="radio" name="halfDayOps" id="firstDayHalf" checked={ half_day_opt == HalfDayOptions::FirstDayHalf } onchange={ half_day_opt_onchage.clone() }/>
                                { "First day is a half day" }
                            </label>
                            <label>
                                <input type="radio" name="halfDayOps" id="lastDayHalf" checked={ half_day_opt == HalfDayOptions::LastDayHalf } onchange={ half_day_opt_onchage.clone() }/>
                                { "Last day is a half day" }
                            </label>
                            <label>
                                <input type="radio" name="halfDayOps" id="firstAndLastHalfs" checked={ half_day_opt == HalfDayOptions::BothHalfDays } onchange={ half_day_opt_onchage.clone() }/>
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

                        <section id="output">
                            <p>{ "You are requesting " }<kbd>{"7"}</kbd>{ format!(" days of {}.", leave_type_name) }</p>
                            if let Some(quota) = leave_type_quota {
                                <p>{ "You have a total quota of " }<kbd>{quota}</kbd>{ format!(" days of {}.", leave_type_name) }</p>
                                <p>{ "After this request has been Accepted, you will have " }<kbd>{"4"}</kbd>{ " days remaining."}</p>
                            }
                        </section>

                        <p class="grid">
                            <button onclick={ cancel_onclick } class="secondary">{ "Cancel"}</button>
                            <button>{ "Submit"}</button>
                        </p>
                    </form>
                </section>
            }
        </main>
    }
}

fn employee_select_options_html(employees: &Vec<Employee>) -> Vec<Html> {
    employees.iter().enumerate().map(|(index, employee)| {
        let selected = if index == 0 { true } else { false };
        html! {
            <option value={ employee.id.unwrap().to_string() } selected={ selected }>
                { format!("{} {}", &employee.first_name, &employee.last_name) }
            </option>
        }
    }).collect()
}

fn leave_type_options_html(leave_types: &Vec<LeaveType>) -> Vec<Html> {
    leave_types.iter().map(|leave_type| {
        html! {
            <option value={ leave_type.id.unwrap().to_string() } selected={ leave_type.default_in_others_days }>{ &leave_type.name }</option>
        }
    }).collect()
}

fn utc_to_yyyy_mm_dd(utc: &DateTime<Utc>) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        utc.year(),
        utc.month(),
        utc.day()
    )
}