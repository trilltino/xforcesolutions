use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod routes;
mod ui;
mod services;



use routes::{Route, switch};
use ui::nav::Nav;

#[function_component(App)]
fn app() -> Html {
    html! {
         <HashRouter>
         <Nav/ >
         <div style = {"padding-top: 0px;"}>
         <Switch<Route> render = {switch} />
         </div>
         </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
