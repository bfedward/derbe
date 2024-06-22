use gloo::console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::api::api_errors::ApiError;
use crate::api::employees::update_employee;
use crate::components::atoms::dds_select::DDSSelect;
use crate::components::atoms::dds_text_input::DDSTextInput;
use crate::components::organisms::navbar::Navbar;
use crate::model::division::Division;
use crate::model::location::Location;
use crate::router::Route;
use crate::{api::employees::get_employee, model::employee::Employee};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: i32,
}

#[function_component(EmployeeDetails)]
pub fn employee_details(props: &Props) -> Html {
    let employee = use_state(|| None::<Employee>);

    let employee_clone = employee.clone();
    let id = props.id;
    use_effect(move || {
        if employee_clone.is_none() {
            wasm_bindgen_futures::spawn_local(async move {
                match get_employee(&id).await {
                    Ok(res) => {
                        employee_clone.set(Some(res));
                    }
                    Err(error) => {
                        log!(JsValue::from(error.to_string()));
                    }
                }
            })
        }
    });

    let onsubmit = {
        let history = use_navigator().unwrap();
        let employee_clone = employee.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let employee_clone = employee_clone.clone();
            if let Some(updated_employee) = (*employee_clone).as_ref().cloned() {
                let history = history.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match update_employee(&updated_employee).await {
                        Ok(_) => {
                            history.push(&Route::Organisation);
                        }
                        Err(ApiError::NotAuthenticated) => {
                            history.push(&Route::Home);
                        }
                        Err(error) => {
                            gloo::console::error!(error.to_string());
                            panic!();
                        }
                    }
                })
            }
        })
    };

    let cancel_onclick = {
        let history = use_navigator().unwrap();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            history.push(&Route::Organisation);
        })
    };

    let first_name_on_change = {
        let employee = employee.clone();
        Callback::from(move |new_first_name: Option<String>| {
            if let Some(first_name) = new_first_name {
                if let Some(mut emp) = (*employee).as_ref().cloned() {
                    emp.first_name = first_name;
                    employee.set(Some(emp.clone()));
                }
            }
        })
    };

    let last_name_on_change = {
        let employee = employee.clone();
        Callback::from(move |new_last_name: Option<String>| {
            if let Some(last_name) = new_last_name {
                if let Some(mut emp) = (*employee).as_ref().cloned() {
                    emp.last_name = last_name;
                    employee.set(Some(emp.clone()));
                }
            }
        })
    };

    let job_title_on_change = {
        let employee = employee.clone();
        Callback::from(move |new_job_title: Option<String>| {
            if let Some(job_title) = new_job_title {
                if let Some(mut emp) = (*employee).as_ref().cloned() {
                    emp.job_title = job_title;
                    employee.set(Some(emp.clone()));
                }
            }
        })
    };

    let manager_on_change = {
        let employee = employee.clone();
        Callback::from(move |manager_id: Option<i32>| {
            if let Some(mut emp) = (*employee).as_ref().cloned() {
                emp.manager_id = manager_id;
                employee.set(Some(emp.clone()));
            }
        })
    };

    let division_on_change = {
        let employee = employee.clone();
        Callback::from(move |division_id: Option<i32>| {
            if let Some(id) = division_id {
                if let Some(mut emp) = (*employee).as_ref().cloned() {
                    emp.division_id = id;
                    employee.set(Some(emp.clone()));
                }
            }
        })
    };

    let location_on_change = {
        let employee = employee.clone();
        Callback::from(move |location_id: Option<i32>| {
            if let Some(id) = location_id {
                if let Some(mut emp) = (*employee).as_ref().cloned() {
                    emp.location_id = id;
                    employee.set(Some(emp.clone()));
                }
            }
        })
    };

    let email_on_change = {
        let employee = employee.clone();
        Callback::from(move |new_email: Option<String>| {
            if let Some(email) = new_email {
                if let Some(mut emp) = (*employee).as_ref().cloned() {
                    emp.email = email;
                    employee.set(Some(emp.clone()));
                }
            }
        })
    };

    let phone_on_change = {
        let employee = employee.clone();
        Callback::from(move |new_phone: Option<String>| {
            if let Some(mut emp) = (*employee).as_ref().cloned() {
                emp.phone = new_phone;
                employee.set(Some(emp.clone()));
            }
        })
    };

    html! {
        <main class="container">
            <Navbar />
            <h1>{ "Employee Details" }</h1>
            if let Some(employee) = (*employee).as_ref() {
                <section id="form">
                    <form onsubmit = {onsubmit}>
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

                        <div role="group">
                            <button onclick={ cancel_onclick } class="secondary">{ "Cancel"}</button>
                            <button>{"Submit"}</button>
                        </div>
                    </form>
                </section>
            }
        </main>

    }
}
