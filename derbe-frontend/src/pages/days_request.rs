use std::ops::Deref;

use time::OffsetDateTime;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use crate::{api::{self, api_errors::ApiError}, components::organisms::navbar::Navbar, model::{leave_request::HalfDayOptions, leave_type}, router::Route};

#[function_component(DaysRequest)]
pub fn component() -> Html {
    let leave_type = use_state(|| 0);
    let date_from = use_state(|| OffsetDateTime::now_utc());
    let date_to = use_state(|| OffsetDateTime::now_utc());
    let half_day_opt = use_state(|| HalfDayOptions::FullDays);
    let notes = use_state(|| None::<String>);

    let leave_types_onchage = {
        let leave_type = leave_type.clone();
        Callback::from(move |new_leave_type| {
            leave_type.set(new_leave_type);
        })
    };

    let date_from_onchage = {
        let date_from = date_from.clone();
        Callback::from(move |new_date_from| {
            date_from.set(new_date_from);
        })
    };

    let date_to_onchage = {
        let date_to = date_to.clone();
        Callback::from(move |new_date_to| {
            date_to.set(new_date_to);
        })
    };

    let half_day_opt_onchage = {
        let half_day_opt = half_day_opt.clone();
        Callback::from(move |new_half_day_opt| {
            half_day_opt.set(new_half_day_opt);
        })
    };

    let notes_onchange = {
        let notes = notes.clone();
        Callback::from(move |new_notes| {
            notes.set(Some(new_notes));
        })
    };

    let onsubmit = {
        let leave_type = leave_type;
        let date_from = date_from;
        let date_to = date_to;
        let half_day_opt = half_day_opt;
        let notes = notes;
        let history = use_navigator().unwrap();
        
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let leave_type = leave_type.deref().clone();
            let date_from = date_from.deref().clone();
            let date_to = date_to.deref().clone();
            let half_day_opt = half_day_opt.deref().clone();
            let notes = notes.deref().clone();
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::create_leave_request(leave_type, date_from, date_to, half_day_opt, notes).await {
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


    html! {
        <main class="container">
            <Navbar />
            <section id="form">
                <form>
                    <h1>{ "Days Off Request" }</h1>
                    <label for="leaveTypeSelect">{ " Leave Type "}</label>
                    <select id="leaveTypeSelect" name="leaveTypeSelect" required=true >
                        <option value={ 1 } selected=true >{"Annual Leave"}</option>
                    </select>
                    
                    <label for="dateFrom">{ "Date From "}</label>
                    <input type="date" name="dateFrom" />

                    <label for="dateTo">{ "Date To "}</label>
                    <input type="date" name="dateTo" />

                    <fieldset>
                        <label>
                            <input type="radio" name="halfDayOps" id="allDaysFull" checked=true />
                            { "All days are full" }
                        </label>
                        <label>
                            <input type="radio" name="halfDayOps" id="firstDayHalf" />
                            { "First day is a half day" }
                        </label>
                        <label>
                            <input type="radio" name="halfDayOps" id="lastDayHalf" />
                            { "Last day is a half day" }
                        </label>
                        <label>
                            <input type="radio" name="halfDayOps" id="firstAndLastHalfs" />
                            { "First and last days are half days" }
                        </label>
                    </fieldset>

                    <textarea
                        name="daysRequestNotes"
                        placeholder="Notes..."
                    >
                    </textarea>
                </form>
            </section>
        </main>
    }
}