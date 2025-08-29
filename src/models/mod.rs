pub mod user;
pub mod prompt;
pub mod article;
pub mod comment;
pub mod user_interaction;
use crate::services::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState { 
    pub user_service: Arc<user_service::UserService>,
    pub auth_service: Arc<auth_service::AuthService>,
    pub prompt_service: Arc<prompt_service::PromptService>,
    pub article_service: Arc<article_service::ArticleService>,
    pub comment_service: Arc<comment_service::CommentService>,
    pub user_interaction_service: Arc<user_interaction_service::UserInteractionService>,
}