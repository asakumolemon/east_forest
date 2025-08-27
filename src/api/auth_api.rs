use actix_web::{web, HttpResponse};
use crate::models::user::AuthUserRequest;
use crate::services::auth_service::AuthService;

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
    service: web::Data<AuthService>,
) -> HttpResponse {
    let auth_user_request = req.into_inner();
    let auth_user_response = service.auth(auth_user_request).await;
    match auth_user_response {
        Ok(auth_user_response) => HttpResponse::Ok().json(auth_user_response),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}