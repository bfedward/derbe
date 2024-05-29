use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::leave_request_create::LeaveRequestCreate;
use crate::pages::leave_request_details::LeaveRequestDetails;
use crate::pages::{home::Home, leave_types::LeaveTypes, my_leave::MyLeave};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/leave-types")]
    LeaveTypes,
    #[at("/my-leave")]
    MyLeave,
    #[at("/leave-request")]
    LeaveRequestCreate,
    #[at("/leave-request/:id")]
    LeaveRequestDetails { id: i32 },
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::LeaveTypes => html! { <LeaveTypes />},
        Route::MyLeave => html! { <MyLeave />},
        Route::LeaveRequestCreate => html! { <LeaveRequestCreate />},
        Route::LeaveRequestDetails { id } => html! { <LeaveRequestDetails id={id} /> },
    }
}
