use rocket::form::DataField;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::{ContentType, Status};
use rocket::serde::json::Json;
use rocket::{post, Data, FromForm};

use crate::middlewares::JwtMiddleware;

#[derive(FromForm)]
pub struct Upload<'r> {
    // name: &'r str,
    pub file: TempFile<'r>,
}

#[post("/", data = "<data>")]
pub async fn upload(data: Form<Upload<'_>>, _jwt: JwtMiddleware) -> Result<Json<String>, Status> {
    println!("{:?}", data.file.content_type().unwrap().extension());
    Ok(Json(String::from("Hello")))
}
