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
                                <li><a href="#">{"Settings"}</a></li>
                                <li><a href="#">{"Security"}</a></li>
                                <li><a href="#">{"Logout"}</a></li>
                            </ul>
                        </details>
                    </li>
                    <li>
                        <details class="dropdown">
                            <summary>
                             { "Configuration" }
                            </summary>
                            <ul dir="rtl">
                                <Link<Route> to={Route::LeaveTypes}>{ "Leave Types" }</Link<Route>>
                            </ul>
                        </details>
                    </li>
                </ul>
            </nav>
        </main>
    }
}