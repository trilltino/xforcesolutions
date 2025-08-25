use sqlx::PgPool;
use crate::database::models::user::User;
use shared::dto::auth::UserType;

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

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, display_name, password_hash, user_type, g_address, project_type, admin_type)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, email, display_name, password_hash, user_type, g_address, project_type, admin_type, created_at
            "#,
            email,
            display_name,
            password_hash,
            user_type_str,
            g_address,
            project_type,
            admin_type
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, display_name, password_hash, user_type, g_address, project_type, admin_type, created_at FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(
            User,
            "SELECT id, email, display_name, password_hash, user_type, g_address, project_type, admin_type, created_at FROM users"
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }
}