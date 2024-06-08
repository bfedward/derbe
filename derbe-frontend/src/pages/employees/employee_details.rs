use std::ops::Deref;

use gloo::console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::components::organisms::navbar::Navbar;
use crate::{
    api::{
        divisions::get_divisions,
        employees::{get_employee, get_employees},
        locations::get_locations,
    },
    model::employee::Employee,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component(EmployeeDetails)]
pub fn employee_details(props: &Props) -> Html {
    let is_employee_loaded = use_state(|| false);
    let is_managers_loaded = use_state(|| false);
    let is_divisons_loaded = use_state(|| false);
    let is_locations_loaded = use_state(|| false);

    let employee = use_state(Employee::default);
    let managers = use_state(Vec::new);
    let divisions = use_state(Vec::new);
    let locations = use_state(Vec::new);

    let is_employee_loaded_clone = is_employee_loaded.clone();
    let is_managers_loaded_clone = is_managers_loaded.clone();
    let is_divisons_loaded_clone = is_divisons_loaded.clone();
    let is_locations_loaded_clone = is_locations_loaded.clone();

    let employee_clone = employee.clone();
    let managers_clone = managers.clone();
    let divisions_clone = divisions.clone();
    let locations_clone = locations.clone();

    let id = props.id;
    use_effect(move || {
        if !*is_employee_loaded_clone
            && !*is_managers_loaded_clone
            && !*is_divisons_loaded_clone
            && !*is_locations_loaded_clone
        {
            wasm_bindgen_futures::spawn_local(async move {
                match get_employee(&id).await {
                    Ok(res) => {
                        is_employee_loaded_clone.set(true);
                        employee_clone.set(res);

                        wasm_bindgen_futures::spawn_local(async move {
                            if !*is_managers_loaded_clone {
                                match get_employees().await {
                                    Ok(res) => {
                                        is_managers_loaded_clone.set(true);
                                        managers_clone.set(res);
                                    }
                                    Err(error) => {
                                        log!(JsValue::from(error.to_string()));
                                    }
                                }
                            }
                            if !*is_divisons_loaded_clone {
                                match get_divisions().await {
                                    Ok(res) => {
                                        is_divisons_loaded_clone.set(true);
                                        divisions_clone.set(res);
                                    }
                                    Err(error) => {
                                        log!(JsValue::from(error.to_string()));
                                    }
                                }
                            }
                            if !*is_locations_loaded_clone {
                                match get_locations().await {
                                    Ok(res) => {
                                        is_locations_loaded_clone.set(true);
                                        locations_clone.set(res);
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
    });

    html! {
        <main class="container">
            <Navbar />
            <h1>{ "Employee Details" }</h1>
            if *is_employee_loaded.deref()
                && *is_managers_loaded.deref()
                && *is_divisons_loaded.deref()
                && *is_locations_loaded.deref() {

                    <h2>{ "All loaded!" }</h2>
            }
        </main>

    }
}
