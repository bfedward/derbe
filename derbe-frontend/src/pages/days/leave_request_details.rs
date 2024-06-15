use std::cmp::Ordering;

use crate::{
    api::{
        api_errors::ApiError,
        leave_requests::{get_leave_request, update_leave_request, update_leave_request_status},
    },
    components::{
        atoms::{
            dds_date_input::{DDSDateInput, DateType},
            dds_text_area::DDSTextArea,
            dds_text_input::DDSTextInputForSelectable,
        },
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
};
use chrono::{NaiveDate, Utc};
use gloo::console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component(LeaveRequestDetails)]
pub fn component(props: &Props) -> Html {
    let leave_request = use_state(|| None::<LeaveRequest>);
    let is_form_valid = use_state(|| true);
    let now = Utc::now().date_naive();

    let id = props.id;
    let leave_request_clone = leave_request.clone();
    use_effect(move || {
        if leave_request_clone.is_none() {
            wasm_bindgen_futures::spawn_local(async move {
                match get_leave_request(&id).await {
                    Ok(res) => {
                        log!(JsValue::from("Got leave request!"));
                        leave_request_clone.set(Some(res));
                    }
                    Err(error) => {
                        log!(JsValue::from(error.to_string()));
                    }
                }
            })
        }
        || {}
    });

    let date_from_onchange = {
        let is_form_valid_clone = is_form_valid.clone();
        let leave_request = leave_request.clone();
        Callback::from(move |(new_date, valid): (NaiveDate, bool)| {
            if valid {
                if let Some(mut lr) = (*leave_request).as_ref().cloned() {
                    lr.set_date_from(new_date);
                    leave_request.set(Some(lr));
                }
            } else {
                is_form_valid_clone.set(valid);
            }
        })
    };

    let date_to_onchange = {
        let is_form_valid_clone = is_form_valid.clone();
        let leave_request = leave_request.clone();
        Callback::from(move |(new_date, valid): (NaiveDate, bool)| {
            if valid {
                if let Some(mut lr) = (*leave_request).as_ref().cloned() {
                    lr.set_date_to(new_date);
                    leave_request.set(Some(lr.clone()));
                }
            } else {
                is_form_valid_clone.set(valid);
            }
        })
    };

    let half_day_opt_onchange = {
        let leave_request = leave_request.clone();
        Callback::from(move |half_day_op: HalfDayOptions| {
            if let Some(mut lr) = (*leave_request).as_ref().cloned() {
                let (first_day_half, last_day_half) = HalfDayOptions::to_bools(&half_day_op);
                lr.set_first_last_day_half((first_day_half, last_day_half));
                leave_request.set(Some(lr.clone()));
            }
        })
    };

    let notes_onchange = {
        let leave_request = leave_request.clone();
        Callback::from(move |new_notes: Option<String>| {
            if let Some(mut lr) = (*leave_request).as_ref().cloned() {
                lr.notes = new_notes;
                leave_request.set(Some(lr.clone()));
            }
        })
    };

    let onsubmit = {
        let history = use_navigator().unwrap();
        let leave_request = leave_request.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            if let Some(updated_leave_request) = (*leave_request).as_ref().cloned() {
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
            }
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

    let accepted_onclick = {
        let history = use_navigator().unwrap();
        let leave_request = leave_request.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            if let Some(lr) = (*leave_request).as_ref().cloned() {
                let req = LeaveRequestStatusUpdate {
                    leave_status: LeaveStatus::Accepted,
                    notes: lr.notes,
                };
                let history = history.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match update_leave_request_status(&lr.id.unwrap(), req).await {
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
            }
        })
    };

    html! {
        <main class="container">
            <Navbar />
            <h1>{ "Leave Request Details" }</h1>
            if let Some(leave_request) = (*leave_request).as_ref().cloned() {
                <section id="form">
                    <form onsubmit = {onsubmit}>
                        <DDSTextInputForSelectable<Employee>
                            id="employeeName"
                            label="Employee"
                            selected_id={leave_request.employee_id}
                        />
                        <DDSTextInputForSelectable<LeaveType>
                            id="leaveTypeName"
                            label="Leave Type"
                            selected_id={leave_request.leave_type_id}
                        />

                        <p class="grid">
                            <DDSDateInput id="dateFrom"
                                name="dateFrom"
                                label="Date From"
                                value={leave_request.get_date_from()}
                                date_type={DateType::DateFrom}
                                other_date={leave_request.get_date_to()}
                                min={now}
                                max={now}
                                on_change={date_from_onchange}/>
                            <DDSDateInput id="dateTo"
                                name="dateTo"
                                label="Date To"
                                value={leave_request.get_date_to()}
                                date_type={DateType::DateTo}
                                other_date={leave_request.get_date_from()}
                                min={now}
                                max={now}
                                on_change={date_to_onchange}/>
                        </p>

                        <HalfDayOpRadioGroup
                                duration_gt_one={
                                    leave_request.get_total_duration().partial_cmp(&1.0) != Some(Ordering::Greater)
                                }
                                selected={HalfDayOptions::from_bools(leave_request.get_first_last_day_half())}
                                on_change={half_day_opt_onchange}
                        />

                        <DDSTextArea
                            name="daysRequestNotes"
                            placeholder="Notes..."
                            label="Notes"
                            value={leave_request.notes.unwrap_or_default()}
                            small={Some("Optional".to_owned())}
                            on_change={notes_onchange}
                        />

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
