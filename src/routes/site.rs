use log::error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, State};
use rocket_db_pools::Connection;

use crate::config::Config;
use crate::guards::jwt::Middleware;
use crate::handlers::site;
use crate::handlers::site::get_sites;
use crate::request::site::{CreateSite, UpdateSite};
use crate::response::site::{Site, SiteWithCategory};
use crate::response::WithTotal;
use crate::utils::standardize_url;
use crate::MySQLDb;

#[get("/<id>")]
pub async fn get(
    id: i64,
    config: &State<Config>,
    mut db: Connection<MySQLDb>,
) -> Result<Json<Site>, Status> {
    let mut site = site::get_site(id, &mut db).await.map_err(|e| {
        error!("{}", e);

        e.status()
    })?;

    let icon = site.icon.clone();

    let icon = if icon.starts_with("http") || icon.starts_with("https") {
        icon
    } else {
        format!("{}/{}", config.upload_url, icon)
    };

    site.icon = icon;

    Ok(Json(site))
}

#[get("/?<page>&<size>&<search>")]
pub async fn all(
    page: Option<i64>,
    size: Option<i64>,
    search: Option<&str>,
    config: &State<Config>,
    mut db: Connection<MySQLDb>,
) -> Result<Json<WithTotal<SiteWithCategory>>, Status> {
    let page = page.unwrap_or(0);

    let size = size.unwrap_or(10);

    let result = get_sites(page, size, search, &config.upload_url, &mut db)
        .await
        .map_err(|e| {
            error!("{}", e);

            e.status()
        })?;

    Ok(Json(result))
}

#[post("/", format = "json", data = "<site>")]
pub async fn add(
    site: Json<CreateSite<'_>>,
    config: &State<Config>,
    mut db: Connection<MySQLDb>,
    _jwt: Middleware,
) -> Result<(), Status> {
    let mut site = site.into_inner();

    let icon = standardize_url(site.icon, &config.upload_url);

    let icon = icon.unwrap_or_else(|| String::from(site.icon));

    site.icon = icon.as_str();

    site::add_site(&site, &mut db).await.map_err(|e| {
        error!("{}", e);

        e.status()
    })?;

    Ok(())
}

#[put("/<id>", format = "json", data = "<site>")]
pub async fn update<'r>(
    id: &'r str,
    site: Json<UpdateSite<'r>>,
    config: &State<Config>,
    mut db: Connection<MySQLDb>,
    _jwt: Middleware,
) -> Result<(), Status> {
    let mut site = site.into_inner();

    let icon = match site.icon {
        Some(icon) => standardize_url(icon, &config.upload_url),
        None => None,
    };

    site.icon = icon.as_deref();

    site::update_site(id, &site, &mut db).await.map_err(|e| {
        error!("{}", e);

        e.status()
    })?;

    Ok(())
}

#[delete("/<id>")]
pub async fn delete(id: &str, mut db: Connection<MySQLDb>, _jwt: Middleware) -> Result<(), Status> {
    site::delete_site(id, &mut db).await.map_err(|e| {
        error!("{}", e);

        e.status()
    })?;

    Ok(())
}

#[post("/<id>/visit")]
pub async fn analytics(id: &str, mut db: Connection<MySQLDb>) -> Result<(), Status> {
    site::analytics(id, &mut db).await.map_err(|e| {
        error!("{}", e);

        e.status()
    })?;

    Ok(())
}
