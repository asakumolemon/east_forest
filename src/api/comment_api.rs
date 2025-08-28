use actix_web::{web, HttpResponse};
use crate::models::comment::*;
use crate::models::AppState;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/comments")
            .route(web::get().to(get_comments))
    ).service(web::resource("/comment")
            .route(web::put().to(update_comment))
            .route(web::delete().to(delete_comment))
            .route(web::post().to(add_comment))
        );
}

async fn get_comments(
    app_state: web::Data<AppState>,
    query: web::Json<CommentQuery>
) -> HttpResponse {
    let comment_views = app_state.comment_service.get_comment(query.into_inner()).await;
    match comment_views {
        Ok(comment_views) => HttpResponse::Ok().json(comment_views),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

async fn add_comment(
    app_state: web::Data<AppState>,
    req: web::Json<CreateCommentRequest>
) -> HttpResponse {
    let comment_view = app_state.comment_service.create_comment(req.into_inner()).await;
    match comment_view {
        Ok(comment_view) => HttpResponse::Ok().json(comment_view),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

async fn update_comment(
    app_state: web::Data<AppState>,
    req: web::Json<UpdateCommentRequest>
) -> HttpResponse {
    let comment_view = app_state.comment_service.update_comment(req.into_inner()).await;
    match comment_view {
        Ok(comment_view) => HttpResponse::Ok().json(comment_view),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

async fn delete_comment(
    app_state: web::Data<AppState>,
    req: web::Json<DeleteCommentRequest>
) -> HttpResponse {
    let comment_view = app_state.comment_service.delete_comment(req.into_inner()).await;
    match comment_view {
        Ok(comment_view) => HttpResponse::Ok().json(comment_view),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}
