use std::cmp::Ordering;

use crate::api::leave_requests::create_leave_request;
use crate::components::atoms::dds_date_input::DateType;
use crate::{
    api::api_errors::ApiError,
    components::{
        atoms::{dds_date_input::DDSDateInput, dds_select::DDSSelect, dds_text_area::DDSTextArea},
        molecules::half_day_op_radio_group::HalfDayOpRadioGroup,
        organisms::navbar::Navbar,
    },
    router::Route,
};
use chrono::{NaiveDate, Utc};
use derbe_shared::dto::request::leave_request::LeaveRequestCreate;
use derbe_shared::dto::response::employee::EmployeeResponse;
use derbe_shared::dto::response::leave_type::LeaveTypeResponse;
use derbe_shared::enums::HalfDayOptions;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[function_component(LeaveRequestDetailsCreate)]
pub fn component() -> Html {
    let now = Utc::now().date_naive();
    let leave_request = use_state(LeaveRequestCreate::default);
    let is_form_valid = use_state(|| true);

    let employee_onchange = {
        let leave_request = leave_request.clone();
        Callback::from(move |employee_id: Option<i32>| {
            if let Some(id) = employee_id {
                let mut lr = (*leave_request).clone();
                lr.employee_id = id;
                leave_request.set(lr.clone());
            }
        })
    };

    let leave_types_onchange = {
        let leave_request = leave_request.clone();
        Callback::from(move |leave_type_id: Option<i32>| {
            if let Some(id) = leave_type_id {
                let mut lr = (*leave_request).clone();
                lr.leave_type_id = id;
                leave_request.set(lr.clone());
            }
        })
    };

    let date_from_onchange = {
        let is_form_valid_clone = is_form_valid.clone();
        let leave_request = leave_request.clone();
        Callback::from(move |(new_date, valid): (NaiveDate, bool)| {
            let mut lr = (*leave_request).clone();
            lr.values.set_date_from(new_date);
            leave_request.set(lr.clone());
            is_form_valid_clone.set(valid);
        })
    };

    let date_to_onchange = {
        let is_form_valid_clone = is_form_valid.clone();
        let leave_request = leave_request.clone();
        Callback::from(move |(new_date, valid): (NaiveDate, bool)| {
            let mut lr = (*leave_request).clone();
            lr.values.set_date_to(new_date);
            leave_request.set(lr.clone());
            is_form_valid_clone.set(valid);
        })
    };

    let half_day_opt_onchange = {
        let leave_request = leave_request.clone();
        Callback::from(move |half_day_op: HalfDayOptions| {
            let (first_day_half, last_day_half) = HalfDayOptions::to_bools(&half_day_op);
            let mut lr = (*leave_request).clone();
            lr.values
                .set_first_last_day_half((first_day_half, last_day_half));
            leave_request.set(lr.clone());
        })
    };

    let notes_onchange = {
        let leave_request = leave_request.clone();
        Callback::from(move |new_notes: Option<String>| {
            let mut lr = (*leave_request).clone();
            lr.notes = new_notes;
            leave_request.set(lr.clone());
        })
    };

    let onsubmit = {
        let leave_request = leave_request.clone();
        let history = use_navigator().unwrap();
        let mut lr = (*leave_request).clone();
        lr.date_requested = Utc::now();

        Callback::from(move |event: SubmitEvent| {
            let lr = lr.clone();
            event.prevent_default();
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match create_leave_request(&lr).await {
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

    html! {
        <main class="container">
            <Navbar />
                <section id="form">
                    <form {onsubmit}>
                        <h1>{ "Leave Request" }</h1>
                        <div class="grid">
                        <div>
                            <DDSSelect<EmployeeResponse>
                                    id="employeeSelect"
                                    name="employeeSelect"
                                    label="Employee"
                                    selected_id={leave_request.employee_id}
                                    excluded_id={None}
                                    is_optional_field=false
                                    onchange={employee_onchange}/>

                            <DDSSelect<LeaveTypeResponse>
                                    id="leaveTypeSelect"
                                    name="leaveTypeSelect"
                                    label="Leave Type"
                                    selected_id={leave_request.leave_type_id}
                                    excluded_id={None}
                                    is_optional_field=false
                                    onchange={leave_types_onchange}/>

                            <p class="grid">
                                <DDSDateInput id="dateFrom"
                                    name="dateFrom"
                                    label="Date From"
                                    value={leave_request.values.get_date_from()}
                                    date_type={DateType::DateFrom}
                                    other_date={leave_request.values.get_date_to()}
                                    min={now}
                                    max={now}
                                    on_change={date_from_onchange}/>
                                <DDSDateInput id="dateTo"
                                    name="dateTo"
                                    label="Date To"
                                    value={leave_request.values.get_date_to()}
                                    date_type={DateType::DateTo}
                                    other_date={leave_request.values.get_date_from()}
                                    min={now}
                                    max={now}
                                    on_change={date_to_onchange}/>
                            </p>

                            <HalfDayOpRadioGroup
                                duration_gt_one={
                                    leave_request.values.get_total_duration().partial_cmp(&1.0) != Some(Ordering::Greater)
                                }
                                selected={HalfDayOptions::from_bools(leave_request.values.get_first_last_day_half())}
                                on_change={half_day_opt_onchange}
                            />

                            <DDSTextArea
                                name="daysRequestNotes"
                                placeholder="Notes..."
                                label="Notes"
                                value={leave_request.notes.clone().unwrap_or_default()}
                                small={Some("Optional".to_owned())}
                                on_change={notes_onchange}
                            />
                        </div>

                        // <div>
                        // if *is_form_valid {
                        //     <section id="output">
                        //         <p>{ "You are requesting " }<kbd>{ leave_request.get_total_duration() }</kbd>{ format!(" days of {}.", leave_type_name) }</p>
                        //         if let Some(quota) = leave_type_quota {
                        //             <p>{ "You have a total quota of " }<kbd>{quota}</kbd>{ format!(" days of {}.", leave_type_name) }</p>
                        //             <p>{ "After this request has been Accepted, you will have " }<kbd>{"???"}</kbd>{ " days remaining."}</p>
                        //         }
                        //     </section>
                        // }
                        // </div>
                        </div>

                        <p class="grid">
                            <button onclick={ cancel_onclick } class="secondary">{ "Cancel"}</button>
                            <button disabled={ !*is_form_valid }>{ "Submit"}</button>
                        </p>
                    </form>
                </section>

        </main>
    }
}
