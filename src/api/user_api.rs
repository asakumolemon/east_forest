use actix_web::{web, HttpResponse, Responder};
use crate::services::user_service::{self, UserService};
use crate::database::{self, repositories::UserRepository};
use crate::models::user::{CreateUserRequest, UpdateUserRequest, DeleteUserRequest, AuthUserRequest, AuthUserResponse, UserQuery};
use crate::utils::auth_util::verify_jwt;

pub struct UserApi { 
    pub service: UserService,
}

pub fn configure(cfg: &mut web::ServiceConfig) { 
    cfg.service(
web::resource("/users")
            .route(web::post().to(create_user))
    );
}

async fn create_user(
    user_data: web::Json<CreateUserRequest>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    let CreateUserRequest {
        username,
        password,
        email,
        avatar_url,
        bio,
    } = user_data.into_inner();

    let user_service = user_service.into_inner();

    match user_service.create( &username, &password, &email, &avatar_url, &bio).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error creating user: {:?}", e)),
    }
}