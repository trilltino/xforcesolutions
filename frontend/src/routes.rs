use crate::pages::{about::About, home::Home, projects::Projects, voters::Voters,learn::Learn};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/about")]
    About,

    #[at("/voters")]
    Voters,

    #[at("/projects")]
    Projects,

    #[at("/learn")]
    Learn,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/>},
        Route::Voters => html! { <Voters/>},
        Route::Projects => html! {<Projects />},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::About => html! {<About/>  },
        Route::Learn => html! {<Learn/> },
    }
}
