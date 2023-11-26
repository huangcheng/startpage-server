use std::ops::Deref;

use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;

use crate::handlers;
use crate::middlewares::JwtMiddleware;
use crate::request;
use crate::response;
use crate::response::auth::Logout;
use crate::state::AppState;

#[post("/login", format = "json", data = "<user>")]
pub async fn login(
    user: Json<request::auth::User<'_>>,
    state: &State<AppState>,
) -> Result<response::auth::JwtToken, Status> {
    let token = handlers::auth::login(user.deref(), state)
        .await
        .map_err(|e| e.status())?;

    Ok(response::auth::JwtToken { token })
}

#[post("/logout")]
pub async fn logout(_jwt: JwtMiddleware) -> Result<Logout, Status> {
    Ok(Logout)
}
