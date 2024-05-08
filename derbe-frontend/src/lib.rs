use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{switch, Route};

mod api;
mod components;
mod model;
mod pages;
mod router;
mod store;

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
