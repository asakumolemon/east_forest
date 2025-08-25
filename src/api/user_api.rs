use actix_web::{web, HttpResponse};
use crate::services::user_service::UserService;
use crate::models::user::{CreateUserRequest, UpdateUserRequest, DeleteUserRequest, AuthUserRequest, AuthUserResponse, UserQuery};
use crate::utils::auth_util::verify_jwt;

pub fn configure(cfg: &mut web::ServiceConfig) { 
    cfg.service(web::resource("/users")
        
    );
}