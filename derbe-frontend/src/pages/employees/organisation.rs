use std::ops::Deref;

use derbe_shared::dto::response::employee::EmployeeDetailsResponse;
use gloo::console::log;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::components::Link;
use yew_router::hooks::use_navigator;

use crate::api::employees::get_employee_details_table;
use crate::components::organisms::navbar::Navbar;

use crate::router::Route;

#[function_component(Organisation)]
pub fn organisation() -> Html {
    let employees = use_state(|| None::<Vec<EmployeeDetailsResponse>>);

    let employees_clone = employees.clone();
    use_effect(move || {
        if employees_clone.is_none() {
            spawn_local(async move {
                match get_employee_details_table().await {
                    Ok(res) => {
                        log!(JsValue::from("Got employees!"));
                        employees_clone.set(Some(res));
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
                navigator.push(&Route::EmployeeDetailsCreate);
            })
        })
    };

    html! {
        <main class="container">
            <Navbar />
            <h1>{ "Organisation" }</h1>
            <button onclick={ request_onclick }>{ "Create Employee"}</button>
            if let Some(employees) = employees.deref() {
                <table>
                    <thead>
                        <tr>
                            <th scope="col">{ "Name" }</th>
                            <th scope="col">{ "Job Title" }</th>
                            <th scope="col">{ "Employment Type" }</th>
                            <th scope="col">{ "Division" }</th>
                            <th scope="col">{ "Location" }</th>
                            <th scope="col">{ "Manager" }</th>
                            <th scope="col">{ "Email" }</th>
                            <th scope="col">{ "Phone" }</th>
                        </tr>
                    </thead>
                    <tbody>
                    { for table_row_html(&employees.deref()) }
                    </tbody>
                </table>
            }
        </main>
    }
}

fn table_row_html(employees: &[EmployeeDetailsResponse]) -> Vec<Html> {
    employees
        .iter()
        .map(|row| {
            html! {
                <tr>
                    <th scope="col"><Link<Route> to={Route::EmployeeDetails { id: row.id }}>
                            { &row.name }
                        </Link<Route>>
                    </th>
                    <td>{ &row.job_title }</td>
                    <td>{ &row.employment_type }</td>
                    <td>{ &row.division }</td>
                    <td>{ &row.location }</td>
                    <td>{ &row.manager }</td>
                    <td>{ &row.email }</td>
                    <td>{ &row.phone }</td>
                </tr>
            }
        })
        .collect()
}
