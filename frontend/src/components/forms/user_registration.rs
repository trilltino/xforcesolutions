use yew::prelude::*;
use shared::dto::auth::UserType;

#[derive(Properties, PartialEq)]
pub struct UserRegistrationProps {
    pub on_submit: Callback<(UserType, String, Option<String>, Option<String>, Option<String>)>,
}

#[function_component(UserRegistration)]
pub fn user_registration(props: &UserRegistrationProps) -> Html {
    let user_type = use_state(|| UserType::Guest);
    let display_name = use_state(String::new);
    let email = use_state(String::new);
    let password = use_state(String::new);
    let address = use_state(String::new);

    let on_submit_callback = props.on_submit.clone();
    let on_submit = {
        let user_type = user_type.clone();
        let display_name = display_name.clone();
        let email = email.clone();
        let password = password.clone();
        let address = address.clone();

        Callback::from(move |_| {
            let email_val = if email.is_empty() { None } else { Some((*email).clone()) };
            let password_val = if password.is_empty() { None } else { Some((*password).clone()) };
            let address_val = if address.is_empty() { None } else { Some((*address).clone()) };

            on_submit_callback.emit(((*user_type).clone(), (*display_name).clone(), email_val, password_val, address_val));
        })
    };

    html! {
        <form>
            <h2>{"User Registration"}</h2>
            <p>{"This is a placeholder for the user registration form component"}</p>
            <p>{"Move relevant code from login_page.rs here"}</p>
            <button type="button" onclick={on_submit}>{"Submit"}</button>
        </form>
    }
}