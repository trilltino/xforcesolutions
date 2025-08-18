use crate::routes::Route;
use gloo_net::http::Request;
use shared::dto::user::{SignUpRequest, SignUpResponse};
use web_sys::{HtmlInputElement, SubmitEvent};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(SignUp)]
pub fn sign_up() -> Html {
    let email = use_state(|| String::new());
    let display_name = use_state(|| String::new());
    let password = use_state(|| String::new());

    let on_email = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_display_name = {
        let display_name = display_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            display_name.set(input.value());
        })
    };

    let on_password = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let email = email.clone();
        let display_name = display_name.clone();
        let password = password.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let email_v = (*email).clone();
            let display_name_v = (*display_name).clone();
            let password_v = (*password).clone();

            wasm_bindgen_futures::spawn_local(async move {
                let req = SignUpRequest {
                    email: email_v,
                    display_name: display_name_v,
                    password: password_v,
                };

                let resp: SignUpResponse = Request::post("/api/users")
                    .header("content-type", "application/json")
                    .json(&req)
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                web_sys::console::log_1(&format!("User created: {}", resp.user.id).into());
            });
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <input type="email" placeholder="Email" value={(*email).clone()} oninput={on_email} required=true />
            <input type="text" placeholder="Display name" value={(*display_name).clone()} oninput={on_display_name} required=true />
            <input type="password" placeholder="Password" value={(*password).clone()} oninput={on_password} required=true />
            <button type="submit">{ "Sign up" }</button>
        </form>
    }
}

#[function_component(Home)]

pub fn home() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Home" } </h1>
            <SignUp />
            <button {onclick}>{"Go Home"}</button>
    </div>
    }
}
