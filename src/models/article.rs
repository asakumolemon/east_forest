use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Article { 
    pub id: String,
    pub user_id: String,
    pub prompt_id: String,
    pub title: String,
    pub word_count: i32,
    pub is_public: bool,
    pub content: String,
    pub ai_score: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ArticleView {
    pub id: String,
    pub user_id: String,
    pub prompt_id: String,
    pub title: String,
    pub word_count: i32,
    pub is_public: bool,
    pub content: String,
    pub ai_score: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CreateArticleRequest {
    pub user_id: String,
    pub prompt_id: String,
    pub title: String,
    pub word_count: i32,
    pub is_public: bool,
    pub content: String,
    pub ai_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UpdateArticleRequest {
    pub id: String,
    pub user_id: String,
    pub prompt_id: String,
    pub title: String,
    pub word_count: i32,
    pub is_public: bool,
    pub content: String,
    pub ai_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeleteArticleRequest {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ArticleQuery {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub prompt_id: Option<String>,
    pub title: Option<String>,
    pub word_count: Option<i32>,
    pub is_public: Option<bool>,
    pub content: Option<String>,
    pub ai_score: Option<f64>,
}

impl Default for ArticleQuery {
    fn default() -> Self {
        Self {
            id: None,
            user_id: None,
            prompt_id: None,
            title: None,
            word_count: None,
            is_public: None,
            content: None,
            ai_score: None,
        }
    }
}