use std::ops::Deref;

use crate::{api::get_my_leave, components::organisms::navbar::Navbar, router::Route};
use gloo::console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[function_component(MyLeave)]
pub fn component() -> Html {
    let is_loaded = use_state(|| false);
    let my_leaves = use_state(Vec::new);

    let is_loaded_clone = is_loaded.clone();
    let my_leaves_clone = my_leaves.clone();

    use_effect(move || {
        let my_leaves_clone = my_leaves_clone.clone();
        if !*is_loaded_clone {
            let my_leaves_clone = my_leaves_clone.clone();
            let is_loaded_clone = is_loaded_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let my_leaves_clone = my_leaves_clone.clone();
                let is_loaded_clone = is_loaded_clone.clone();
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
                            <th scope="col">{ "Leave Status" }</th>
                            <th scope="col">{ "Date From" }</th>
                            <th scope="col">{ "Date To" }</th>
                            <th scope="col">{ "Total Duration" }</th>
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

fn my_leave_html(my_leaves: &Vec<crate::model::my_leave::MyLeaveResponse>) -> Vec<Html> {
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
                        { &my_leave.date_from }
                    </td>
                    <td>
                        { &my_leave.date_to }
                    </td>
                    <td>
                        { &my_leave.total_duration }
                    </td>
                </tr>
            }
        })
        .collect()
}
