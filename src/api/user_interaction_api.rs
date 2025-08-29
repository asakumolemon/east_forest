use actix_web::{web, HttpResponse};
use crate::models::user_interaction::*;
use crate::models::AppState;


pub fn configure(cfg: &mut web::ServiceConfig) { 
    cfg.service(
        web::scope("/user_interaction")
            .route("", web::post().to(create_user_interaction))
            .route("", web::get().to(get_user_interaction))
            .route("", web::delete().to(delete_user_interaction))
    );
}

async fn create_user_interaction(
    request: web::Json<CreateUserInteractionRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let user_interaction_service = state.user_interaction_service.clone();
    let request = request.into_inner();
    let user_interaction = user_interaction_service.create_user_interaction(request).await;
    match user_interaction {
        Ok(user_interaction) => HttpResponse::Ok().json(user_interaction),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn get_user_interaction(
    query: web::Query<UserInteractionQuery>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let user_interaction_service = state.user_interaction_service.clone();
    let query = query.into_inner();
    let user_interaction = user_interaction_service.get_user_interaction(query).await;
    match user_interaction {
        Ok(user_interaction) => HttpResponse::Ok().json(user_interaction),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn delete_user_interaction(state: web::Data<AppState>,
    request: web::Json<DeleteUserInteractionRequest>,
) -> HttpResponse {
    let user_interaction_service = state.user_interaction_service.clone();
    let request = request.into_inner();
    let user_interaction = user_interaction_service.delete_user_interaction(request).await;
    match user_interaction {
        Ok(_) => HttpResponse::Ok().body("delete success"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}