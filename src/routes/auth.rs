use log::error;
use std::ops::Deref;

use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::Connection;

use crate::config::Config;
use crate::middlewares::jwt::Middleware;
use crate::response::auth::Logout;
use crate::state::AppState;
use crate::{handlers, request, response, MySQLDb, RedisDb};

#[post("/login", format = "json", data = "<user>")]
pub async fn login(
    user: Json<request::auth::User<'_>>,
    state: &State<AppState>,
    config: &State<Config>,
    mut db: Connection<MySQLDb>,
    mut cache: Connection<RedisDb>,
) -> Result<response::auth::JwtToken, Status> {
    let token = handlers::auth::login(user.deref(), state, config, &mut db, &mut cache)
        .await
        .map_err(|e| {
            error!("{}", e);

            e.status()
        })?;

    Ok(response::auth::JwtToken { token })
}

#[post("/logout")]
pub async fn logout(_jwt: Middleware, mut cache: Connection<RedisDb>) -> Result<Logout, Status> {
    let session = _jwt.session.clone();

    cache.del(session).await.map_err(|e| {
        error!("{}", e);

        Status::InternalServerError
    })?;

    Ok(Logout)
}
