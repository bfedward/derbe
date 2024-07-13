use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub label: String,
    pub readonly: bool,
    pub min: f64,
    pub value: Option<f64>,
    pub on_change: Option<Callback<Option<f64>>>,
}

#[function_component(DDSNumberInput)]
pub fn dds_text_input(props: &Props) -> Html {
    let on_change = {
        let props_on_change = props.on_change.clone();
        match props_on_change {
            Some(oc) => Callback::from(move |event: Event| {
                if let Some(input_element) = event
                    .target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                {
                    if let Ok(parsed_value) = input_element.value().parse::<f64>() {
                        oc.emit(Some(parsed_value));
                    } else {
                        oc.emit(None);
                    }
                } else {
                    oc.emit(None);
                }
            }),
            None => Callback::from(|_: Event| {}),
        }
    };

    let props_value: String = props.value.map(|v| v.to_string()).unwrap_or_default();

    html! {
        <>
            <label for={props.id.clone()}>{ props.label.clone() }</label>
            <input
                type="number"
                id={props.id.clone()}
                readonly={props.readonly}
                min={props.min.to_string()}
                value={props_value}
                onchange={on_change}
            />
        </>
    }
}
