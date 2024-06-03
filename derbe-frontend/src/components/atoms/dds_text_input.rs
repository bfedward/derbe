use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub label: String,
    pub readonly: bool,
    pub value: String,
    pub on_change: Option<Callback<String>>,
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
                    oc.emit(input_element.value());
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
