use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserInteraction {
    pub id: String,
    pub user_id: String,
    pub prompt_id: String,
    pub article_id: String,
    pub comment_id: String,
    pub interaction_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CreateUserInteractionRequest {
    pub user_id: String,
    pub prompt_id: String,
    pub article_id: String,
    pub comment_id: String,
    pub interaction_type: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DeleteUserInteractionRequest { 
    pub id: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserInteractionView { 
    pub id: String,
    pub user_id: String,
    pub prompt_id: String,
    pub article_id: String,
    pub comment_id: String,
    pub interaction_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserInteractionQuery {
    pub user_id: Option<String>,
    pub prompt_id: Option<String>,
    pub article_id: Option<String>,
    pub comment_id: Option<String>,
    pub interaction_type: Option<String>,
}
