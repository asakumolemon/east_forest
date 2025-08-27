use actix_web::{web, HttpResponse};
use crate::models::article::{ArticleQuery, CreateArticleRequest, DeleteArticleRequest, UpdateArticleRequest};
use crate::services::article_service::ArticleService;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/articles")
        .service(web::resource("/").route(web::get().to(get_article)))
    ).service(web::resource("/article")
        .route(web::post().to(create_article))
        .route(web::put().to(update_article))
        .route(web::delete().to(delete_article)))
    .service(web::resource("/article/{id}").route(web::get().to(get_article_by_id)));
}

async fn create_article(
    article_service: web::Data<ArticleService>,
    request: web::Json<CreateArticleRequest>,
) -> HttpResponse {
    let article = article_service.create_article(request.into_inner()).await;
    match article {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

async fn get_article(
    article_service: web::Data<ArticleService>,
    request: web::Json<ArticleQuery>,
) -> HttpResponse {
    let articles = article_service.get_article(request.into_inner()).await;
    match articles {
        Ok(articles) => HttpResponse::Ok().json(articles),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

async fn update_article(
    article_service: web::Data<ArticleService>, 
    request: web::Json<UpdateArticleRequest>,
) -> HttpResponse {
    let article = article_service.update_article(request.into_inner()).await;
    match article {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

async fn delete_article(
    article_service: web::Data<ArticleService>, 
    request: web::Json<DeleteArticleRequest>,
) -> HttpResponse {
    let article = article_service.delete_article(request.into_inner()).await;   
    match article {
        Ok(_) => HttpResponse::Ok().body("Delete success"),
        Err(_) => HttpResponse::InternalServerError().body("Delete failed"),
    }
}

async fn get_article_by_id(
    article_service: web::Data<ArticleService>, 
    id: web::Path<String>,
) -> HttpResponse { 
    let query = ArticleQuery { id: Some(id.into_inner()), ..Default::default() };
    let articles = article_service.get_article(query).await;
    let article = articles.unwrap().get(0).cloned();
    match article {
        Some(article) => HttpResponse::Ok().json(article),
        None => HttpResponse::InternalServerError().body("Internal server error"),
    }
}
