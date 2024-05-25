use std::ops::Deref;

use crate::{
    api::{
        employees::get_employee, leave_requests::get_leave_request, leave_types::get_leave_type,
    },
    components::organisms::navbar::Navbar,
    model::{
        employee::Employee,
        leave_request::LeaveRequest,
        leave_type::LeaveType,
    },
};
use gloo::console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component(LeaveRequestDetails)]
pub fn component(props: &Props) -> Html {
    let is_leave_request_loaded = use_state(|| false);
    let is_employee_loaded = use_state(|| false);
    let is_leave_type_loaded = use_state(|| false);
    let leave_request = use_state(LeaveRequest::default);
    let employee = use_state(Employee::default);
    let leave_type = use_state(LeaveType::default);

    let is_leave_request_loaded_clone = is_leave_request_loaded.clone();
    let is_employee_loaded_clone = is_employee_loaded.clone();
    let is_leave_type_loaded_clone = is_leave_type_loaded.clone();
    let leave_request_clone = leave_request.clone();
    let employee_clone = employee.clone();
    let leave_type_clone = leave_type.clone();
    let id = props.id;

    use_effect(move || {
        if !*is_leave_request_loaded_clone
            && !*is_employee_loaded_clone
            && !*is_leave_type_loaded_clone
        {
            wasm_bindgen_futures::spawn_local(async move {
                match get_leave_request(&id).await {
                    Ok(res) => {
                        log!(JsValue::from("Got leave request!"));
                        is_leave_request_loaded_clone.set(true);
                        let res_clone = res.clone();
                        leave_request_clone.set(res_clone);

                        wasm_bindgen_futures::spawn_local(async move {
                            if !*is_leave_type_loaded_clone {
                                match get_leave_type(&res.leave_type_id).await {
                                    Ok(res) => {
                                        log!(JsValue::from("Got leave type!"));
                                        is_leave_type_loaded_clone.set(true);
                                        leave_type_clone.set(res);
                                    }
                                    Err(error) => {
                                        log!(JsValue::from(error.to_string()));
                                    }
                                }
                            }
                            if !*is_employee_loaded_clone {
                                match get_employee(&res.employee_id).await {
                                    Ok(res) => {
                                        log!(JsValue::from("Got employee!"));
                                        is_employee_loaded_clone.set(true);
                                        employee_clone.set(res)
                                    }
                                    Err(error) => {
                                        log!(JsValue::from(error.to_string()));
                                    }
                                }
                            }
                        })
                    }
                    Err(error) => {
                        log!(JsValue::from(error.to_string()));
                    }
                }
            })
        }
        || {}
    });

    html! {
        <main class="container">
            <Navbar />
            <h1>{ "Leave Request Details" }</h1>
            if *is_leave_request_loaded.deref() && *is_employee_loaded.deref() && *is_leave_type_loaded.deref() {
                <h2>{ "loaded everything" }</h2>
            }
        </main>
    }
}
