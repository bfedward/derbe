use yew::prelude::*;
use crate::components::organisms::navbar::Navbar;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: i32,
}
#[function_component(LeaveRequestDetails)]
pub fn component(props: &Props) -> Html {
    html!{
        <main class="container">
            <Navbar />
            <h1>{ "Leave Request Details" }</h1>
        </main>
    }
}