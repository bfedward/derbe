use derbe_shared::dto::response::leave_type::LeaveTypeResponse;

use yew::prelude::*;

use crate::api::{
    api_errors::ApiError,
    leave_types::{get_leave_type, get_leave_types},
};

use super::Selectable;

impl Selectable for LeaveTypeResponse {
    fn get_id(&self) -> i32 {
        self.id
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
                    <option value={ leave_type.id.to_string() }
                        selected={ leave_type.default_in_others_days }>{ &leave_type.get_name() }
                    </option>
                }
            })
            .collect()
    }
}
