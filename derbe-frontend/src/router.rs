use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::leave_request_details::LeaveRequestDetails;
use crate::pages::{
    days_request::DaysRequest, home::Home, leave_types::LeaveTypes, my_leave::MyLeave,
};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/leave-types")]
    LeaveTypes,
    #[at("/my-leave")]
    MyLeave,
    #[at("/days-request")]
    DaysRequest,
    #[at("/leave-request/:id")]
    LeaveRequestDetails { id: i32 },
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::LeaveTypes => html! { <LeaveTypes />},
        Route::MyLeave => html! { <MyLeave />},
        Route::DaysRequest => html! { <DaysRequest />},
        Route::LeaveRequestDetails { id } => html! { <LeaveRequestDetails id={id} /> },
    }
}
