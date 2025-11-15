use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Learn)]
pub fn learn() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::SignUp));
    html! {
        <div>
            <h1>{ "Learn" } </h1>
            <button {onclick}>{"Go Home"}</button>
    </div>
    }
}