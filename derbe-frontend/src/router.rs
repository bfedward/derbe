use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    configuration::{leave_type_create::LeaveTypeDetailsCreate, leave_types::LeaveTypes},
    days::{
        all_leave::AllLeave, leave_request_create::LeaveRequestDetailsCreate,
        leave_request_details::LeaveRequestDetails, my_leave::MyLeave,
    },
    employees::{
        employee_create::EmployeeDetailsCreate, employee_details::EmployeeDetails,
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
    #[at("/leave-types/new")]
    LeaveTypeDetailsCreate,
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
    EmployeeDetailsCreate,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::LeaveTypes => html! { <LeaveTypes />},
        Route::LeaveTypeDetailsCreate => html! { <LeaveTypeDetailsCreate /> },
        Route::MyLeave => html! { <MyLeave />},
        Route::AllLeave => html! { <AllLeave />},
        Route::LeaveRequestCreate => html! { <LeaveRequestDetailsCreate />},
        Route::LeaveRequestDetails { id } => html! { <LeaveRequestDetails id={id} /> },
        Route::Organisation => html! { <Organisation />},
        Route::EmployeeDetails { id } => html!( <EmployeeDetails id={id} /> ),
        Route::EmployeeDetailsCreate => html! { <EmployeeDetailsCreate />},
    }
}
