use std::ops::Deref;

use gloo::console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::components::atoms::dds_select::DDSSelect;
use crate::components::atoms::dds_text_input::DDSTextInput;
use crate::components::organisms::navbar::Navbar;
use crate::model::division::Division;
use crate::model::location::Location;
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

    let first_name_on_change = {
        let employee = employee.clone();
        Callback::from(move |new_first_name: Option<String>| {
            if let Some(first_name) = new_first_name {
                let mut emp = (*employee).clone();
                emp.first_name = first_name;
                employee.set(emp.clone());
            }
        })
    };

    let last_name_on_change = {
        let employee = employee.clone();
        Callback::from(move |new_last_name: Option<String>| {
            if let Some(last_name) = new_last_name {
                let mut emp = (*employee).clone();
                emp.last_name = last_name;
                employee.set(emp.clone());
            }
        })
    };

    let job_title_on_change = {
        let employee = employee.clone();
        Callback::from(move |new_job_title: Option<String>| {
            if let Some(job_title) = new_job_title {
                let mut emp = (*employee).clone();
                emp.job_title = job_title;
                employee.set(emp.clone());
            }
        })
    };

    let division_on_change = {
        let employee = employee.clone();
        Callback::from(move |division_id: i32| {
            let mut emp = (*employee).clone();
            emp.division_id = division_id;
        })
    };

    let location_on_change = {
        let employee = employee.clone();
        Callback::from(move |location_id: i32| {
            let mut emp = (*employee).clone();
            emp.location_id = location_id;
        })
    };

    let email_on_change = {
        let employee = employee.clone();
        Callback::from(move |new_email: Option<String>| {
            if let Some(email) = new_email {
                let mut emp = (*employee).clone();
                emp.email = email;
                employee.set(emp.clone());
            }
        })
    };

    let phone_on_change = {
        let employee = employee.clone();
        Callback::from(move |new_phone: Option<String>| {
            let mut emp = (*employee).clone();
            emp.phone = new_phone;
            employee.set(emp.clone());
        })
    };

    let divisions_clone = divisions.clone();

    html! {
        <main class="container">
            <Navbar />
            <h1>{ "Employee Details" }</h1>
            if *is_employee_loaded.deref()
                && *is_managers_loaded.deref()
                && *is_divisons_loaded.deref()
                && *is_locations_loaded.deref() {

                    <DDSTextInput
                        id="firstName"
                        label="First Name"
                        readonly=false
                        value={employee.first_name.clone()}
                        on_change={first_name_on_change}
                    />

                    <DDSTextInput
                        id="lastName"
                        label="Last Name"
                        readonly=false
                        value={employee.last_name.clone()}
                        on_change={last_name_on_change}
                    />

                    <DDSTextInput
                        id="jobTitle"
                        label="Job Title"
                        readonly=false
                        value={employee.job_title.clone()}
                        on_change={job_title_on_change}
                    />

                    <DDSSelect<Division>
                        id="divisionSelect"
                        name="divisionSelect"
                        label="Division"
                        selected_id={employee.division_id}
                        options={divisions_clone.deref().clone()}
                        onchange={division_on_change}
                    />

                    <DDSSelect<Location>
                        id="locationSelect"
                        name="locationSelect"
                        label="Location"
                        selected_id={employee.location_id}
                        options={locations.deref().clone()}
                        onchange={location_on_change}
                    />

                    <DDSTextInput
                        id="email"
                        label="Email"
                        readonly=false
                        value={employee.email.clone()}
                        on_change={email_on_change}
                    />

                    <DDSTextInput
                        id="phone"
                        label="Phone"
                        readonly=false
                        value={employee.phone.clone()}
                        on_change={phone_on_change}
                    />
            }
        </main>

    }
}
