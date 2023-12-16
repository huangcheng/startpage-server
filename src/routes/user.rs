use std::ops::Deref;

use log::error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, put, State};
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::Connection;

use crate::config::Config;
use crate::handlers::user::{get_user, update_user, update_user_password};
use crate::middlewares::jwt::Middleware;
use crate::request::user::{UpdatePassword, UpdateUser};
use crate::response::auth::Logout;
use crate::response::user::User;
use crate::utils::standardize_url;
use crate::{MySQLDb, RedisDb};

#[get("/")]
pub async fn me(
    mut db: Connection<MySQLDb>,
    config: &State<Config>,
    jwt: Middleware,
) -> Result<Json<User>, Status> {
    let username = jwt.session;

    let user = get_user(&username, &config.upload_url, &mut db)
        .await
        .map_err(|e| {
            error!("{}", e);

            e.status()
        })?;

    Ok(Json(user))
}

#[put("/<username>", format = "json", data = "<user>")]
pub async fn update(
    username: &'_ str,
    user: Json<UpdateUser<'_>>,
    config: &State<Config>,
    mut db: Connection<MySQLDb>,
    _jwt: Middleware,
) -> Result<(), Status> {
    let mut user = user.into_inner();

    let avatar = match user.avatar {
        Some(avatar) => standardize_url(avatar, &config.upload_url),
        None => None,
    };

    user.avatar = avatar.as_deref();

    update_user(username, &user, &mut db).await.map_err(|e| {
        error!("{}", e);

        e.status()
    })?;

    Ok(())
}

#[put("/<username>/password", format = "json", data = "<password>")]
pub async fn update_password<'r>(
    username: &'r str,
    password: Json<UpdatePassword<'r>>,
    mut db: Connection<MySQLDb>,
    mut cache: Connection<RedisDb>,
    _jwt: Middleware,
) -> Result<Logout, Status> {
    update_user_password(username, password.deref(), &mut db)
        .await
        .map_err(|e| {
            error!("{}", e);

            e.status()
        })?;

    let keys = cache
        .keys::<String, Vec<String>>(format!("{}:*", username))
        .await
        .map_err(|e| {
            error!("{}", e);

            Status::InternalServerError
        })?;

    for key in keys {
        cache.del(key).await.map_err(|e| {
            error!("{}", e);

            Status::InternalServerError
        })?;
    }

    Ok(Logout)
}
