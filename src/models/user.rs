use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub avatar_url: String,
    pub bio: String,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserView {
    pub id: String,
    pub username: String,
    pub email: String,
    pub avatar_url: String,
    pub bio: String,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CreateUserRequest { 
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UpdateUserRequest {
    pub id: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeleteUserRequest { 
    pub id: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuthUserRequest { 
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuthUserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub avatar_url: String,
    pub bio: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserQuery {
    pub id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
}


