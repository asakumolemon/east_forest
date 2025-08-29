use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use crate::utils::auth_util::*;

// 定义中间件结构体
pub struct AuthMiddleware;

// 中间件工厂实现
impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

// 中间件服务结构体
pub struct AuthMiddlewareService<S> {
    service: S,
}

// 中间件服务实现
impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        log::info!("{}",req.path());
        if !req.path().starts_with("/api/v1/auth") && !req.path().starts_with("/api/v1/register") {
            let token = req.headers().get("Authorization");
            if token.is_none() {
                return Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized("Authorization header is missing"))
                });
            }
            if !verify_jwt(token.unwrap().to_str().unwrap()) {
                return Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized("Invalid token"))
                });
            }
        }
        
        let fut = self.service.call(req);
        
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
