use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{switch, Route};


mod store;
mod api;
mod model;
mod pages;
mod router;
mod components;

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}