use crate::routes::Route;
use gloo_net::http::Request;
use shared::{dto::{logindata::{AuthenticatedUser, Admin, ProjectOwner, Voter, Guest}, user::{SignUpRequest, SignUpResponse}}};
use web_sys::SubmitEvent;
use yew::prelude::*;
use yew_router::prelude::*;
use shared::dto::logindata::UserType;
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
                  let user_type = user_type.clone();

                  Callback::from(move |e: SubmitEvent| {
                      e.prevent_default();

                      let email_v = (*email).clone();
                      let display_name_v = (*display_name).clone();
                      let password_v = (*password).clone();
                      let address_v = (*address).clone();
                      let user_type = user_type.clone();

                      wasm_bindgen_futures::spawn_local(async move {
                          let (req_json, endpoint) = match user_type {
                              UserType::Guest => {
                                  let req = Guest {
                                      username: display_name_v,
                                  };
                                  (serde_json::to_value(&req).unwrap(), "/api/guest")
                              }
                              UserType::Voter => {
                                  let req = Voter {
                                      base: AuthenticatedUser {
                                          username: display_name_v,
                                          email: email_v,
                                          password: password_v,
                                          g_address: address_v,
                                      }
                                  };
                                  (serde_json::to_value(&req).unwrap(), "/api/voter")
                              }
                              UserType::ProjectOwner => {
                                  let req = ProjectOwner {
                                      base: AuthenticatedUser {
                                          username: display_name_v,
                                          email: email_v,
                                          password: password_v,
                                          g_address: address_v,
                                      },
                                      project_type: "".to_string(),
                                  };
                                  (serde_json::to_value(&req).unwrap(), "/api/project-owner")
                              }
                              UserType::Admin => {
                                  let req = Admin {
                                      base: AuthenticatedUser {
                                          username: display_name_v,
                                          email: email_v,
                                          password: password_v,
                                          g_address: address_v,
                                      },
                                      admin_type: "".to_string(),
                                  };
                                  (serde_json::to_value(&req).unwrap(), "/api/admin")
                              }
                          };

                          let resp = Request::post(endpoint)
                              .header("content-type", "application/json")
                              .json(&req_json)
                              .unwrap()
                              .send()
                              .await
                              .unwrap()
                              .json::<serde_json::Value>()
                              .await
                              .unwrap();

                          web_sys::console::log_1(
                              &format!("User created: {:?}", resp).into()
                          );
                      });
                  })
              };

              html! {
                  <form onsubmit={on_submit}>
                      <h2>{format!("Sign Up as {}", user_type)}</h2>

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
                      <button type="submit">{"Sign Up"}</button>
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
