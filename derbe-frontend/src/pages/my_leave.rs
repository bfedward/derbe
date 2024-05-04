use yew::prelude::*;
use yew_router::hooks::use_navigator;
use crate::{components::organisms::navbar::Navbar, router::Route};

#[function_component(MyLeave)]
pub fn component() -> Html {
    let request_onclick = {
        let navigator = use_navigator().unwrap();
        Callback::from(move |_: MouseEvent| {
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                navigator.push(&Route::DaysRequest);
            })
        })
    };


    html! {
        <main class="container">
            <Navbar />
            <h1>{ "My Leave" }</h1>
            <button onclick={ request_onclick }>{ "Request Days Off "}</button>
        </main>
    }
}