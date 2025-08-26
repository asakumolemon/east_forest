use actix_web::{web, HttpResponse};
pub mod user_api;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(user_api::configure)
    );
}
