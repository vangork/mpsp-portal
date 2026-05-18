use super::auth::get_user_id;
use crate::db::Pool;
use crate::error::ServiceError;
use crate::models::user::{NewPassword, NewUser, User};
use actix_session::Session;
use actix_web::{
    HttpResponse, Result,
    web::{Data, Json, Path},
};

pub async fn get_profile(pool: Data<Pool>, session: Session) -> Result<HttpResponse> {
    let user_id = get_user_id(&session);
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    match User::find_user_by_id(user_id, &mut conn).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Err(ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
        .into()),
    }
}

pub async fn update_password(
    Json(password): Json<NewPassword>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let user_id = get_user_id(&session);
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    match User::update_password(user_id, password, &mut conn).await {
        Ok(true) => Ok(HttpResponse::Ok().json("password updated")),
        Ok(false) => Err(ServiceError::BadRequest {
            error_message: "invalid passord".to_string(),
        }
        .into()),
        Err(e) => Err(ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
        .into()),
    }
}

pub async fn list_users(pool: Data<Pool>) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    match User::get_all(&mut conn).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Err(ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
        .into()),
    }
}

pub async fn add_user(Json(user): Json<NewUser>, pool: Data<Pool>) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    match User::add_new_user(user, &mut conn).await {
        Ok(_) => Ok(HttpResponse::Ok().json("user added")),
        Err(e) => Err(ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
        .into()),
    }
}

pub async fn update_user(
    user_id: Path<i32>,
    Json(user): Json<NewUser>,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    let user_id = user_id.into_inner();
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    match User::update_user(user_id, user, &mut conn).await {
        Ok(()) => Ok(HttpResponse::Ok().json("user updated")),
        Err(e) => Err(ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
        .into()),
    }
}
