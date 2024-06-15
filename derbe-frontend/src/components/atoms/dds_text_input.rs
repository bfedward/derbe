use gloo::console::log;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::model::Selectable;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub label: String,
    pub readonly: bool,
    pub value: Option<String>,
    pub on_change: Option<Callback<Option<String>>>,
}

#[function_component(DDSTextInput)]
pub fn dds_text_input(props: &Props) -> Html {
    let on_change = {
        let props_on_change = props.on_change.clone();
        match props_on_change {
            Some(oc) => Callback::from(move |event: Event| {
                if let Some(input_element) = event
                    .target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                {
                    oc.emit(Some(input_element.value()));
                } else {
                    oc.emit(None);
                }
            }),
            None => Callback::from(|_: Event| {}),
        }
    };

    html! {
        <>
            <label for={props.id.clone()}>{ props.label.clone() }</label>
            <input
                type="text"
                id={props.id.clone()}
                readonly={props.readonly}
                value={props.value.clone()}
                onchange={on_change}
            />
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct PropsForSelectable {
    pub id: String,
    pub label: String,
    pub selected_id: i32,
}

#[function_component(DDSTextInputForSelectable)]
pub fn dds_text_input_for_selectable<S: Selectable + 'static>(props: &PropsForSelectable) -> Html {
    let select_option = use_state(|| None::<S>);
    let props_selected_id = props.selected_id;

    let select_option_clone = select_option.clone();
    use_effect(move || {
        wasm_bindgen_futures::spawn_local(async move {
            match S::get_entity(&props_selected_id).await {
                Ok(res) => {
                    select_option_clone.set(Some(res));
                }
                Err(error) => {
                    log!(JsValue::from(error.to_string()));
                }
            }
        });

        || ()
    });

    html! {
        <>
        if let Some(option) = &*select_option {
            <label for={props.id.clone()}>{ props.label.clone() }</label>
            { Selectable::to_readonly_text_input(option) }
        }
        </>
    }
}
