use sqlx::PgPool;
use crate::models::user::{User, UserView, CreateUserRequest, UpdateUserRequest, DeleteUserRequest, AuthUserRequest, AuthUserResponse, UserQuery};
use crate::utils::auth_util::{hash_password, verify_password, create_jwt, verify_jwt};

use crate::database::connection::create_pool;
use crate::config::database_config::DatabaseConfig;

pub struct UserRepository { 
    pool: PgPool,
}

impl UserRepository { 
    pub async fn new(config: DatabaseConfig) -> Self {
        Self {
            pool: create_pool(config).await,
        }
    }

    pub async fn create(&self, user: CreateUserRequest) -> Result<User, sqlx::Error> {
        let hashed_password = hash_password(&user.password);
        sqlx::query_as("INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id, username, email, created_at, updated_at")
            .bind(user.username)
            .bind(user.email)
            .bind(hashed_password)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn update(&self, user: UpdateUserRequest) -> Result<User, sqlx::Error> {
        let password_hash = hash_password(&user.password.unwrap_or("".to_string()));
        sqlx::query_as("UPDATE users SET name = $1, email = $2, password_hash = $3 WHERE id = $4 RETURNING id, name, email, created_at, updated_at")
            .bind(user.username.unwrap_or("".to_string()))
            .bind(user.email.unwrap_or("".to_string()))
            .bind(password_hash)
            .bind(user.id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn delete(&self, user: DeleteUserRequest) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn auth(&self, auth_user: AuthUserRequest) -> Result<AuthUserResponse, sqlx::Error> { 
        let user: User = sqlx::query_as("SELECT id, username, email, password_hash, avatar_url, bio FROM users WHERE username = $1")
            .bind(auth_user.username)
            .fetch_one(&self.pool)
            .await?;
        let password = user.password_hash;
        if !verify_password(&auth_user.password, &password) {
            return Err(sqlx::Error::RowNotFound);
        }else {
            return Ok(AuthUserResponse {
                id: user.id.to_string(),
                username: user.username,
                email: user.email,
                avatar_url: user.avatar_url,
                bio: user.bio,
                token: create_jwt(user.id),
            });
        }
    }

    pub async fn get_all_users(&self, query: UserQuery) -> Result<Vec<UserView>, sqlx::Error> { 

        let users = sqlx::query_as("SELECT id, username, email, avatar_url, bio FROM users where username LIKE $1 or email LIKE $2 or id = $3")
            .bind(format!("%{}%", query.username.unwrap_or("".to_string())))
            .bind(format!("%{}%", query.email.unwrap_or("".to_string())))
            .bind(query.id.unwrap_or("".to_string()))
            .fetch_all(&self.pool)
            .await?;
        Ok(users.into_iter().map(|user: User| UserView {
            id: user.id,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            bio: user.bio,
        }).collect())
    }


}
