use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::DateTime;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Comment {
    pub id: String,
    pub article_id: String,
    pub user_id: String,
    pub content: String,
    pub likes: i32,
    pub created_at: DateTime<chrono::Utc>
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct CommentQuery {
    pub article_id: Option<String>,
    pub user_id: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct CreateCommentRequest {
    pub article_id: String,
    pub user_id: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UpdateCommentRequest {
    pub id: String,
    pub article_id: Option<String>,
    pub user_id: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct DeleteCommentRequest {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct CommentView {
    pub id: String,
    pub article_id: String,
    pub user_id: String,
    pub username: String,
    pub avatar_url: String,
    pub content: String,
    pub likes: i32,
    pub created_at: DateTime<chrono::Utc>
}

