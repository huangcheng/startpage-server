use std::ops::Deref;

use rocket::post;
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::request;
use crate::response;
use crate::handlers;
use crate::state::AppState;

#[post("/login", format = "json", data = "<user>")]
pub async fn login(user: Json<request::auth::User<'_>>, state: &State<AppState>) -> Result<response::auth::User, Status> {
    handlers::auth::login(user.deref(), state).await
}
