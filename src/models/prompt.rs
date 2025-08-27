use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Prompt{
    pub id: String,
    pub title: String,
    pub category: String,
    pub content: String,
    pub difficulty_level: i32,
    pub is_active: bool,
    pub display_date: DateTime<chrono::Utc>,
    pub created_at: DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PromptView {
    pub id: String,
    pub title: String,
    pub category: String,
    pub content: String,
    pub difficulty_lecel: i32,
    pub is_active: bool,
    pub display_date: DateTime<chrono::Utc>,
    pub created_at: DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CreatePromptRequest { 
    pub title: Option<String>,
    pub category: Option<String>,
    pub content: Option<String>,
    pub difficulty_level: Option<i32>,
    pub is_active: Option<bool>,
    pub display_date: Option<DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UpdatePromptRequest {
    pub id: String,
    pub title: Option<String>,
    pub category: Option<String>,
    pub content: Option<String>,
    pub difficulty_level: Option<i32>,
    pub is_active: Option<bool>,
    pub display_date: Option<DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeletePromptRequest { 
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PromptQuery { 
    pub id: Option<String>,
    pub title: Option<String>,
    pub category: Option<String>,
    pub content: Option<String>,
    pub difficulty_lecel: Option<i32>,
}