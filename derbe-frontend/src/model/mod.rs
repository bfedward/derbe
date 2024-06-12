use yew::prelude::*;

pub mod division;
pub mod employee;
pub mod leave_request;
pub mod leave_type;
pub mod location;

pub trait Selectable {
    fn get_id(&self) -> i32;
    fn get_name(&self) -> &str;
    fn to_select_options_html(items: &[Self], selected_id: Option<i32>, excluded_id: Option<i32>) -> Vec<Html>
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
                        { &item.get_name() }
                    </option>
                }
            })
            .collect()
    }
}
