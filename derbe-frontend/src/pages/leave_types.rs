use std::ops::Deref;

use gloo::console::log;
use wasm_bindgen_futures::wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::components::organisms::navbar::Navbar;
use crate::{api::get_leave_types, model::leave_type::LeaveType};

#[function_component(LeaveTypes)]
pub fn component() -> Html {
    let is_loaded = use_state(|| false);
    let leave_types = use_state(Vec::new);

    let is_loaded_clone = is_loaded.clone();
    let leave_types_clone = leave_types.clone();

    use_effect(move || {
        let leave_types_clone = leave_types_clone.clone();
        if !*is_loaded_clone {
            let leave_types_clone = leave_types_clone.clone();
            let is_loaded_clone = is_loaded_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let leave_types_clone = leave_types_clone.clone();
                let is_loaded_clone = is_loaded_clone.clone();
                match get_leave_types().await {
                    Ok(res) => {
                        let leave_types_clone = leave_types_clone.clone();
                        log!(JsValue::from("Got leave types!"));
                        leave_types_clone.set(res);
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

    html! {
        <main class="container">
            <Navbar />
            if *is_loaded.deref() {
                <table>
                    <thead>
                        <tr>
                            <th scope="col">{ "Leave Type" }</th>
                            <th scope="col">{ "Default Quota" }</th>
                            <th scope="col">{ "Can book in future?" }</th>
                            <th scope="col">{ "Show in \"Others Days\"?" }</th>
                            <th scope="col">{ "Default in \"Others Days\"?" }</th>
                            <th scope="col">{ "Default in \"Days Request\"?" }</th>
                            <th scope="col">{ "Show in \"Approve Days\"?" }</th>
                        </tr>
                    </thead>
                    <tbody>
                    { for leave_type_html(&leave_types.deref().clone()) }
                    </tbody>
                </table>
            }
        </main>
    }
}

fn leave_type_html(leave_types: &Vec<LeaveType>) -> Vec<Html> {
    leave_types.iter().map(|leave_type| {
        html! {
            <tr>
                <th scope="col">{ &leave_type.name }</th>
                if let Some(quota) = leave_type.default_quota {
                    <td>{ quota }</td>
                } else {
                    <td>{ "N/A"}</td>
                }
                <td>
                    <input type="checkbox" id={ "future_cb_".to_owned() + &leave_type.id.unwrap().to_string()}
                        checked={ leave_type.can_book_in_future } disabled=true />
                </td>
                <td>
                    <input type="checkbox" id={ "show_in_others_days_cb_".to_owned() + &leave_type.id.unwrap().to_string()}
                        checked={ leave_type.show_in_others_days } disabled=true />
                </td>
                <td>
                    <input type="checkbox" id={ "default_in_others_days_cb_".to_owned() + &leave_type.id.unwrap().to_string()}
                        checked={ leave_type.default_in_others_days } disabled=true />
                </td>
                <td>
                    <input type="checkbox" id={ "default_in_days_request_cb_".to_owned() + &leave_type.id.unwrap().to_string()}
                        checked={ leave_type.default_in_leave_request } disabled=true />
                </td>
                <td>
                    <input type="checkbox" id={ "show_in_approve_days_cb_".to_owned() + &leave_type.id.unwrap().to_string()}
                        checked={ leave_type.show_in_approve_days } disabled=true />
                </td>

            </tr>
        }
    }).collect()
}
