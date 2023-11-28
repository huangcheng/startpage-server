use log::error;
use std::ops::Deref;

use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;
use rocket_db_pools::Connection;

use crate::config::Config;
use crate::middlewares::JwtMiddleware;
use crate::request;
use crate::response;
use crate::response::auth::Logout;
use crate::state::AppState;
use crate::{handlers, Db};

#[post("/login", format = "json", data = "<user>")]
pub async fn login(
    user: Json<request::auth::User<'_>>,
    state: &State<AppState>,
    config: &State<Config>,
    mut db: Connection<Db>,
) -> Result<response::auth::JwtToken, Status> {
    let token = handlers::auth::login(user.deref(), state, config, &mut db)
        .await
        .map_err(|e| {
            error!("{}", e);

            e.into()
        })?;

    Ok(response::auth::JwtToken { token })
}

#[post("/logout")]
pub async fn logout(_jwt: JwtMiddleware) -> Result<Logout, Status> {
    Ok(Logout)
}
