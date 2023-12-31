use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use log::error;
use rocket::futures::TryFutureExt;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::Connection;
use sqlx::query_as;
use uuid::Uuid;

#[cfg(feature = "turnstile")]
use reqwest;

use crate::config::Config;
use crate::errors::ServiceError;
use crate::request;
use crate::state::AppState;
use crate::utils::calculate_expires;
use crate::Claims;
use crate::{models, MySQLDb, RedisDb};

#[cfg(feature = "turnstile")]
use crate::response::auth::TurnstileResponse;

pub async fn login(
    user: &request::auth::User<'_>,
    state: &AppState,
    config: &Config,
    _remote_ip: Option<String>,
    db: &mut Connection<MySQLDb>,
    cache: &mut Connection<RedisDb>,
) -> Result<String, ServiceError> {
    #[cfg(feature = "turnstile")]
    {
        let token = match user.token {
            Some(token) => token,
            None => {
                return Err(ServiceError::BadRequest(String::from(
                    "Invalid challenge token",
                )))
            }
        };

        let secret = match &config.turnstile_secret {
            Some(secret) => secret,
            None => {
                return Err(ServiceError::BadRequest(String::from(
                    "Missing Turnstile secret",
                )))
            }
        };

        let url = match &config.turnstile_url {
            Some(url) => url,
            None => {
                return Err(ServiceError::BadRequest(String::from(
                    "Missing Turnstile URL",
                )))
            }
        };

        let idempotency_key = Uuid::new_v4().to_string();

        let params: [(&str, Option<&str>); 4] = [
            ("secret", Some(secret)),
            ("response", Some(token)),
            ("remoteip", _remote_ip.as_deref()),
            ("idempotency_key", Some(&idempotency_key)),
        ];

        let client = reqwest::Client::new();

        let response = client
            .post(url)
            .form(&params)
            .send()
            .map_err(|e| {
                error!("Failed to send request to Turnstile: {}", e);

                ServiceError::InternalServerError
            })
            .await?
            .json::<TurnstileResponse>()
            .map_err(|e| {
                error!("Failed to parse Turnstile response: {}", e);

                ServiceError::InternalServerError
            })
            .await?;

        if !response.success {
            return Err(ServiceError::BadRequest(String::from(
                "Invalid challenge token",
            )));
        }
    }

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

    let session = Uuid::new_v4().to_string();

    let session = format!("{}:{}", record.username, session);

    if valid {
        let claims = Claims {
            sub: session.clone(),
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

        cache
            .pset_ex(
                session,
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
