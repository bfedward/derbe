use yew::prelude::*;
use crate::components::organisms::navbar::Navbar;

#[function_component(Home)]
pub fn component() -> Html {
    html! {
        <main class="container">
            <Navbar />
            { "This is home! "}
        </main>
    }
}