use actix_web::{web, App, HttpServer};
use east_forest::api::config;
use east_forest::services::prompt_service::PromptService;
use east_forest::services::user_service::UserService;
use east_forest::services::article_service::ArticleService;
use east_forest::database::repositories::*;
use east_forest::services::comment_service::CommentService;
use east_forest::services::auth_service::AuthService;
use east_forest::services::user_interaction_service::UserInteractionService;
use std::sync::Arc;
use east_forest::models::AppState;
use env_logger::Env;
use actix_web::middleware::Logger; // 导入 Logger
use east_forest::middleware::auth_middleware::AuthMiddleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user_repository = UserRepository::new().await;
    let user_service = Arc::new(UserService::new(user_repository.clone()));
    let auth_service = Arc::new(AuthService::new(user_repository));
    
    let prompt_repository = PromptRepository::new().await;
    let prompt_service = Arc::new(PromptService::new(prompt_repository));
    
    let article_repository = ArticleRepository::new().await;
    let article_service = Arc::new(ArticleService::new(article_repository));
    
    let comment_repository = CommentRepository::new().await;
    let comment_service = Arc::new(CommentService::new(comment_repository));
    let user_interaction_repository = UserInteractionRepository::new().await;
    let user_interaction_service = Arc::new(UserInteractionService::new(user_interaction_repository));

    let app_state = web::Data::new(AppState { 
        user_service, 
        auth_service, 
        prompt_service, 
        article_service, 
        comment_service,
        user_interaction_service,
    });

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    log::info!("Starting server at 0.0.0.0:18080");
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .configure(config)
            // .wrap(AuthMiddleware)
    })
    .bind("0.0.0.0:18080")?;

    server.run().await
}