use std::ops::Deref;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, post, put, delete};

use crate::handlers::category::{add_category, get_all_categories, update_category, delete_category};
use crate::middlewares::JwtMiddleware;
use crate::models::category::Category;
use crate::request::category::{CreateCategory, UpdateCategory};
use crate::state::AppState;

#[get("/")]
pub async fn all(state: &State<AppState>) -> Result<Json<Vec<Category>>, Status> {
    let categories = get_all_categories(state).await.map_err(|e| e.status())?;

    Ok(Json(categories))
}

#[put("/<id>", format = "json", data = "<category>")]
pub async fn update<'r>(
    id: &'r str,
    category: Json<UpdateCategory<'r>>,
    state: &State<AppState>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    update_category(id, category.deref(), state)
        .await
        .map_err(|e| e.status())?;

    Ok(())
}

#[post("/", format = "json", data = "<category>")]
pub async fn add<'r>(
    category: Json<CreateCategory<'r>>,
    state: &State<AppState>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    add_category(category.deref(), state)
        .await
        .map_err(|e| e.status())?;

    Ok(())
}

#[delete("/<id>")]
pub async fn delete<'r>(
    id: &'r str,
    state: &State<AppState>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    delete_category(id, state)
        .await
        .map_err(|e| e.status())?;

    Ok(())
}
