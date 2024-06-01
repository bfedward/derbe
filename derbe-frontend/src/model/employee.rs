use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Employee {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub job_title: String,
    pub division_id: i32,
    pub employment_type: String,
    pub location_id: i32,
    pub manager_id: Option<i32>,
    pub email: String,
    pub phone: Option<String>,
}

impl Employee {
    pub fn to_select_options_html(employees: &[Employee]) -> Vec<Html> {
        employees
            .iter()
            .enumerate()
            .map(|(index, employee)| {
                let selected = index == 0;
                html! {
                    <option value={ employee.id.unwrap().to_string() } selected={ selected }>
                        { format!("{} {}", &employee.first_name, &employee.last_name) }
                    </option>
                }
            })
            .collect()
    }
}


