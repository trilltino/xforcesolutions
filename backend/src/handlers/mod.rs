pub mod users;
pub mod projects;
pub mod auth;

pub async fn health() -> &'static str {
    "ok"
}