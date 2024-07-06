use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::api::{
    api_errors::ApiError,
    leave_types::{get_leave_type, get_leave_types},
};

use super::Selectable;

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct LeaveType {
    pub id: Option<i32>,
    pub name: String,
    pub default_quota: Option<f64>,
    pub colour: String,
    pub can_book_in_future: bool,
    pub show_in_others_days: bool,
    pub default_in_others_days: bool,
    pub default_in_leave_request: bool,
    pub show_in_approve_days: bool,
}

impl Selectable for LeaveType {
    fn get_id(&self) -> i32 {
        self.id.unwrap_or(0)
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    async fn get_entity(id: &i32) -> Result<Self, ApiError>
    where
        Self: Sized,
    {
        match get_leave_type(id).await {
            Ok(res) => Ok(res),
            Err(error) => Err(error),
        }
    }

    async fn get_entities() -> Result<Vec<Self>, ApiError>
    where
        Self: Sized,
    {
        match get_leave_types().await {
            Ok(res) => Ok(res),
            Err(error) => Err(error),
        }
    }

    fn to_select_options_html(
        items: &[Self],
        _selected_id: Option<i32>,
        _excluded_id: Option<i32>,
    ) -> Vec<Html> {
        items
            .iter()
            .map(|leave_type| {
                html! {
                    <option value={ leave_type.id.unwrap().to_string() }
                        selected={ leave_type.default_in_others_days }>{ &leave_type.get_name() }
                    </option>
                }
            })
            .collect()
    }
}
