pub mod user;
pub mod prompt;
pub mod article;
use crate::services::{user_service::UserService, auth_service::AuthService, prompt_service::PromptService, article_service::ArticleService};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState { 
    pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthService>,
    pub prompt_service: Arc<PromptService>,
    pub article_service: Arc<ArticleService>,
}