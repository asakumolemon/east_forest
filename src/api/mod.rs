use actix_web::{web};
pub mod user_api;
pub mod auth_api;
pub mod prompt_api;
pub mod article_api;
pub mod comment_api;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(user_api::configure)
            .configure(prompt_api::configure)
            .configure(article_api::configure)
    );
}
