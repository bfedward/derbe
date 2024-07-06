use std::{collections::HashMap, ops::Deref};

use chrono::{Datelike, Days, Utc};
use gloo::console::log;
use stylist::yew::styled_component;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::{
    api::leave_requests::get_all_leave,
    components::organisms::navbar::Navbar,
    dto::leave_request::{AllLeaveInnerResponse, AllLeaveResponse},
    utils::date_utils::get_first_and_last_date,
};

#[derive(PartialEq)]
enum AllLeaveRowType {
    Upper,
    Lower,
}

#[styled_component(AllLeave)]
pub fn component() -> Html {
    let all_leaves = use_state(|| None::<Vec<AllLeaveResponse>>);

    let all_leaves_clone = all_leaves.clone();
    use_effect(move || {
        if all_leaves_clone.is_none() {
            wasm_bindgen_futures::spawn_local(async move {
                match get_all_leave().await {
                    Ok(res) => {
                        log!(JsValue::from("Got all leave!"));
                        all_leaves_clone.set(Some(res));
                    }
                    Err(error) => {
                        log!(JsValue::from(error.to_string()));
                    }
                }
            })
        }
        || {}
    });

    html! {
        <main class="container">
            <Navbar />
            <h1>{ "All Leave" }</h1>
            if let Some(all_leaves) = all_leaves.deref() {
                <div class="overflow-auto">
                    <table>
                        <thead>
                            <tr>
                                { for table_header_html() }
                            </tr>
                        </thead>
                        <tbody>
                        { for table_row_html(&all_leaves.deref()) }
                        </tbody>
                    </table>
                </div>
            }
        </main>
    }
}

fn table_header_html() -> Vec<Html> {
    let now = Utc::now().naive_utc(); // convert to NaiveDateTime
    let (first, last) = get_first_and_last_date(now.year(), now.month());

    let mut html = vec![html! { <th scope="col">{ "Employee" }</th> }];
    let mut current_date = first;

    while current_date <= last {
        html.push(html! {
            <th scope="col">{ current_date.day() }</th>
        });
        current_date = current_date
            .checked_add_days(Days::new(1))
            .expect("Could not increment day");
    }

    html
}

fn table_row_html(all_leaves: &[AllLeaveResponse]) -> Vec<Html> {
    all_leaves
        .iter()
        .map(|employee_all_leave| {
            html! {
                <>
                    <tr>
                        <th scope="row" rowspan="2">{ &employee_all_leave.employee_name } </th>
                        { table_employee_all_leave_html(&employee_all_leave.leaves, AllLeaveRowType::Upper)}
                    </tr>
                    <tr>
                        { table_employee_all_leave_html(&employee_all_leave.leaves, AllLeaveRowType::Lower)}
                    </tr>
                </>
            }
        })
        .collect()
}

fn table_employee_all_leave_html(
    employee_all_leave: &[AllLeaveInnerResponse],
    row_type: AllLeaveRowType,
) -> Html {
    let now = Utc::now().naive_utc();
    let (_, last) = get_first_and_last_date(now.year(), now.month());

    let mut days: HashMap<u32, &AllLeaveInnerResponse> = HashMap::new();

    for eal in employee_all_leave {
        let mut df = eal.date_from;

        while df <= eal.date_to {
            days.insert(df.day(), eal);
            df = df
                .checked_add_days(Days::new(1))
                .expect("Could not increment day");
            log!(df.to_string());
        }
    }

    html! {
        <>
            { for (1..=last.day()).map(|d| {
                if let Some(&curr_day) = days.get(&d) {
                    if (curr_day.first_day_half && d == curr_day.date_from.day()) || (curr_day.last_day_half && d == curr_day.date_to.day()) {
                        match row_type {
                            AllLeaveRowType::Upper => html! { <td>{ "" }</td> },
                            AllLeaveRowType::Lower => html! {
                                <td style={ format!("background-color:{};color:white;border:1px solid {}", curr_day.colour, curr_day.colour) }>
                                </td>
                            }
                        }
                    } else {
                        html! {
                            <td style={ format!("background-color:{};color:white;border:1px solid {}", curr_day.colour, curr_day.colour) }>
                                { "" }
                            </td>
                        }
                    }
                } else {
                    html! { <td>{ "" }</td> }
                }
            }) }
        </>
    }
}
