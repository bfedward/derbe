use derbe_shared::enums::HalfDayOptions;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub name: String,
    pub label: String,
    pub checked: bool,
    pub disabled: bool,
    pub on_change: Callback<HalfDayOptions>,
}

#[function_component(DDSRadioInput)]
pub fn dds_radio_input(props: &Props) -> Html {
    let on_change = {
        let props_on_change = props.on_change.clone();
        Callback::from(move |event: Event| {
            if let Some(input_element) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                let option = HalfDayOptions::from_string(&input_element.id());
                props_on_change.emit(option);
            }
        })
    };

    html! {
        <label>
            <input type="radio"
                name={props.name.clone()}
                id={props.id.clone()}
                checked={ props.checked }
                disabled={ props.disabled }
                onchange={ on_change }/>
            { props.label.clone() }
        </label>
    }
}
