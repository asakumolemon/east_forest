use actix_web::{web};
pub mod user_api;
pub mod auth_api;
pub mod prompt_api;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(user_api::configure)
            .configure(prompt_api::configure)
    );
}
