use std::ops::Deref;

use derbe_shared::dto::response::leave_request::MyLeaveResponse;
use gloo::console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::components::Link;
use yew_router::hooks::use_navigator;

use crate::api::leave_requests::get_my_leave;
use crate::utils::date_utils::format_my_leave_date;
use crate::{components::organisms::navbar::Navbar, router::Route};

#[function_component(MyLeave)]
pub fn component() -> Html {
    let my_leaves = use_state(|| None::<Vec<MyLeaveResponse>>);

    let my_leaves_clone = my_leaves.clone();
    use_effect(move || {
        if my_leaves_clone.is_none() {
            wasm_bindgen_futures::spawn_local(async move {
                match get_my_leave().await {
                    Ok(res) => {
                        log!(JsValue::from("Got my leave!"));
                        my_leaves_clone.set(Some(res));
                    }
                    Err(error) => {
                        log!(JsValue::from(error.to_string()));
                    }
                }
            });
        }
        || {}
    });

    let request_onclick = {
        let navigator = use_navigator().unwrap();
        Callback::from(move |_: MouseEvent| {
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                navigator.push(&Route::LeaveRequestCreate);
            })
        })
    };

    html! {
        <main class="container">
            <Navbar />
            <h1>{ "My Leave" }</h1>
            <button onclick={ request_onclick }>{ "Request Days Off"}</button>
            if let Some(my_leaves) = my_leaves.deref() {
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
                    { for my_leave_html(my_leaves.deref()) }
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
                        { format_my_leave_date(&my_leave.date_from) }
                    </td>
                    <td>
                        { format_my_leave_date(&my_leave.date_to) }
                    </td>
                    <td>
                        { my_leave.total_duration }
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
