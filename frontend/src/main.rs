use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;
mod pages;
mod routes;
use crate::routes::switch;

#[function_component(App)]
fn app() -> Html {
    html! {
        <HashRouter>
        <Switch<Route> render = {switch} />
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
