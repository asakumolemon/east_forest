use actix_web::{web, App, HttpServer};
use east_forest::api::config;
use east_forest::services::user_service::UserService;
use east_forest::database::{repositories::UserRepository};
use east_forest::services::auth_service::AuthService;

pub struct AppState { 
    pub user_service: UserService,
    pub auth_service: AuthService,
}

#[actix_web::main]
async fn main() {
    let user_service = UserService::new(UserRepository::new().await);
    let auth_service = AuthService::new(UserRepository::new().await);
    let app_state = web::Data::new(AppState { user_service, auth_service });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(config)
    })
    .bind("127.0.0.1:8080")
    .unwrap();
    server.run().await.unwrap();
}