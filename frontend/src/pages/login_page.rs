use crate::routes::Route;
use web_sys::SubmitEvent;
use yew::prelude::*;
use yew_router::prelude::*;
use shared::dto::auth::UserType;
use crate::services::auth::AuthService;
//{ProjectOwner, Admin, Voter, Guest};

#[function_component(UserTypeSelector)]
pub fn user_type_selector() -> Html {
    let selected_type = use_state(|| None::<UserType>);

    let on_select = {
        let selected_type = selected_type.clone();
        Callback::from(move |user_type: UserType| {
            selected_type.set(Some(user_type));
        })
    };

        html! {
            <div class="user-type-selection">
                <h2>{"Select Your User Type"}</h2>

                <div class="type-buttons">
                    <button onclick={
                        let on_select = on_select.clone();
                        Callback::from(move |_| on_select.emit(UserType::Guest))
                    }>
                        {"Guest Access"}
                    </button>

                    <button onclick={
                        let on_select = on_select.clone();
                        Callback::from(move |_| on_select.emit(UserType::Voter))
                    }>
                        {"Voter"}
                    </button>

                    <button onclick={
                        let on_select = on_select.clone();
                        Callback::from(move |_| on_select.emit(UserType::ProjectOwner))
                    }>
                        {"Project Owner"}
                    </button>

                    <button onclick={
                        let on_select = on_select.clone();
                        Callback::from(move |_| on_select.emit(UserType::Admin))
                    }>
                        {"Admin"}
                    </button>
                </div>

                {
                    if let Some(user_type) = (*selected_type).clone() {
                        html! { <SignUp user_type={Some(user_type)} /> }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }


#[derive(Properties, PartialEq)]
pub struct SignUpProps {
    pub user_type: Option<UserType>,
}

 #[function_component(SignUp)]
  pub fn sign_up(props: &SignUpProps) -> Html {
      let email = use_state(String::new);
      let display_name = use_state(String::new);
      let password = use_state(String::new);
      let address = use_state(String::new);
      let loading = use_state(|| false);
      let message = use_state(|| None::<String>);
      let is_success = use_state(|| false);

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

      let on_address = {
          let address = address.clone();
          Callback::from(move |e: InputEvent| {
              let input: web_sys::HtmlInputElement = e.target_unchecked_into();
              address.set(input.value());
          })
      };

      match &props.user_type {
          Some(user_type) => {
              let on_submit = {
                  let email = email.clone();
                  let display_name = display_name.clone();
                  let password = password.clone();
                  let address = address.clone();
                  let loading = loading.clone();
                  let message = message.clone();
                  let is_success = is_success.clone();
                  let user_type = user_type.clone();

                  Callback::from(move |e: SubmitEvent| {
                      e.prevent_default();

                      let email_v = (*email).clone();
                      let display_name_v = (*display_name).clone();
                      let password_v = (*password).clone();
                      let address_v = (*address).clone();
                      let user_type = user_type.clone();
                      let loading = loading.clone();
                      let message = message.clone();
                      let is_success = is_success.clone();

                      // Start loading, clear previous message
                      loading.set(true);
                      message.set(None);

                      wasm_bindgen_futures::spawn_local(async move {
                          let email_opt = if email_v.is_empty() { None } else { Some(email_v) };
                          let password_opt = if password_v.is_empty() { None } else { Some(password_v) };
                          let address_opt = if address_v.is_empty() { None } else { Some(address_v) };

                          match AuthService::register_user(user_type, display_name_v, email_opt, password_opt, address_opt).await {
                              Ok(response) => {
                                  // ✅ Success - show user feedback
                                  message.set(Some("Account created successfully! 🎉".to_string()));
                                  is_success.set(true);
                                  web_sys::console::log_1(
                                      &format!("User created successfully: {}", response).into()
                                  );
                              }
                              Err(error) => {
                                  // ❌ Error - show user feedback  
                                  message.set(Some(format!("Failed to create account: {}", error)));
                                  is_success.set(false);
                                  web_sys::console::log_1(
                                      &format!("Error creating user: {}", error).into()
                                  );
                              }
                          }
                          
                          // Stop loading
                          loading.set(false);
                      });
                  })
              };

              html! {
                  <form onsubmit={on_submit}>
                      <h2>{format!("Sign Up as {}", user_type)}</h2>
                      
                      // Show success/error message
                      {
                          if let Some(msg) = (*message).clone() {
                              html! {
                                  <div style={if *is_success { 
                                      "padding: 1rem; margin-bottom: 1rem; border-radius: 4px; background-color: #d4edda; color: #155724; border: 1px solid #c3e6cb;" 
                                  } else { 
                                      "padding: 1rem; margin-bottom: 1rem; border-radius: 4px; background-color: #f8d7da; color: #721c24; border: 1px solid #f5c6cb;" 
                                  }}>
                                      {msg}
                                  </div>
                              }
                          } else {
                              html! {}
                          }
                      }

                      <input
                          type="text"
                          placeholder="Username"
                          oninput={on_display_name}
                          value={(*display_name).clone()}
                      />

                      if *user_type != UserType::Guest {
                          <>
                              <input
                                  type="email"
                                  placeholder="Email"
                                  oninput={on_email}
                                  value={(*email).clone()}
                              />
                              <input
                                  type="password"
                                  placeholder="Password"
                                  oninput={on_password}
                                  value={(*password).clone()}
                              />
                              <input
                                  type="text"
                                  placeholder="Address"
                                  oninput={on_address}
                                  value={(*address).clone()}
                              />
                          </>
                      }
                      <button type="submit" disabled={*loading}>
                          {
                              if *loading {
                                  html! {
                                      <>
                                          <span style="display: inline-block; width: 16px; height: 16px; border: 2px solid #f3f3f3; border-top: 2px solid #3498db; border-radius: 50%; animation: spin 1s linear infinite; margin-right: 8px;"></span>
                                          {"Creating Account..."}
                                      </>
                                  }
                              } else {
                                  html! { "Sign Up" }
                              }
                          }
                      </button>
                  </form>
              }
          }
          None => {
              html! {
                  <div>{"Please select a user type first"}</div>
              }
          }
      }
  }
