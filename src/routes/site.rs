use std::ops::Deref;

use log::error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use rocket_db_pools::Connection;

use crate::handlers::site;
use crate::handlers::site::get_sites;
use crate::middlewares::JwtMiddleware;
use crate::request::site::{CreateSite, UpdateSite};
use crate::response::site::SiteWithCategory;
use crate::response::WithTotal;
use crate::Db;

#[get("/?<page>&<size>")]
pub async fn all(
    page: Option<i64>,
    size: Option<i64>,
    mut db: Connection<Db>,
) -> Result<Json<WithTotal<SiteWithCategory>>, Status> {
    let page = page.unwrap_or(0);

    let size = size.unwrap_or(10);

    let result = get_sites(page, size, &mut db).await.map_err(|e| {
        error!("{}", e);

        e.into()
    })?;

    Ok(Json(result))
}

#[post("/", format = "json", data = "<site>")]
pub async fn add(
    site: Json<CreateSite<'_>>,
    mut db: Connection<Db>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    site::add_site(site.deref(), &mut db).await.map_err(|e| {
        error!("{}", e);

        e.into()
    })?;

    Ok(())
}

#[put("/<id>", format = "json", data = "<site>")]
pub async fn update<'r>(
    id: &'r str,
    site: Json<UpdateSite<'r>>,
    mut db: Connection<Db>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    site::update_site(id, site.deref(), &mut db)
        .await
        .map_err(|e| {
            error!("{}", e);

            e.into()
        })?;

    Ok(())
}

#[delete("/<id>")]
pub async fn delete(id: &str, mut db: Connection<Db>, _jwt: JwtMiddleware) -> Result<(), Status> {
    site::delete_site(id, &mut db).await.map_err(|e| {
        error!("{}", e);

        e.into()
    })?;

    Ok(())
}
