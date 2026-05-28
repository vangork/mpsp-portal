use crate::db::Pool;
use crate::error::ServiceError;
use crate::models::receiver::{NewReceiver, Receiver};
use actix_web::{
    HttpResponse, Result,
    web::{Data, Json, Path},
};

pub async fn list_receivers(pool: Data<Pool>) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    match Receiver::get_all(&mut conn).await {
        Ok(receiver) => Ok(HttpResponse::Ok().json(receiver)),
        Err(e) => Err(ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
        .into()),
    }
}

pub async fn add_receiver(
    Json(receiver): Json<NewReceiver>,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    match Receiver::add_new_receiver(receiver, &mut conn).await {
        Ok(_) => Ok(HttpResponse::Ok().json("receiver added")),
        Err(e) => Err(ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
        .into()),
    }
}

pub async fn update_receiver(
    receiver_id: Path<i32>,
    Json(receiver): Json<NewReceiver>,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    let receiver_id = receiver_id.into_inner();
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    match Receiver::update_receiver(receiver_id, receiver, &mut conn).await {
        Ok(()) => Ok(HttpResponse::Ok().json("receiver updated")),
        Err(e) => Err(ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
        .into()),
    }
}

pub async fn delete_receiver(receiver_id: Path<i32>, pool: Data<Pool>) -> Result<HttpResponse> {
    let receiver_id = receiver_id.into_inner();
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    match Receiver::delete_receiver(receiver_id, &mut conn).await {
        Ok(()) => Ok(HttpResponse::Ok().json("receiver deleted")),
        Err(e) => Err(ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
        .into()),
    }
}

pub async fn get_default_receiver(pool: Data<Pool>) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    match Receiver::get_default_receiver(&mut conn).await {
        Ok(receiver) => Ok(HttpResponse::Ok().json(receiver)),
        Err(e) => Err(ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
        .into()),
    }
}
