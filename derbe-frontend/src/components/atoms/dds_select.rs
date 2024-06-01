use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub name: String,
    pub label: String,
    pub options: Vec<Html>,
    pub onchange: Callback<i32>,
}

#[function_component(DDSSelect)]
pub fn dds_select(props: &Props) -> Html {
    let onchange = {
        let props_onchange = props.onchange.clone();
        Callback::from(move |event: Event| {
            if let Some(select_element) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
            {
                let value = select_element.value();
                log!(&value);

                if let Ok(value) = value.parse::<i32>() {
                    props_onchange.emit(value);
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
            <label for={props.id.clone()}>{ props.label.clone() }</label>
            <select id={props.id.clone()} name={props.name.clone()} required=true {onchange}>
                { for props.options.clone() }
            </select>
        </>

    }
}
