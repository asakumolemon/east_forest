use actix_web::{web, HttpResponse, Responder};
use crate::services::user_service::{self, UserService};
use crate::database::{self, repositories::UserRepository};
use crate::models::user::{CreateUserRequest, UpdateUserRequest, DeleteUserRequest, AuthUserRequest, AuthUserResponse, UserQuery};
use crate::utils::auth_util::verify_jwt;

pub struct UserApi { 
    pub service: UserService,
}

pub fn configure(cfg: &mut web::ServiceConfig) { 
    cfg
    .service(
        web::resource("/users")
            .route(web::post().to(create_user))
            .route(web::get().to(get_all_users))
    )
    .service(web::resource("/user")
        .route(web::delete().to(delete_user))
        .route(web::put().to(update_user))
    );
}

async fn update_user(
    path: web::Path<UpdateUserRequest>,
    user_service: web::Data<UserService>,
) -> impl Responder { 
    let user = user_service.update(path.into_inner()).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error updating user: {:?}", err)),
    }
}

async fn delete_user(
    path: web::Path<DeleteUserRequest>,
    user_service: web::Data<UserService>,
) -> impl Responder { 
    let user = user_service.delete(path.into_inner()).await;
    match user {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
    }
}

async fn get_all_users(
    query: web::Query<UserQuery>,
    user_service: web::Data<UserService>,
) -> impl Responder { 
    let users = user_service.get_all(query.into_inner()).await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error getting users: {:?}", err)),
    }
}

async fn create_user(
    user_data: web::Json<CreateUserRequest>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    let user_service = user_service.into_inner();

    match user_service.create(user_data.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error creating user: {:?}", e)),
    }
}

async fn get_user(
    query: web::Query<UserQuery>,
    user_service: web::Data<UserService>,
) -> impl Responder {
    let user = user_service.get_user(query.into_inner()).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error getting user: {:?}", e)),
    }
}
