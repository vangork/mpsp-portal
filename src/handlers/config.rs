use crate::db::Pool;
use crate::error::ServiceError;
use crate::models::config::{Config, WebConfig};
use actix_web::{
    HttpResponse, Result,
    web::{Data, Json},
};

pub async fn get_web_config(pool: Data<Pool>) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    match Config::get_web_config(&mut conn).await {
        Ok(web_config) => Ok(HttpResponse::Ok().json(web_config)),
        Err(e) => Err(ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
        .into()),
    }
}

pub async fn update_web_config(
    Json(web_config): Json<WebConfig>,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    match Config::update_web_config(web_config, &mut conn).await {
        Ok(_) => Ok(HttpResponse::Ok().json("web config updated")),
        Err(e) => Err(ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
        .into()),
    }
}
