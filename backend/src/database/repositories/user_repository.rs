use crate::database::models::user::User;
use shared::dto::auth::UserType;
use sqlx::PgPool;

pub struct UserRepository;

impl UserRepository {
    pub async fn create_user(
        pool: &PgPool,
        email: Option<&str>,
        display_name: &str,
        password_hash: Option<&str>,
        user_type: UserType,
        g_address: Option<&str>,
        project_type: Option<&str>,
        admin_type: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        let user_type_str = match user_type {
            UserType::Guest => "Guest",
            UserType::Voter => "Voter",
            UserType::ProjectOwner => "ProjectOwner",
            UserType::Admin => "Admin",
        };

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, display_name, password_hash, user_type, g_address, project_type, admin_type)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, email, display_name, password_hash, user_type, g_address, project_type, admin_type, created_at
            "#,
        )
        .bind(email)
        .bind(display_name)
        .bind(password_hash)
        .bind(user_type_str)
        .bind(g_address)
        .bind(project_type)
        .bind(admin_type)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}
