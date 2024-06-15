use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::api::{
    api_errors::ApiError,
    employees::{get_employee, get_employees},
};

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

    async fn get_entity(id: &i32) -> Result<Self, ApiError>
    where
        Self: Sized,
    {
        match get_employee(id).await {
            Ok(res) => Ok(res),
            Err(error) => Err(error),
        }
    }

    async fn get_entities() -> Result<Vec<Self>, ApiError>
    where
        Self: Sized,
    {
        match get_employees().await {
            Ok(res) => Ok(res),
            Err(error) => Err(error),
        }
    }

    fn to_select_options_html(
        items: &[Self],
        selected_id: Option<i32>,
        excluded_id: Option<i32>,
    ) -> Vec<Html>
    where
        Self: Sized,
    {
        items
            .iter()
            .filter(|item| Some(item.get_id()) != excluded_id)
            .map(|item| {
                let selected = Some(item.get_id()) == selected_id;
                html! {
                    <option value={ item.get_id().to_string() } {selected} >
                        { &item.get_full_name() }
                    </option>
                }
            })
            .collect()
    }

    fn to_readonly_text_input(&self) -> Html {
        html! {
            <input
                type="text"
                id={self.get_id().to_string()}
                readonly={true}
                value={self.get_full_name().to_string()}
            />
        }
    }
}

impl Employee {
    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
