use yew::prelude::*;
use crate::components::organisms::navbar::Navbar;

#[function_component(DaysRequest)]
pub fn component() -> Html {
    html! {
        <main class="container">
            <Navbar />
            <section id="form">
                <label for="leaveTypeSelect">{ " Leave Type "}</label>
                <select id="leaveTypeSelect" name="leaveTypeSelect" required=true >
                    <option value={ 1 } selected=true >{"Annual Leave"}</option>
                </select>

            </section>
        </main>
    }
}