use log::error;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, FromForm, State};

use crate::config::Config;
use crate::handlers;
use crate::middlewares::jwt::JwtMiddleware;

#[derive(FromForm)]
pub struct Upload<'r> {
    // name: &'r str,
    pub file: TempFile<'r>,
}

#[post("/", data = "<data>")]
pub async fn upload(
    data: Form<Upload<'_>>,
    config: &State<Config>,
    _jwt: JwtMiddleware,
) -> Result<Json<String>, Status> {
    let result = handlers::upload::upload(&data.file, &config.upload_dir)
        .await
        .map_err(|e| {
            error!("{}", e);

            e.status()
        })?;

    Ok(Json(format!("{}/{}", config.upload_url, result)))
}
