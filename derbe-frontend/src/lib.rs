use std::ops::Deref;

use api::get_leave_types;
use gloo::console::log;
use wasm_bindgen_futures::wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::model::leave_type::LeaveType;

mod store;
mod api;
mod model;
mod pages;

#[function_component]
pub fn App() -> Html {
    let is_loaded = use_state(|| false);
    let leave_types = use_state(Vec::new);

    let leave_types_clone = leave_types.clone();
    
    use_effect(move || {
        let leave_types_clone = leave_types_clone.clone();
        if !*is_loaded {
            let leave_types_clone = leave_types_clone.clone();
            let is_loaded = is_loaded.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let leave_types_clone = leave_types_clone.clone();
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
                is_loaded.set(true);
            });
        }
        || {}
    });

    html! {
        { for leave_type_html(&leave_types.deref().clone()) }
    }
}

fn leave_type_html(leave_types: &Vec<LeaveType>) -> Vec<Html> {
    leave_types.iter().map(|leave_type| {
        html! {
            <>
                <div>
                    <b>{ &leave_type.name }</b>
                    
                </div>
                <div>
                    { "Quota: " }
                    { &leave_type.default_quota.unwrap_or_default() }
                </div>
                <div>
                    <label for={ "can_book_in_future_cb".to_owned() + 
                        &leave_type.id.unwrap().to_string() }>{ "Can book in the future?" }</label>
                    <input type="checkbox" id={ "can_book_in_future_cb".to_owned() + 
                        &leave_type.id.unwrap().to_string() } checked={ leave_type.can_book_in_future }/>
                </div>
                <div>
                    <label for={ "show_in_others_days_cb".to_owned() + 
                        &leave_type.id.unwrap().to_string() }>{ "Show in \"Others Days\"?" }</label>
                    <input type="checkbox" id={ "show_in_others_days_cb".to_owned() + 
                        &leave_type.id.unwrap().to_string() } checked={ leave_type.show_in_others_days }/>
                </div>
                <div>
                    <label for={ "default_in_others_days_cb".to_owned() + 
                        &leave_type.id.unwrap().to_string() }>{ "Default in \"Others Days\"?" }</label>
                    <input type="checkbox" id={ "default_in_others_days_cb".to_owned() + 
                        &leave_type.id.unwrap().to_string() } checked={ leave_type.default_in_others_days }/>
                </div>
                <div>
                    <label for={ "show_in_approve_days_cb".to_owned() + 
                        &leave_type.id.unwrap().to_string() }>{ "Show in \"Approve Days\"?" }</label>
                    <input type="checkbox" id={ "show_in_approve_days_cb".to_owned() + 
                        &leave_type.id.unwrap().to_string() } checked={ leave_type.show_in_approve_days }/>
                </div>
                <br/>
            </>
        }
    }).collect()
}