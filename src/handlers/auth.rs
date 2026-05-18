use crate::db::Pool;
use crate::error::ServiceError;
use crate::models::user::User;
use actix_session::Session;
use actix_web::{
    HttpResponse, Result,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Credential {
    pub email: String,
    pub password: String,
}

pub async fn login(
    Json(cred): Json<Credential>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user =
        User::login(cred, &mut conn)
            .await
            .map_err(|e| ServiceError::InternalServerError {
                error_message: e.to_string(),
            })?;
    if let Some(user) = user {
        session.insert("user_id", user.id)?;
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ServiceError::Unauthorized {
            error_message: "Invalid username or password".to_string(),
        }
        .into())
    }
}

pub async fn logout(session: Session) -> Result<HttpResponse> {
    session.purge();
    Ok(HttpResponse::Ok().json("logged out"))
}

pub fn get_user_id(session: &Session) -> i32 {
    session.get::<i32>("user_id").unwrap().unwrap()
}
