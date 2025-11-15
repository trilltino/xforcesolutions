use axum::{routing::{get, post}, Router};
use crate::handlers::{health, users, projects, auth};
use crate::database::connection::DbPool;

pub fn create_api_routes() -> Router<DbPool> {
    Router::new()
        .route("/health", get(health))
        .route("/voters", get(users::list_voters))
        .route("/users", post(users::create_user))
        .route("/guest", post(auth::register_guest))
        .route("/voter", post(auth::register_voter))
        .route("/project-owner", post(auth::register_project_owner))
        .route("/admin", post(auth::register_admin))
        .route("/projects", get(projects::list_projects).post(projects::create_project))
}