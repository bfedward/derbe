use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::model::Selectable;

#[derive(Properties, PartialEq)]
pub struct Props<T: Selectable + PartialEq + 'static> {
    pub id: String,
    pub name: String,
    pub label: String,
    pub selected_id: i32,
    pub options: Vec<T>,
    pub onchange: Callback<i32>,
}

#[function_component(DDSSelect)]
pub fn dds_select<T: Selectable + PartialEq + 'static>(props: &Props<T>) -> Html {
    let on_change = {
        let props_onchange = props.onchange.clone();
        let props_options_ids: Vec<i32> = props.options.iter().map(|opt| opt.get_id()).collect();
        Callback::from(move |event: Event| {
            if let Some(select_element) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
            {
                let value = select_element.value();
                log!(&value);

                if let Ok(value) = value.parse::<i32>() {
                    if props_options_ids.iter().find(|id| **id == value).is_some() {
                        props_onchange.emit(value);
                    } else {
                        log!("Can't select an unknown option");
                    }
                } else {
                    log!("Error parsing value as i32");
                }
            } else {
                log!("Error getting select element");
            }
        })
    };

    html! {
        <>
            <label for={props.id.clone()}>{ &props.label }</label>
            <select id={props.id.clone()} name={props.name.clone()} required=true onchange={on_change}>
                { for T::to_select_options_html(&props.options, props.selected_id) }
            </select>
        </>
    }
}
