use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    let text = use_state(|| String::from("Loading..."));

    {
        let text = text.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match Request::get("/static/about.txt").send().await {
                    Ok(resp) => match resp.text().await {
                        Ok(body) => text.set(body),
                        Err(_) => text.set(String::from("Failed to read about.txt")),
                    },
                    Err(_) => text.set(String::from("Failed to fetch/static/about.txt")),
                }
            });
            || ()
        });
    }

    html! {
     <section>
     <h1>{"About"}</h1>
    { (*text).clone()}
     </section>
    }
}
