use crate::db::Pool;
use crate::models::user::User;
use actix_session::SessionExt;
use actix_web::{
    Error, HttpResponse,
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
};
use futures_util::future::LocalBoxFuture;
use std::future::{Ready, ready};
use std::rc::Rc;

pub struct CheckLogin;

impl<S, B> Transform<S, ServiceRequest> for CheckLogin
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckLoginMiddleware {
            service: Rc::new(service),
        }))
    }
}
pub struct CheckLoginMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CheckLoginMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        Box::pin(async move {
            let session = request.get_session();
            if let Ok(id) = session.get::<i32>("user_id")
                && let Some(id) = id
                && let Some(pool) = request.app_data::<Data<Pool>>()
            {
                let mut conn = pool.get().await;
                if let Ok(ref mut conn) = conn
                    && User::is_active(id, conn).await
                {
                    let res = service.call(request);
                    // forwarded responses map to "left" body
                    return res.await.map(ServiceResponse::map_into_left_body);
                }
            }
            let (request, _pl) = request.into_parts();
            let response = HttpResponse::Unauthorized()
                .json("not authorized")
                .map_into_right_body();
            Ok(ServiceResponse::new(request, response))
        })
    }
}

pub struct CheckAdminLogin;

impl<S, B> Transform<S, ServiceRequest> for CheckAdminLogin
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckAdminLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckAdminLoginMiddleware {
            service: Rc::new(service),
        }))
    }
}
pub struct CheckAdminLoginMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CheckAdminLoginMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        Box::pin(async move {
            let session = request.get_session();
            if let Ok(id) = session.get::<i32>("user_id")
                && let Some(id) = id
                && let Some(pool) = request.app_data::<Data<Pool>>()
            {
                let mut conn = pool.get().await;
                if let Ok(ref mut conn) = conn
                    && User::is_active_admin(id, conn).await
                {
                    let res = service.call(request);
                    // forwarded responses map to "left" body
                    return res.await.map(ServiceResponse::map_into_left_body);
                }
            }
            let (request, _pl) = request.into_parts();
            let response = HttpResponse::Unauthorized()
                .json("not authorized")
                .map_into_right_body();
            Ok(ServiceResponse::new(request, response))
        })
    }
}
