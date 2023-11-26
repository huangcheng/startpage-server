use sqlx::query_as;
use bcrypt::verify;
use log::error;
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::request;
use crate::models;
use crate::state::AppState;
use crate::errors::ServiceError;
use crate::Claims;

pub async fn login(user: &request::auth::User<'_>, state: &AppState) -> Result<String, ServiceError> {
   let record = query_as::<_, models::user::User>(
       r#"SELECT username, nickname, password, avatar, email FROM user WHERE username = ?"#,
   )
       .bind(user.username)
       .fetch_one(&state.pool).await.map_err(|e| {
            error!("Failed to query user: {}", e);

            ServiceError::InternalServerError
        })?;

    let valid = verify(user.password, &record.password).map_err(|e| {
        error!("Failed to verify password: {}", e);


        ServiceError::InternalServerError
    })?;

    if valid {
        let claims = Claims {
            sub: record.username,
            company: String::from("StartPage"),
            exp: state.jwt_expiration as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
        ).map_err(|e| {
            error!("Failed to encode token: {}", e);

            ServiceError::InternalServerError
        })?;

        Ok(token)
    } else {
        Err(ServiceError::Unauthorized)
    }
}
