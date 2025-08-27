pub mod user;
pub mod prompt;
pub mod article;
use crate::services::{user_service::UserService, auth_service::AuthService};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState { 
    pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthService>,
}