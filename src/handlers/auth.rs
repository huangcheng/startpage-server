use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use log::error;
use rocket::futures::TryFutureExt;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::Connection;
use sqlx::query_as;

use crate::config::Config;
use crate::errors::ServiceError;
use crate::request;
use crate::state::AppState;
use crate::utils::calculate_expires;
use crate::Claims;
use crate::{models, MySQLDb, RedisDb};

pub async fn login(
    user: &request::auth::User<'_>,
    state: &AppState,
    config: &Config,
    db: &mut Connection<MySQLDb>,
    cache: &mut Connection<RedisDb>,
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
            sub: record.username.clone(),
            company: String::from("StartPage"),
            exp: calculate_expires(&config.jwt.expires_in)? as usize,
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

        let username = record.username.clone();

        cache
            .pset_ex(
                &username,
                token.clone(),
                state.jwt_expiration.num_milliseconds() as usize,
            )
            .map_err(|e| {
                error!("Failed to set token: {}", e);

                ServiceError::InternalServerError
            })
            .await?;

        Ok(token)
    } else {
        Err(ServiceError::BadRequest(String::from(
            "Invalid username or password",
        )))
    }
}
