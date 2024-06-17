use gloo::console::log;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::model::Selectable;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub name: String,
    pub label: String,
    pub selected_id: Option<i32>,
    pub excluded_id: Option<i32>,
    pub is_optional_field: bool,
    pub onchange: Callback<Option<i32>>,
}

#[function_component(DDSSelect)]
pub fn dds_select<S: Selectable + 'static>(props: &Props) -> Html {
    let select_options = use_state(|| None::<Vec<S>>);
    let select_options_clone = select_options.clone();

    let props_onchange = props.onchange.clone();
    use_effect(move || {
        if select_options_clone.is_none() {
            wasm_bindgen_futures::spawn_local(async move {
                match S::get_entities().await {
                    Ok(res) => {
                        // Emit onchange event with the first i32 value
                        if let Some(first_value) = res.first().map(|item| item.get_id()) {
                            props_onchange.emit(Some(first_value));
                        } else {
                            log!("No items fetched or no first value found");
                        }
                        select_options_clone.set(Some(res));
                    }
                    Err(error) => {
                        log!(JsValue::from(error.to_string()));
                    }
                }
            });
        }
    });

    let on_change = {
        let props_onchange = props.onchange.clone();
        let props_is_optional = props.is_optional_field;
        Callback::from(move |event: Event| {
            if let Some(select_element) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
            {
                let value = select_element.value();
                log!(&value);

                if let Ok(value) = value.parse::<i32>() {
                    props_onchange.emit(Some(value));
                } else {
                    if props_is_optional {
                        props_onchange.emit(None);
                    } else {
                        log!("Error parsing value as i32");
                    }
                }
            } else {
                log!("Error getting select element");
            }
        })
    };

    html! {
        <>
        if let Some(select_options_vec) = &*select_options {
                <label for={props.id.clone()}>{ &props.label }</label>
                <select id={props.id.clone()} name={props.name.clone()} required={!props.is_optional_field} onchange={on_change}>
                    { if props.selected_id.is_none() && props.is_optional_field {
                        html! {
                            <option value={""} selected=true>{""}</option>
                        }
                    } else if props.selected_id.is_some() && props.is_optional_field {
                        html! {
                            <option value={""}>{""}</option>
                        }
                    } else {
                        html! {}
                    }}
                    { for S::to_select_options_html(select_options_vec, props.selected_id, props.excluded_id) }
                </select>
            }
        </>
    }
}
