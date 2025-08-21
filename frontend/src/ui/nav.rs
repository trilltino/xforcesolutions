use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="navbar">
            <Link<Route> to={Route::SignUp} classes="navbar-brand">
                <img src="/static/logo.jpg" alt="Logo" height="36" />
                {"XFsolutions"}
            </Link<Route>>
            <ul class="navbar-nav">
                <li><Link<Route> to={Route::SignUp} classes="nav-link">{"Home"}</Link<Route>></li>
                <li><Link<Route> to={Route::Projects} classes="nav-link">{"Projects"}</Link<Route>></li>
                <li><Link<Route> to={Route::Voters} classes="nav-link">{"Voters"}</Link<Route>></li>
                <li><Link<Route> to={Route::About} classes="nav-link">{"About"}</Link<Route>></li>
                <li><Link<Route> to={Route::Learn} classes="nav-link">{"Learn More"}</Link<Route>></li>
            </ul>
        </nav>
    }
}
