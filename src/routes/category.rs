use std::ops::Deref;

use log::error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use rocket_db_pools::Connection;

use crate::handlers::category::{
    self, add_category, delete_category, get_categories, update_category,
};
use crate::middlewares::JwtMiddleware;
use crate::request::category::{CreateCategory, UpdateCategory};
use crate::response::category::Category;
use crate::response::site::Site;
use crate::response::WithTotal;
use crate::Db;

#[get("/?<page>&<size>")]
pub async fn all(
    page: Option<i64>,
    size: Option<i64>,
    mut db: Connection<Db>,
) -> Result<Json<WithTotal<Category>>, Status> {
    let page = page.unwrap_or(0);

    let size = size.unwrap_or(10);

    let result = get_categories(page, size, &mut db).await.map_err(|e| {
        error!("{}", e);

        e.into()
    })?;

    Ok(Json(result))
}

#[put("/<id>", format = "json", data = "<category>")]
pub async fn update<'r>(
    id: &'r str,
    category: Json<UpdateCategory<'r>>,
    mut db: Connection<Db>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    update_category(id, category.deref(), &mut db)
        .await
        .map_err(|e| {
            error!("{}", e);

            e.into()
        })?;

    Ok(())
}

#[post("/", format = "json", data = "<category>")]
pub async fn add<'r>(
    category: Json<CreateCategory<'r>>,
    mut db: Connection<Db>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    add_category(category.deref(), &mut db).await.map_err(|e| {
        error!("{}", e);

        e.into()
    })?;

    Ok(())
}

#[delete("/<id>")]
pub async fn delete<'r>(
    id: &'r str,
    mut db: Connection<Db>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    delete_category(id, &mut db).await.map_err(|e| {
        error!("{}", e);

        e.into()
    })?;

    Ok(())
}

#[get("/<id>/sites")]
pub async fn get_sites(id: &str, mut db: Connection<Db>) -> Result<Json<Vec<Site>>, Status> {
    let sites = category::get_sites(id, &mut db).await.map_err(|e| {
        error!("{}", e);

        e.into()
    })?;

    Ok(Json(sites))
}
