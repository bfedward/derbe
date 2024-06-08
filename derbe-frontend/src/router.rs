use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    configuration::leave_types::LeaveTypes,
    days::{
        leave_request_create::LeaveRequestCreate, leave_request_details::LeaveRequestDetails,
        my_leave::MyLeave,
    },
    employees::organisation::Organisation,
    home::Home,
};

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
    #[at("/organisation")]
    Organisation,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::LeaveTypes => html! { <LeaveTypes />},
        Route::MyLeave => html! { <MyLeave />},
        Route::LeaveRequestCreate => html! { <LeaveRequestCreate />},
        Route::LeaveRequestDetails { id } => html! { <LeaveRequestDetails id={id} /> },
        Route::Organisation => html! { <Organisation />},
    }
}
