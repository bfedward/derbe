use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    configuration::leave_types::LeaveTypes,
    days::{
        all_leave::AllLeave, leave_request_create::LeaveRequestCreate,
        leave_request_details::LeaveRequestDetails, my_leave::MyLeave,
    },
    employees::{
        employee_create::EmployeeCreate, employee_details::EmployeeDetails,
        organisation::Organisation,
    },
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
    #[at("/all-leave")]
    AllLeave,
    #[at("/leave-request")]
    LeaveRequestCreate,
    #[at("/leave-request/:id")]
    LeaveRequestDetails { id: i32 },
    #[at("/organisation")]
    Organisation,
    #[at("/employee/:id")]
    EmployeeDetails { id: i32 },
    #[at("/employee")]
    EmployeeCreate,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::LeaveTypes => html! { <LeaveTypes />},
        Route::MyLeave => html! { <MyLeave />},
        Route::AllLeave => html! { <AllLeave />},
        Route::LeaveRequestCreate => html! { <LeaveRequestCreate />},
        Route::LeaveRequestDetails { id } => html! { <LeaveRequestDetails id={id} /> },
        Route::Organisation => html! { <Organisation />},
        Route::EmployeeDetails { id } => html!( <EmployeeDetails id={id} /> ),
        Route::EmployeeCreate => html! { <EmployeeCreate />},
    }
}
