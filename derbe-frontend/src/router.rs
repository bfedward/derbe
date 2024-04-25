use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::{leave_types::LeaveTypes, home::Home, my_leave::MyLeave, days_request::DaysRequest};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/leave-types")]
    LeaveTypes,
    #[at("/my-leave")]
    MyLeave,
    #[at("/days-request")]
    DaysRequest
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::LeaveTypes => html! { <LeaveTypes />},
        Route::MyLeave => html! { <MyLeave />},
        Route::DaysRequest => html! { <DaysRequest />}
    }
}