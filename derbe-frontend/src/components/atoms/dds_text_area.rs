use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub placeholder: String,
    pub label: String,
    pub value: String,
    pub small: Option<String>,
    pub on_change: Callback<Option<String>>,
}

#[function_component(DDSTextArea)]
pub fn dds_text_area(props: &Props) -> Html {
    let on_change = {
        let props_on_change = props.on_change.clone();
        Callback::from(move |event: Event| {
            if let Some(input_element) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
            {
                let value = input_element.value();
                log!("Value in the dds_text_area: {}", &value);
                if value.is_empty() {
                    props_on_change.emit(None);
                } else {
                    props_on_change.emit(Some(value));
                }
            }
        })
    };

    html! {
        <>
            <label for={props.name.clone()}>{ props.label.clone() }</label>
            <textarea
                    name={props.name.clone()}
                    placeholder={props.placeholder.clone()}
                    onchange={ on_change }
                    value={props.value.clone()}
            />
            { if let Some(small_text) = props.small.clone() {
                html! {
                    <small>
                        <i>{ small_text }</i>
                    </small>
                }
            } else {
                html! {}
            }}
        </>
    }
}
