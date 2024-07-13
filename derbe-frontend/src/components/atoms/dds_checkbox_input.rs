use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub name: String,
    pub label: String,
    pub readonly: bool,
    pub value: bool,
    pub on_change: Callback<bool>,
}

#[function_component(DDSCheckboxInput)]
pub fn dds_checkbox_input(props: &Props) -> Html {
    let on_change = {
        let props_on_change = props.on_change.clone();
        Callback::from(move |event: Event| {
            if let Some(input_element) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                props_on_change.emit(input_element.checked());
            }
        })
    };

    html! {
        <>
            <fieldset>
                <input type="checkbox"
                    name={props.name.clone()}
                    id={props.id.clone()}
                    checked={ props.value }
                    disabled={ props.readonly }
                    onchange={ on_change }/>
                { props.label.clone() }
            </fieldset>
        </>
    }
}
