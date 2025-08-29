use actix_web::{web, HttpResponse, Responder};
use crate::services::user_service::{UserService};
use crate::models::user::{CreateUserRequest, UpdateUserRequest, DeleteUserRequest, UserQuery};
use crate::models::AppState;

pub struct UserApi { 
    pub service: UserService,
}

pub fn configure(cfg: &mut web::ServiceConfig) { 
    cfg
    .service(
        web::resource("/users")
            .route(web::get().to(get_all_users))
    )
    .service(web::resource("/user")
        .route(web::post().to(create_user))
        .route(web::delete().to(delete_user))
        .route(web::put().to(update_user))
        .route(web::get().to(get_user))
    );
}

async fn update_user(
    update_user: web::Json<UpdateUserRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder { 
    let user = app_state.user_service.update(update_user.into_inner()).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error updating user: {:?}", err)),
    }
}

async fn delete_user(
    delete_user: web::Json<DeleteUserRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder { 
    let user = app_state.user_service.delete(delete_user.into_inner()).await;
    match user {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
    }
}

async fn get_all_users(
    query: web::Query<UserQuery>,
    app_state: web::Data<AppState>,
) -> impl Responder { 
    let users = app_state.user_service.get_all(query.into_inner()).await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error getting users: {:?}", err)),
    }
}

async fn create_user(
    user_data: web::Json<CreateUserRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let user_service = app_state.user_service.clone();

    match user_service.create(user_data.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error creating user: {:?}", e)),
    }
}

async fn get_user(
    query: web::Query<UserQuery>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let user = app_state.user_service.get_user(query.into_inner()).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error getting user: {:?}", e)),
    }
}
