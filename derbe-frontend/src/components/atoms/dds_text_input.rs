use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

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
