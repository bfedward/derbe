use chrono::NaiveDate;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::utils::date_utils::{naive_date_to_yyyy_mm_dd, naive_date_to_yyyy_mm_dd_next_year_end};

#[derive(Clone, PartialEq)]
pub enum DateType {
    DateFrom,
    DateTo,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub name: String,
    pub label: String,
    pub value: NaiveDate,
    pub date_type: DateType,
    pub other_date: NaiveDate,
    pub min: NaiveDate,
    pub max: NaiveDate,
    pub on_change: Callback<(NaiveDate, bool)>,
}

#[function_component(DDSDateInput)]
pub fn dds_date_input(props: &Props) -> Html {
    let on_change = {
        let props_on_change = props.on_change.clone();
        let props_date_type = props.date_type.clone();
        let props_other_date = props.other_date;

        Callback::from(move |event: Event| {
            if let Some(input_element) = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                let value = input_element.value();
                log!(&value);

                if let Ok(parsed_date) = NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
                    match props_date_type {
                        DateType::DateFrom => {
                            if parsed_date > props_other_date {
                                log!("Dates invalid!");
                                props_on_change.emit((parsed_date, false));
                            } else {
                                log!("Emitting new date from!");
                                props_on_change.emit((parsed_date, true));
                            }
                        }
                        DateType::DateTo => {
                            if parsed_date < props_other_date {
                                log!("Dates invalid!");
                                props_on_change.emit((parsed_date, false));
                            } else {
                                log!("Emitting new date to!");
                                props_on_change.emit((parsed_date, true));
                            }
                        }
                    }
                }
            }
        })
    };

    html! {
        <>
            <div>
                <label for={props.id.clone()}>{ props.label.clone() }</label>
                <input id={props.id.clone()}
                    type="date"
                    name={props.name.clone()}
                    value={ naive_date_to_yyyy_mm_dd(&props.value) }
                    min={ naive_date_to_yyyy_mm_dd(&props.min) }
                    max={ naive_date_to_yyyy_mm_dd_next_year_end(&props.max)}
                    onchange={ on_change }/>
            </div>
        </>
    }
}
