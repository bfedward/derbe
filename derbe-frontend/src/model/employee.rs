use serde::{Deserialize, Serialize};
use yew::prelude::*;

use super::Selectable;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
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

impl Selectable for Employee {
    fn get_id(&self) -> i32 {
        self.id.unwrap_or(0)
    }

    fn get_name(&self) -> &str {
        &self.first_name
    }

    fn to_select_options_html(items: &[Self], selected_id: i32) -> Vec<Html>
    where
        Self: Sized,
    {
        items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let selected = index == 0 || item.get_id() == selected_id;
                html! {
                    <option value={ item.get_id().to_string() } {selected} >
                        { &item.get_full_name() }
                    </option>
                }
            })
            .collect()
    }
}

impl Employee {
    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
