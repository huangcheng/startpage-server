use log::error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};
use rocket_db_pools::Connection;

use crate::config::Config;
use crate::handlers::category::{
    self, add_category, delete_category, get_categories, update_category,
};
use crate::middlewares::JwtMiddleware;
use crate::request::category::{CreateCategory, UpdateCategory};
use crate::response::category::Category;
use crate::response::site::Site;
use crate::response::WithTotal;
use crate::utils::standardize_url;
use crate::Db;

#[get("/?<page>&<size>")]
pub async fn all(
    page: Option<i64>,
    size: Option<i64>,
    config: &State<Config>,
    mut db: Connection<Db>,
) -> Result<Json<WithTotal<Category>>, Status> {
    let page = page.unwrap_or(0);

    let size = size.unwrap_or(10);

    let result = get_categories(page, size, &config.upload_url, &mut db)
        .await
        .map_err(|e| {
            error!("{}", e);

            e.into()
        })?;

    Ok(Json(result))
}

#[put("/<id>", format = "json", data = "<category>")]
pub async fn update<'r>(
    id: &'r str,
    category: Json<UpdateCategory<'r>>,
    config: &State<Config>,
    mut db: Connection<Db>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    let mut category = category.into_inner();

    let icon = match category.icon {
        Some(icon) => standardize_url(icon, &config.upload_url),
        None => None,
    };

    category.icon = icon.as_deref();

    update_category(id, &category, &mut db).await.map_err(|e| {
        error!("{}", e);

        e.into()
    })?;

    Ok(())
}

#[post("/", format = "json", data = "<category>")]
pub async fn add<'r>(
    category: Json<CreateCategory<'r>>,
    config: &State<Config>,
    mut db: Connection<Db>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    let mut category = category.into_inner();

    let icon = standardize_url(category.icon, &config.upload_url);

    let icon = match icon {
        Some(icon) => String::from(icon),
        None => String::from(category.icon),
    };

    category.icon = icon.as_str();

    add_category(&category, &mut db).await.map_err(|e| {
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
pub async fn get_sites(
    id: &str,
    config: &State<Config>,
    mut db: Connection<Db>,
) -> Result<Json<Vec<Site>>, Status> {
    let sites = category::get_sites(id, &config.upload_url, &mut db)
        .await
        .map_err(|e| {
            error!("{}", e);

            e.into()
        })?;

    Ok(Json(sites))
}
