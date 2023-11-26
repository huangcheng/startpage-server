use std::ops::Deref;

use rocket::post;
use rocket::State;
use rocket::http::{Status};
use rocket::serde::json::Json;

use crate::request;
use crate::response;
use crate::errors::ServiceError;
use crate::handlers;
use crate::state::AppState;

#[post("/login", format = "json", data = "<user>")]
pub async fn login(user: Json<request::auth::User<'_>>, state: &State<AppState>) -> Result<response::auth::JwtToken, Status> {
    let token = handlers::auth::login(user.deref(), state).await.map_err(|e| {
        match e {
            ServiceError::NotFound => Status::NotFound,
            ServiceError::Unauthorized => Status::Unauthorized,
            _ => Status::InternalServerError,
        }
    })?;

    Ok(response::auth::JwtToken { token })
}
