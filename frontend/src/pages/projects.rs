use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Projects)]
pub fn project_dashboard() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "projects-dashboard" } </h1>
            <button {onclick}>{"Go Home"}</button>
    </div>
    }
}
