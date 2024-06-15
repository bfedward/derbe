use std::ops::Deref;

use gloo::console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::components::atoms::dds_select::DDSSelect;
use crate::components::atoms::dds_text_input::DDSTextInput;
use crate::components::organisms::navbar::Navbar;
use crate::model::division::Division;
use crate::model::location::Location;
use crate::{api::employees::get_employee, model::employee::Employee};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component(EmployeeDetails)]
pub fn employee_details(props: &Props) -> Html {
    let is_employee_loaded = use_state(|| false);
    let employee = use_state(Employee::default);

    let is_employee_loaded_clone = is_employee_loaded.clone();
    let employee_clone = employee.clone();
    let id = props.id;
    use_effect(move || {
        if !*is_employee_loaded_clone {
            wasm_bindgen_futures::spawn_local(async move {
                match get_employee(&id).await {
                    Ok(res) => {
                        is_employee_loaded_clone.set(true);
                        employee_clone.set(res);
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

    let manager_on_change = {
        let employee = employee.clone();
        Callback::from(move |manager_id: Option<i32>| {
            let mut emp = (*employee).clone();
            emp.manager_id = manager_id;
        })
    };

    let division_on_change = {
        let employee = employee.clone();
        Callback::from(move |division_id: Option<i32>| {
            if let Some(id) = division_id {
                let mut emp = (*employee).clone();
                emp.division_id = id;
            }
        })
    };

    let location_on_change = {
        let employee = employee.clone();
        Callback::from(move |location_id: Option<i32>| {
            if let Some(id) = location_id {
                let mut emp = (*employee).clone();
                emp.location_id = id;
            }
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

    html! {
        <main class="container">
            <Navbar />
            <h1>{ "Employee Details" }</h1>
            if *is_employee_loaded.deref() {
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

                    <DDSSelect<Employee>
                        id="managerSelect"
                        name="managerSelect"
                        label="Manager"
                        selected_id={employee.manager_id}
                        excluded_id={employee.id}
                        is_optional_field=true
                        onchange={manager_on_change}
                    />

                    <DDSSelect<Division>
                        id="divisionSelect"
                        name="divisionSelect"
                        label="Division"
                        selected_id={employee.division_id}
                        excluded_id={None}
                        is_optional_field=false
                        onchange={division_on_change}
                    />

                    <DDSSelect<Location>
                        id="locationSelect"
                        name="locationSelect"
                        label="Location"
                        selected_id={employee.location_id}
                        excluded_id={None}
                        is_optional_field=false
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
