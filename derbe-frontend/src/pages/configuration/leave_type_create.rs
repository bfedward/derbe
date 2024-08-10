use derbe_shared::dto::request::leave_type::LeaveTypeCreate;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::api::api_errors::ApiError;
use crate::api::leave_types::create_leave_type;
use crate::components::atoms::{
    dds_checkbox_input::DDSCheckboxInput, dds_colour_input::DDSColourInput,
    dds_number_input::DDSNumberInput, dds_text_input::DDSTextInput,
};
use crate::components::organisms::navbar::Navbar;
use crate::router::Route;

#[function_component(LeaveTypeDetailsCreate)]
pub fn component() -> Html {
    let leave_type = use_state(|| Some(LeaveTypeCreate::default()));

    let on_submit = {
        let history = use_navigator().unwrap();
        let leave_type_clone = leave_type.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            if let Some(leave_type) = (*leave_type_clone).as_ref().cloned() {
                let history = history.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match create_leave_type(&leave_type).await {
                        Ok(_) => {
                            history.push(&Route::LeaveTypes);
                        }
                        Err(ApiError::NotAuthenticated) => {
                            history.push(&Route::Home);
                        }
                        Err(error) => {
                            gloo::console::error!(error.to_string());
                            panic!();
                        }
                    }
                })
            }
        })
    };

    let cancel_onclick = {
        let history = use_navigator().unwrap();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            history.push(&Route::LeaveTypes);
        })
    };

    let name_on_change = {
        let leave_type = leave_type.clone();
        Callback::from(move |new_name: Option<String>| {
            if let Some(name) = new_name {
                if let Some(mut lt) = (*leave_type).as_ref().cloned() {
                    lt.name = name;
                    leave_type.set(Some(lt.clone()));
                }
            }
        })
    };

    let default_quota_change = {
        let leave_type = leave_type.clone();
        Callback::from(move |new: Option<f64>| {
            if let Some(mut lt) = (*leave_type).as_ref().cloned() {
                lt.default_quota = new;
                leave_type.set(Some(lt.clone()));
            }
        })
    };

    let colour_on_change = {
        let leave_type = leave_type.clone();
        Callback::from(move |new: Option<String>| {
            if let Some(n) = new {
                if let Some(mut lt) = (*leave_type).as_ref().cloned() {
                    lt.colour = n;
                    leave_type.set(Some(lt.clone()));
                }
            }
        })
    };

    let can_book_in_future_on_change = {
        let leave_type = leave_type.clone();
        Callback::from(move |new: bool| {
            if let Some(mut lt) = (*leave_type).as_ref().cloned() {
                lt.can_book_in_future = new;
                leave_type.set(Some(lt.clone()));
            }
        })
    };

    let show_in_others_days_on_change = {
        let leave_type = leave_type.clone();
        Callback::from(move |new: bool| {
            if let Some(mut lt) = (*leave_type).as_ref().cloned() {
                lt.show_in_others_days = new;
                leave_type.set(Some(lt.clone()));
            }
        })
    };

    let default_in_others_days_on_change = {
        let leave_type = leave_type.clone();
        Callback::from(move |new: bool| {
            if let Some(mut lt) = (*leave_type).as_ref().cloned() {
                lt.default_in_others_days = new;
                leave_type.set(Some(lt.clone()));
            }
        })
    };

    let default_in_leave_request_on_change = {
        let leave_type = leave_type.clone();
        Callback::from(move |new: bool| {
            if let Some(mut lt) = (*leave_type).as_ref().cloned() {
                lt.default_in_leave_request = new;
                leave_type.set(Some(lt.clone()));
            }
        })
    };

    let show_in_approve_days_on_change = {
        let leave_type = leave_type.clone();
        Callback::from(move |new: bool| {
            if let Some(mut lt) = (*leave_type).as_ref().cloned() {
                lt.show_in_approve_days = new;
                leave_type.set(Some(lt.clone()));
            }
        })
    };

    html! {
        <main class="container">
            <Navbar />
            <h1>{ "New Leave Type" }</h1>
            if let Some(leave_type) = (*leave_type).as_ref() {
                <section id="form">
                    <form onsubmit = {on_submit}>
                        <DDSTextInput
                            id="name"
                            label="Name"
                            readonly=false
                            value={leave_type.name.clone()}
                            on_change={name_on_change}
                        />
                        <DDSNumberInput
                            id="default_quota"
                            label="Default Quota"
                            readonly=false
                            min=0.0
                            value={leave_type.default_quota.clone()}
                            on_change={default_quota_change}
                        />
                        <DDSColourInput
                            id="colour"
                            label="Colour"
                            readonly=false
                            value={leave_type.colour.clone()}
                            on_change={colour_on_change}
                        />
                        <DDSCheckboxInput
                            id="can_book_in_future"
                            name="can_book_in_future"
                            label="Can book in future?"
                            readonly=false
                            value={leave_type.can_book_in_future}
                            on_change={can_book_in_future_on_change}
                        />
                        <DDSCheckboxInput
                            id="show_in_others_days"
                            name="show_in_others_days"
                            label="Show In Others Days?"
                            readonly=false
                            value={leave_type.show_in_others_days}
                            on_change={show_in_others_days_on_change}
                        />
                        <DDSCheckboxInput
                            id="default_in_others_days"
                            name="default_in_others_days"
                            label="Default In Others Days?"
                            readonly=false
                            value={leave_type.default_in_others_days}
                            on_change={default_in_others_days_on_change}
                        />
                        <DDSCheckboxInput
                            id="default_in_leave_request"
                            name="default_in_leave_request"
                            label="Default In Leave Request?"
                            readonly=false
                            value={leave_type.default_in_leave_request}
                            on_change={default_in_leave_request_on_change}
                        />
                        <DDSCheckboxInput
                            id="show_in_approve_days"
                            name="show_in_approve_days"
                            label="Show In Approve Days?"
                            readonly=false
                            value={leave_type.show_in_approve_days}
                            on_change={show_in_approve_days_on_change}
                        />
                        <div role="group">
                            <button onclick={ cancel_onclick } class="secondary">{ "Cancel"}</button>
                            <button>{"Submit"}</button>
                        </div>
                    </form>
                </section>
            }
        </main>
    }
}
