use crate::config::Config;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, FromForm, State};

use crate::middlewares::JwtMiddleware;

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
    println!("{:?}", data.file.content_type().unwrap().extension());
    Ok(Json(String::from("Hello")))
}
