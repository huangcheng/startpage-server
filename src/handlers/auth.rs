use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use log::error;
use rocket_db_pools::Connection;
use sqlx::query_as;

use crate::config::Config;
use crate::errors::ServiceError;
use crate::request;
use crate::state::AppState;
use crate::Claims;
use crate::{models, Db};

pub async fn login(
    user: &request::auth::User<'_>,
    state: &AppState,
    config: &Config,
    db: &mut Connection<Db>,
) -> Result<String, ServiceError> {
    let record = query_as::<_, models::user::User>(
        r#"SELECT username, nickname, password, avatar, email FROM user WHERE username = ?"#,
    )
    .bind(user.username)
    .fetch_one(&mut ***db)
    .await
    .map_err(|e| {
        error!("Failed to query user: {}", e);

        ServiceError::BadRequest(String::from("Invalid username or password"))
    })?;

    let valid = verify(user.password, &record.password).map_err(|e| {
        error!("Failed to verify password: {}", e);

        ServiceError::BadRequest(String::from("Invalid username or password"))
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
            &EncodingKey::from_secret(config.jwt.secret.as_bytes()),
        )
        .map_err(|e| {
            error!("Failed to encode token: {}", e);

            ServiceError::InternalServerError
        })?;

        Ok(token)
    } else {
        Err(ServiceError::BadRequest(String::from(
            "Invalid username or password",
        )))
    }
}
