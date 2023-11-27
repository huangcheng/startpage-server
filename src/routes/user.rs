use log::error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, put, State};
use std::ops::Deref;

use crate::handlers::user::{get_user_by_username, update_user, update_user_password};
use crate::middlewares::JwtMiddleware;
use crate::request::user::{UpdatePassword, UpdateUser};
use crate::response::auth::Logout;
use crate::response::user::User;
use crate::state::AppState;

#[get("/")]
pub async fn me(state: &State<AppState>, jwt: JwtMiddleware) -> Result<Json<User>, Status> {
    let username = jwt.username;

    let user = get_user_by_username(&username, state).await.map_err(|e| {
        error!("{}", e);

        e.into()
    })?;

    Ok(Json(user))
}

#[put("/<username>", format = "json", data = "<user>")]
pub async fn update<'r>(
    username: &'r str,
    user: Json<UpdateUser<'r>>,
    state: &State<AppState>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    update_user(username, user.deref(), state)
        .await
        .map_err(|e| {
            error!("{}", e);

            e.into()
        })?;

    Ok(())
}

#[put("/<username>/password", format = "json", data = "<password>")]
pub async fn update_password<'r>(
    username: &'r str,
    password: Json<UpdatePassword<'r>>,
    state: &State<AppState>,
    _jwt: JwtMiddleware,
) -> Result<Logout, Status> {
    update_user_password(username, password.deref(), state)
        .await
        .map_err(|e| {
            error!("{}", e);

            e.into()
        })?;

    Ok(Logout)
}
