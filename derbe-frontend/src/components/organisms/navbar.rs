use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Navbar)]
pub fn component() -> Html {
    html! {
        <main class="container">
            <nav>
                <ul>
                    <li><strong><Link<Route> to={Route::Home}>{ "derbe" }</Link<Route>></strong></li>
                </ul>
                <ul>
                    <li>
                        <details class="dropdown">
                            <summary>
                             { "Days Off" }
                            </summary>
                            <ul dir="rtl">
                                <li><Link<Route> to={Route::MyLeave}>{ "My Leave" }</Link<Route>></li>
                                <li><a href="#">{"All Leave"}</a></li>
                            </ul>
                        </details>
                    </li>
                    <li>
                        <details class="dropdown">
                            <summary>
                             { "Employees" }
                            </summary>
                            <ul dir="rtl">
                                <li><a href="#">{"My Profile"}</a></li>
                                <li><Link<Route> to={Route::Organisation}>{"Organisation"}</Link<Route>></li>
                                <li><a href="#">{"Goals"}</a></li>
                                <li><a href="#">{"Feedback"}</a></li>
                            </ul>
                        </details>
                    </li>
                    <li>
                        <details class="dropdown">
                            <summary>
                             { "Configuration" }
                            </summary>
                            <ul dir="rtl">
                                <li><Link<Route> to={Route::LeaveTypes}>{ "Leave Types" }</Link<Route>></li>
                            </ul>
                        </details>
                    </li>
                </ul>
            </nav>
        </main>
    }
}
