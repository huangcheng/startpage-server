use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

use crate::config::Config;
use crate::Claims;

pub struct JwtMiddleware {
    pub username: String,
}

#[derive(Debug)]
pub enum JwtError {
    ConfigError,
    MissingToken,
    InvalidToken,
    ExpiredToken,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtMiddleware {
    type Error = JwtError;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let config = match request.rocket().figment().extract::<Config>() {
            Ok(config) => config,
            Err(_) => return Outcome::Error((Status::InternalServerError, JwtError::ConfigError)),
        };

        let token = match request.headers().get_one("Authorization") {
            Some(token) => token,
            None => return Outcome::Error((Status::Unauthorized, JwtError::MissingToken)),
        };

        let token = match token.strip_prefix("Bearer ") {
            Some(token) => token,
            None => return Outcome::Error((Status::Unauthorized, JwtError::MissingToken)),
        };

        let token = match decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt.secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token) => token,
            Err(_) => return Outcome::Error((Status::Unauthorized, JwtError::InvalidToken)),
        };

        Outcome::Success(JwtMiddleware {
            username: token.claims.sub,
        })
    }
}
