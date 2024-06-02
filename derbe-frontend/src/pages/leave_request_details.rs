use std::ops::Deref;

use crate::{
    api::{
        api_errors::ApiError,
        employees::get_employee,
        leave_requests::{get_leave_request, update_leave_request, update_leave_request_status},
        leave_types::get_leave_type,
    },
    components::{
        atoms::dds_date_input::{DDSDateInput, DateType},
        molecules::half_day_op_radio_group::HalfDayOpRadioGroup,
        organisms::navbar::Navbar,
    },
    dto::leave_request::LeaveRequestStatusUpdate,
    model::{
        employee::Employee,
        leave_request::{HalfDayOptions, LeaveRequest, LeaveStatus},
        leave_type::LeaveType,
    },
    router::Route,
    utils::date_utils::calculate_requested_days,
};
use chrono::{NaiveDate, Utc};
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

    let now = Utc::now().date_naive();
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
    let half_day_opt_clone = half_day_opt.clone();
    let notes_clone = notes.clone();
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
                        notes_clone.set(res_clone.notes);
                        half_day_opt_clone.set(HalfDayOptions::from_bools(
                            res.first_day_half,
                            res.last_day_half,
                        ));

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
        Callback::from(move |half_day_op: HalfDayOptions| {
            half_day_opt.set(half_day_op);
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

    let leave_request_id = leave_request.id;
    let employee_id = employee.id;
    let leave_type_id = leave_type.id;
    let leave_status = leave_request.leave_status;
    let date_from_clone = date_from.clone();
    let date_to_clone = date_to.clone();
    let date_requested = leave_request.date_requested;
    let date_finalised = leave_request.date_finalised;
    let notes_clone = notes.clone();

    let onsubmit = {
        let half_day_opt = half_day_opt.clone();
        let total_duration =
            calculate_requested_days(&date_from_clone, &date_to_clone, &half_day_opt);
        let history = use_navigator().unwrap();

        let (first_day_half, last_day_half) = half_day_opt.deref().to_bools();

        Callback::from(move |event: SubmitEvent| {
            let note_clone = notes_clone.deref().clone();
            let employee_id = employee_id.unwrap();
            let leave_type_id = leave_type_id.unwrap();

            let updated_leave_request = LeaveRequest {
                id: leave_request_id,
                employee_id,
                leave_type_id,
                leave_status,
                date_from: *date_from_clone,
                date_to: *date_to_clone,
                first_day_half,
                last_day_half,
                total_duration,
                date_requested,
                date_finalised,
                notes: note_clone,
            };

            event.prevent_default();
            let updated_leave_request = updated_leave_request.clone();
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match update_leave_request(&updated_leave_request).await {
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

    let more_info_req_onclick = {
        let history = use_navigator().unwrap();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            history.push(&Route::MyLeave);
        })
    };

    let rejected_onclick = {
        let history = use_navigator().unwrap();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            history.push(&Route::MyLeave);
        })
    };

    let notes_clone = notes.clone();
    let id = leave_request.id;
    let accepted_onclick = {
        let history = use_navigator().unwrap();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let note_clone_str = notes_clone.deref().clone();
            let req = LeaveRequestStatusUpdate {
                leave_status: LeaveStatus::Accepted,
                notes: note_clone_str,
            };
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match update_leave_request_status(&id.unwrap(), req).await {
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

    let half_day_opt_clone = half_day_opt.deref().clone();

    let notes_clone = notes.clone();
    let notes_string = notes_clone.deref().clone().unwrap_or(String::new());

    let date_from_clone = *date_from.deref();
    let date_to_clone = *date_to.deref();

    html! {
        <main class="container">
            <Navbar />
            <h1>{ "Leave Request Details" }</h1>
            if *is_leave_request_loaded.deref() && *is_employee_loaded.deref() && *is_leave_type_loaded.deref() {
                <section id="form">
                    <form onsubmit = {onsubmit}>
                        <label for="employeeName">{ "Employee" }</label>
                        <input type="text" id="employeeName" readonly=true value={format!("{} {}", employee.first_name, employee.last_name)} />

                        <label for="leaveTypeName">{ "Leave Type" }</label>
                        <input type="text" id="leaveTypeName" readonly=true value={leave_type_name} />

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

                        <HalfDayOpRadioGroup
                            duration_gt_one={!duration_gt_one}
                            selected={half_day_opt_clone}
                            on_change={half_day_opt_onchange}
                        />

                        <label for="daysRequestNotes">{ "Notes" }</label>
                        <textarea
                                name="daysRequestNotes"
                                placeholder="Notes..."
                                onchange={ notes_onchange }
                                value={notes_string}
                        />
                        <small id="notesHelp">
                            <i>{ "Optional" }</i>
                        </small>

                        <div role="group">
                            <button onclick={ cancel_onclick } class="secondary">{ "Cancel"}</button>
                            <button disabled={ !*is_form_valid }>{ "Submit"}</button>
                        </div>

                        <div role="group">
                            <button onclick={ more_info_req_onclick } disabled={ !*is_form_valid }>{ "More Info Required" }</button>
                            <button onclick={ rejected_onclick } disabled={ !*is_form_valid }>{ "Reject" }</button>
                            <button onclick={ accepted_onclick } disabled={ !*is_form_valid } aria-current="true">{ "Accept" }</button>
                        </div>

                    </form>
                </section>
            }
        </main>
    }
}
