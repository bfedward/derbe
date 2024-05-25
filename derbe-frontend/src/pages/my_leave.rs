use chrono::{DateTime, Datelike, Utc};
use std::ops::Deref;

use gloo::console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::components::Link;
use yew_router::hooks::use_navigator;

use crate::api::leave_requests::get_my_leave;
use crate::dto::leave_request::MyLeaveResponse;
use crate::{components::organisms::navbar::Navbar, router::Route};

#[function_component(MyLeave)]
pub fn component() -> Html {
    let is_loaded = use_state(|| false);
    let my_leaves = use_state(Vec::new);

    let is_loaded_clone = is_loaded.clone();
    let my_leaves_clone = my_leaves.clone();

    use_effect(move || {
        if !*is_loaded_clone {
            wasm_bindgen_futures::spawn_local(async move {
                match get_my_leave().await {
                    Ok(res) => {
                        let my_leaves_clone = my_leaves_clone.clone();
                        log!(JsValue::from("Got my leave!"));
                        my_leaves_clone.set(res);
                    }
                    Err(error) => {
                        log!(JsValue::from(error.to_string()));
                    }
                }
                is_loaded_clone.set(true);
            });
        }
        || {}
    });

    let request_onclick = {
        let navigator = use_navigator().unwrap();
        Callback::from(move |_: MouseEvent| {
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                navigator.push(&Route::DaysRequest);
            })
        })
    };

    // let leave_onclick = {
    //     let navigator = use_navigator().unwrap();
    //     Callback::from(move |_: MouseEvent| {
    //         let navigator = navigator.clone();
    //         wasm_bindgen_futures::spawn_local(async move {
    //
    //         })
    //     })
    // };

    html! {
        <main class="container">
            <Navbar />
            <h1>{ "My Leave" }</h1>
            <button onclick={ request_onclick }>{ "Request Days Off "}</button>
            if *is_loaded.deref() {
                <table>
                    <thead>
                        <tr>
                            <th scope="col">{ "Leave Type" }</th>
                            <th scope="col">{ "Status" }</th>
                            <th scope="col">{ "Date From" }</th>
                            <th scope="col">{ "Date To" }</th>
                            <th scope="col">{ "Total Duration" }</th>
                            <th scope="col">{ "" }</th>
                        </tr>
                    </thead>
                    <tbody>
                    { for my_leave_html(&my_leaves.deref().clone()) }
                    </tbody>
                </table>
            }
        </main>
    }
}

fn my_leave_html(my_leaves: &[MyLeaveResponse]) -> Vec<Html> {
    my_leaves
        .iter()
        .map(|my_leave| {
            html! {
                <tr>
                    <th scope="col">{ &my_leave.leave_type }</th>
                    <td>
                        { &my_leave.leave_status.to_string() }
                    </td>
                    <td>
                        { format_my_leave_date(my_leave.date_from) }
                    </td>
                    <td>
                        { format_my_leave_date(my_leave.date_to) }
                    </td>
                    <td>
                        { &my_leave.total_duration }
                    </td>
                    <td>
                        <Link<Route> to={Route::LeaveRequestDetails { id: my_leave.leave_request_id }}>
                            { "✎" }
                        </Link<Route>>
                    </td>
                </tr>
            }
        })
        .collect()
}

fn format_my_leave_date(date_time: DateTime<Utc>) -> String {
    let day_ending = match date_time.day() {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        4..=20 | 24..=30 => "th",
        _ => panic!("Unknown day: {}", date_time.day()),
    };
    format!(
        "{}{}{}",
        date_time.format("%A, %e"),
        day_ending,
        date_time.format(" of %B, %Y")
    )
}
