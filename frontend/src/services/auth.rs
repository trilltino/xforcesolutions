use shared::dto::auth::{Guest, Voter, ProjectOwner, Admin, AuthenticatedUser, UserType};
use serde_json;
use crate::services::api::ApiClient;

pub struct AuthService;

impl AuthService {
    pub async fn register_user(user_type: UserType, display_name: String, email: Option<String>, password: Option<String>, address: Option<String>) -> Result<String, String> {
        let (req_json, endpoint) = match user_type {
            UserType::Guest => {
                let req = Guest {
                    username: display_name,
                };
                (serde_json::to_value(&req).unwrap(), "/api/guest")
            }
            UserType::Voter => {
                let req = Voter {
                    base: AuthenticatedUser {
                        username: display_name,
                        email: email.unwrap_or_default(),
                        password: password.unwrap_or_default(),
                        g_address: address.unwrap_or_default(),
                    }
                };
                (serde_json::to_value(&req).unwrap(), "/api/voter")
            }
            UserType::ProjectOwner => {
                let req = ProjectOwner {
                    base: AuthenticatedUser {
                        username: display_name,
                        email: email.unwrap_or_default(),
                        password: password.unwrap_or_default(),
                        g_address: address.unwrap_or_default(),
                    },
                    project_type: "".to_string(),
                };
                (serde_json::to_value(&req).unwrap(), "/api/project-owner")
            }
            UserType::Admin => {
                let req = Admin {
                    base: AuthenticatedUser {
                        username: display_name,
                        email: email.unwrap_or_default(),
                        password: password.unwrap_or_default(),
                        g_address: address.unwrap_or_default(),
                    },
                    admin_type: "".to_string(),
                };
                (serde_json::to_value(&req).unwrap(), "/api/admin")
            }
        };

        ApiClient::post(endpoint, req_json).await
    }
}