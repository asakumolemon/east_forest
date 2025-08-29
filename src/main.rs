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
    
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(config)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind("127.0.0.1:18080")?;
    
    let addr = server.addrs();
    println!("Server started at: {:?}", addr);
    server.run().await
}