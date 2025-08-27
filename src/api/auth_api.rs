use actix_web::{web, HttpResponse};
use crate::models::user::AuthUserRequest;
use crate::services::auth_service::AuthService;
use crate::models::AppState;

pub struct AuthApi { 
    pub service: AuthService,
}

pub fn configure(cfg: &mut web::ServiceConfig) { 
    cfg
    .service(web::resource("/auth")
        .route(web::post().to(auth))
    );
}

async fn auth(
    req: web::Json<AuthUserRequest>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let auth_user_request = req.into_inner();
    let auth_user_response = app_state.auth_service.auth(auth_user_request).await;
    match auth_user_response {
        Ok(auth_user_response) => HttpResponse::Ok().json(auth_user_response),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}