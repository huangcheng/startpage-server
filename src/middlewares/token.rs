use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

pub struct TokenMiddleware {
    pub token: String,
}

#[derive(Debug)]
pub enum TokenError {
    MissingToken,
    InvalidToken,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TokenMiddleware {
    type Error = TokenError;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match request.headers().get_one("Authorization") {
            Some(token) => token,
            None => return Outcome::Error((Status::Unauthorized, TokenError::MissingToken)),
        };

        let token = match token.strip_prefix("Bearer ") {
            Some(token) => token,
            None => return Outcome::Error((Status::Unauthorized, TokenError::InvalidToken)),
        };

        Outcome::Success(Self {
            token: token.to_string(),
        })
    }
}
