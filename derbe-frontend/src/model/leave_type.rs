use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct LeaveType {
    pub id: Option<i32>,
    pub name: String,
    pub default_quota: Option<f64>,
    pub can_book_in_future: bool,
    pub show_in_others_days: bool,
    pub default_in_others_days: bool,
    pub default_in_leave_request: bool,
    pub show_in_approve_days: bool,
}

impl LeaveType {
    pub fn to_select_options_html(leave_types: &[LeaveType]) -> Vec<Html> {
        leave_types.iter().map(|leave_type| {
            html! {
                <option value={ leave_type.id.unwrap().to_string() } selected={ leave_type.default_in_others_days }>{ &leave_type.name }</option>
            }
        }).collect()
    }
}
