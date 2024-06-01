use std::ops::Deref;

use chrono::{NaiveDate, Utc};
use gloo::console::log;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::api::employees::get_employees;
use crate::api::leave_requests::create_leave_request;
use crate::api::leave_types::get_leave_types;
use crate::components::atoms::dds_date_input::DateType;
use crate::utils::date_utils::calculate_requested_days;
use crate::{
    api::api_errors::ApiError,
    components::{
        atoms::{dds_date_input::DDSDateInput, dds_select::DDSSelect},
        organisms::navbar::Navbar,
    },
    model::{employee::Employee, leave_request::HalfDayOptions, leave_type::LeaveType},
    router::Route,
};

#[function_component(LeaveRequestCreate)]
pub fn component() -> Html {
    let is_leave_types_loaded = use_state(|| false);
    let is_employees_loaded = use_state(|| false);

    let now = Utc::now().date_naive();

    let date_from = use_state(|| now);
    let date_to = use_state(|| now);
    let half_day_opt = use_state(|| HalfDayOptions::FullDays);
    let notes = use_state(|| None::<String>);

    let leave_types = use_state(Vec::new);
    let selected_leave_type = use_state(|| None);
    let employees = use_state(Vec::new);
    let selected_employee = use_state(|| None);
    let is_form_valid = use_state(|| true);

    let is_leave_types_loaded_clone = is_leave_types_loaded.clone();
    let is_employees_loaded_clone = is_employees_loaded.clone();
    let leave_types_clone = leave_types.clone();
    let selected_leave_type_clone = selected_leave_type.clone();
    let employees_clone = employees.clone();
    let selected_employee_clone = selected_employee.clone();

    use_effect(move || {
        if !*is_leave_types_loaded_clone && !*is_employees_loaded_clone {
            wasm_bindgen_futures::spawn_local(async move {
                match get_leave_types().await {
                    Ok(res) => {
                        log!(JsValue::from("Got leave types!"));
                        if let Some(selected_leave_type) = res
                            .iter()
                            .find(|&leave_type| leave_type.default_in_others_days)
                        {
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
                    Ok(mut res) => {
                        log!(JsValue::from("Got employees!"));
                        res.sort_by_key(|employee| employee.last_name.clone());
                        if let Some(first) = res.first().cloned() {
                            selected_employee_clone.set(Some(first));
                        } else {
                            selected_employee_clone.set(None);
                        }
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

    let selected_employee_clone = selected_employee.clone();
    let employees_clone = employees.clone();
    let employee_onchange = {
        Callback::from(move |employee_id: i32| {
            if let Some(selected_employee) = employees_clone
                .iter()
                .find(|&employee| employee.id == Some(employee_id))
            {
                selected_employee_clone.set(Some(selected_employee.clone()));
            } else {
                selected_employee_clone.set(None);
            }
        })
    };

    let selected_leave_type_clone = selected_leave_type.clone();
    let leave_types_clone = leave_types.clone();
    let leave_types_onchange = {
        Callback::from(move |leave_type_id: i32| {
            if let Some(selected_leave_type) = leave_types_clone
                .iter()
                .find(|&leave_type| leave_type.id == Some(leave_type_id))
            {
                selected_leave_type_clone.set(Some(selected_leave_type.clone()));
            } else {
                selected_leave_type_clone.set(None);
            }
        })
    };

    let date_from_clone = date_from.clone();
    let is_form_valid_clone = is_form_valid.clone();
    let date_from_onchange = {
        Callback::from(move |(new_date, valid): (NaiveDate, bool)| {
            date_from_clone.set(new_date);
            is_form_valid_clone.set(valid);
        })
    };

    let date_to_clone = date_to.clone();
    let is_form_valid_clone = is_form_valid.clone();
    let date_to_onchange = {
        Callback::from(move |(new_date, valid): (NaiveDate, bool)| {
            date_to_clone.set(new_date);
            is_form_valid_clone.set(valid);
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
    let selected_leave_type_clone = selected_leave_type.clone();
    let selected_employee_clone = selected_employee.clone();
    let onsubmit = {
        let half_day_opt = half_day_opt.clone();
        let notes = notes;
        let history = use_navigator().unwrap();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let employee = selected_employee_clone.deref().clone();
            let leave_type = selected_leave_type_clone.deref().clone();
            let date_from_clone = *date_from_clone.deref();
            let date_to_clone = *date_to_clone.deref();
            let half_day_opt = half_day_opt.deref().clone();
            let total_duration =
                calculate_requested_days(&date_from_clone, &date_to_clone, &half_day_opt);
            let notes = notes.deref().clone();
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match create_leave_request(
                    employee.unwrap().id.unwrap(),
                    leave_type.unwrap().id.unwrap(),
                    date_from_clone,
                    date_to_clone,
                    half_day_opt,
                    total_duration,
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

    let cancel_onclick = {
        let history = use_navigator().unwrap();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            history.push(&Route::MyLeave);
        })
    };

    let half_day_opt = half_day_opt.deref().clone();
    let date_from_clone = *date_from.deref();
    let date_to_clone = *date_to.deref();

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

    let calculated_leave_duration =
        calculate_requested_days(date_from.deref(), date_to.deref(), &half_day_opt);
    let duration_gt_one = calculated_leave_duration > 1.0;

    html! {
        <main class="container">
            <Navbar />
            if *is_leave_types_loaded.deref() && *is_employees_loaded.deref() {
                <section id="form">
                    <form {onsubmit}>
                        <h1>{ "Leave Request" }</h1>

                        <div class="grid">
                        <div>
                            <DDSSelect id="employeeSelect"
                                    name="employeeSelect"
                                    label="Employee"
                                    options={Employee::to_select_options_html(employees.deref())}
                                    onchange={employee_onchange}/>

                            <DDSSelect id="leaveTypeSelect"
                                    name="leaveTypeSelect"
                                    label="Leave Type"
                                    options={ LeaveType::to_select_options_html(leave_types.deref()) }
                                    onchange={leave_types_onchange}/>

                            <p class="grid">
                                <DDSDateInput id="dateFrom"
                                    name="dateFrom"
                                    label="Date From"
                                    value={date_from_clone}
                                    date_type={DateType::DateFrom}
                                    other_date={date_to_clone}
                                    min={now}
                                    max={now}
                                    on_change={date_from_onchange}/>
                                <DDSDateInput id="dateTo"
                                    name="dateTo"
                                    label="Date To"
                                    value={date_to_clone}
                                    date_type={DateType::DateTo}
                                    other_date={date_from_clone}
                                    min={now}
                                    max={now}
                                    on_change={date_to_onchange}/>
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

                            <label for="daysRequestNotes">{ "Notes"}</label>
                            <textarea
                                name="daysRequestNotes"
                                placeholder="Notes..."
                                onchange={ notes_onchange }
                            >
                            </textarea>
                            <small id="notesHelp">
                                <i>{ "Optional" }</i>
                            </small>
                        </div>

                        <div>
                        if *is_form_valid {
                            <section id="output">
                                <p>{ "You are requesting " }<kbd>{ calculated_leave_duration }</kbd>{ format!(" days of {}.", leave_type_name) }</p>
                                if let Some(quota) = leave_type_quota {
                                    <p>{ "You have a total quota of " }<kbd>{quota}</kbd>{ format!(" days of {}.", leave_type_name) }</p>
                                    <p>{ "After this request has been Accepted, you will have " }<kbd>{"???"}</kbd>{ " days remaining."}</p>
                                }
                            </section>
                        }
                        </div>
                        </div>

                        <p class="grid">
                            <button onclick={ cancel_onclick } class="secondary">{ "Cancel"}</button>
                            <button disabled={ !*is_form_valid }>{ "Submit"}</button>
                        </p>
                    </form>
                </section>
            }
        </main>
    }
}
