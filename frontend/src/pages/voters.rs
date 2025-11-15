use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Voters)]
pub fn voters() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::SignUp));
    html! {
        <div>
            <h1>{ "Voters" } </h1>
            <button {onclick}>{"Go Home"}</button>
    </div>
    }
}
