use std::ops::Deref;

use chrono::{DateTime, NaiveDate, Utc};
use gloo::console::log;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use crate::{api::{self, api_errors::ApiError}, components::organisms::navbar::Navbar, model::{leave_request::HalfDayOptions, leave_type}, router::Route};

#[function_component(DaysRequest)]
pub fn component() -> Html {
    let now = Utc::now();
    let leave_type = use_state(|| 1);
    let date_from = use_state(|| now);
    let date_to = use_state(|| now);
    let half_day_opt = use_state(|| HalfDayOptions::FullDays);
    let notes = use_state(|| None::<String>);

    let leave_types_onchage = {
        let leave_type = leave_type.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();
            log!(&value);

            let value = value.parse::<i32>().unwrap();

            leave_type.set(value);
        })
    };

    let date_from_onchage = {
        let date_from = date_from.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            log!(&value);

            let parsed_date = NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap();
            let datetime = DateTime::<Utc>::from_utc(parsed_date.and_hms(0, 0, 0), Utc);

            date_from.set(datetime);
        })
    };

    let date_to_onchage = {
        let date_to = date_to.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            log!(&value);
            let parsed_date = NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap();
            let datetime = DateTime::<Utc>::from_utc(parsed_date.and_hms(0, 0, 0), Utc);

            date_to.set(datetime);
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

    let onsubmit = {
        let leave_type = leave_type;
        let date_from = date_from;
        let date_to = date_to;
        let half_day_opt = half_day_opt.clone();
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

    let cancel_onclick = {
        let history = use_navigator().unwrap();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            history.push(&Route::MyLeave);
        })
    };

    let half_day_opt = half_day_opt.deref().clone();

    html! {
        <main class="container">
            <Navbar />
            <section id="form">
                <form {onsubmit}>
                    <h1>{ "Days Off Request" }</h1>
                    <label for="leaveTypeSelect">{ " Leave Type "}</label>
                    <select id="leaveTypeSelect" name="leaveTypeSelect" required=true onchange={leave_types_onchage}>
                        <option value={ 1 } selected=true >{"Annual Leave"}</option>
                        <option value={ 2 } selected=false >{"Maternity Leave"}</option>
                    </select>
                    
                    <label for="dateFrom">{ "Date From "}</label>
                    <input type="date" name="dateFrom" onchange={ date_from_onchage }/>

                    <label for="dateTo">{ "Date To "}</label>
                    <input type="date" name="dateTo" onchange={ date_to_onchage }/>

                    

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
                    <button onclick={ cancel_onclick }>{ "Cancel"}</button>
                    <button>{ "Submit"}</button>
                </form>
            </section>
        </main>
    }
}