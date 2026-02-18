use std::ops::Deref;

use crate::api::leave_types::get_leave_types;
use crate::components::organisms::navbar::Navbar;
use crate::router::Route;
use derbe_shared::dto::response::leave_type::LeaveTypeResponse;
use gloo::console::log;
use wasm_bindgen_futures::wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[function_component(LeaveTypes)]
pub fn component() -> Html {
    let leave_types = use_state(|| None::<Vec<LeaveTypeResponse>>);

    let leave_types_clone = leave_types.clone();
    use_effect(move || {
        if leave_types_clone.is_none() {
            wasm_bindgen_futures::spawn_local(async move {
                match get_leave_types().await {
                    Ok(res) => {
                        log!(JsValue::from("Got leave types!"));
                        leave_types_clone.set(Some(res));
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
                navigator.push(&Route::LeaveTypeDetailsCreate);
            })
        })
    };

    html! {
        <main class="container">
            <Navbar />
            <h1>{ "Leave Types" }</h1>
            <button onclick={ request_onclick }>{ "Create Leave Type"}</button>
            if let Some(leave_types) = leave_types.deref() {
                <table>
                    <thead>
                        <tr>
                            <th scope="col">{ "Leave Type" }</th>
                            <th scope="col">{ "Default Quota" }</th>
                            <th scope="col">{ "Colour" }</th>
                            <th scope="col">{ "Can book in future?" }</th>
                            <th scope="col">{ "Show in \"Others Days\"?" }</th>
                            <th scope="col">{ "Default in \"Others Days\"?" }</th>
                            <th scope="col">{ "Default in \"Days Request\"?" }</th>
                            <th scope="col">{ "Show in \"Approve Days\"?" }</th>
                        </tr>
                    </thead>
                    <tbody>
                    { for leave_type_html(leave_types.deref()) }
                    </tbody>
                </table>
            }
        </main>
    }
}

fn leave_type_html(leave_types: &[LeaveTypeResponse]) -> Vec<Html> {
    leave_types.iter().map(|leave_type| {
        html! {
            <tr>
                <th scope="col">{ &leave_type.name }</th>
                if let Some(quota) = leave_type.default_quota {
                    <td>{ quota }</td>
                } else {
                    <td>{ "N/A"}</td>
                }
                <td><input
                    type="color"
                    value={ leave_type.colour.clone() }
                    disabled={ true }
                />
                </td>
                <td>
                    <input type="checkbox" id={ "future_cb_".to_owned() + &leave_type.id.to_string()}
                        checked={ leave_type.can_book_in_future } disabled=true />
                </td>
                <td>
                    <input type="checkbox" id={ "show_in_others_days_cb_".to_owned() + &leave_type.id.to_string()}
                        checked={ leave_type.show_in_others_days } disabled=true />
                </td>
                <td>
                    <input type="checkbox" id={ "default_in_others_days_cb_".to_owned() + &leave_type.id.to_string()}
                        checked={ leave_type.default_in_others_days } disabled=true />
                </td>
                <td>
                    <input type="checkbox" id={ "default_in_days_request_cb_".to_owned() + &leave_type.id.to_string()}
                        checked={ leave_type.default_in_leave_request } disabled=true />
                </td>
                <td>
                    <input type="checkbox" id={ "show_in_approve_days_cb_".to_owned() + &leave_type.id.to_string()}
                        checked={ leave_type.show_in_approve_days } disabled=true />
                </td>

            </tr>
        }
    }).collect()
}
