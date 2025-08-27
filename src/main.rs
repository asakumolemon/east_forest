use actix_web::{web, App, HttpServer};
use east_forest::api::config;
use east_forest::services::user_service::UserService;
use east_forest::database::{repositories::UserRepository};
use east_forest::services::auth_service::AuthService;
use std::sync::Arc;
use east_forest::models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user_repository = UserRepository::new().await;
    let user_service = Arc::new(UserService::new(user_repository.clone()));
    let auth_service = Arc::new(AuthService::new(user_repository));
    let app_state = web::Data::new(AppState { user_service, auth_service });
    
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