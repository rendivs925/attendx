use shared::models::user_model::User;
use shared::types::requests::user::update_user_request::UpdateUserRequest;
use sqlx::{Error, PgPool};

pub struct UserRepository {
    pub pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn register_user(&self, user: &User) -> Result<User, Error> {
        let rec = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, name, email, password) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password)
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn find_user(&self, email: &str) -> Result<Option<User>, Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn update_user(
        &self,
        email: &str,
        update: UpdateUserRequest,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
        UPDATE users
        SET
            name = COALESCE($1, name),
            email = COALESCE($2, email),
            updated_at = now()
        WHERE email = $3
        RETURNING id, name, email, role, status, created_at, updated_at
        "#,
        )
        .bind(update.name)
        .bind(update.email)
        .bind(email)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_user(&self, email: &str) -> Result<(), Error> {
        sqlx::query("DELETE FROM users WHERE email = $1")
            .bind(email)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
