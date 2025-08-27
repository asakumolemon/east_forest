use actix_web::{web, HttpResponse};
use crate::models::prompt::{CreatePromptRequest, UpdatePromptRequest, DeletePromptRequest, PromptQuery};
use crate::services::prompt_service::PromptService;

pub struct PromptApi { 
    pub service: PromptService,
}

pub fn configure(cfg: &mut web::ServiceConfig) { 
    cfg
    .service(web::resource("/prompts")
        .route(web::get().to(get_all_prompts))
    )
    .service(web::resource("/prompt")
        .route(web::post().to(create_prompt))
        .route(web::delete().to(delete_prompt))
        .route(web::put().to(update_prompt))
    );
}

async fn create_prompt(
    req: web::Json<CreatePromptRequest>,
    service: web::Data<PromptService>,
) -> HttpResponse {
    let create_prompt_request = req.into_inner();
    let prompt = service.create_prompt(create_prompt_request).await;
    match prompt {
        Ok(prompt) => HttpResponse::Ok().json(prompt),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

async fn update_prompt(
    req: web::Json<UpdatePromptRequest>,
    service: web::Data<PromptService>,
) -> HttpResponse { 
    let update_prompt_request = req.into_inner();
    let prompt = service.update_prompt(update_prompt_request).await;
    match prompt {
        Ok(prompt) => HttpResponse::Ok().json(prompt),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

async fn delete_prompt(
    req: web::Json<DeletePromptRequest>,
    service: web::Data<PromptService>,
) -> HttpResponse { 
    let delete_prompt_request = req.into_inner();
    let prompt = service.delete_prompt(delete_prompt_request).await;
    match prompt {
        Ok(prompt) => HttpResponse::Ok().json(prompt),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

async fn get_all_prompts(
    query: web::Query<PromptQuery>,
    service: web::Data<PromptService>,
) -> HttpResponse { 
    let prompts = service.get_prompt(query.into_inner()).await;
    match prompts {
        Ok(prompts) => HttpResponse::Ok().json(prompts),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}
