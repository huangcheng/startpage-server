use std::ops::Deref;

use log::error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{delete, get, post, put};

use crate::handlers::category::{
    self, add_category, delete_category, get_all_categories, modify_site, update_category,
};
use crate::middlewares::JwtMiddleware;
use crate::request::category::{CreateCategory, UpdateCategory};
use crate::request::site::{CreateSite, UpdateSite};
use crate::response::category::Category;
use crate::response::site::Site;
use crate::state::AppState;

#[get("/")]
pub async fn all(state: &State<AppState>) -> Result<Json<Vec<Category>>, Status> {
    let categories = get_all_categories(state).await.map_err(|e| {
        error!("{}", e);

        e.into()
    })?;

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
        .map_err(|e| {
            error!("{}", e);

            e.into()
        })?;

    Ok(())
}

#[post("/", format = "json", data = "<category>")]
pub async fn add<'r>(
    category: Json<CreateCategory<'r>>,
    state: &State<AppState>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    add_category(category.deref(), state).await.map_err(|e| {
        error!("{}", e);

        e.into()
    })?;

    Ok(())
}

#[delete("/<id>")]
pub async fn delete<'r>(
    id: &'r str,
    state: &State<AppState>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    delete_category(id, state).await.map_err(|e| {
        error!("{}", e);

        e.into()
    })?;

    Ok(())
}

#[post("/<id>/site", format = "json", data = "<site>")]
pub async fn add_site(
    id: &str,
    site: Json<CreateSite<'_>>,
    state: &State<AppState>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    category::add_site(id, site.deref(), state)
        .await
        .map_err(|e| {
            error!("{}", e);

            e.into()
        })?;

    Ok(())
}

#[put("/<id>/site/<site_id>", format = "json", data = "<site>")]
pub async fn update_site<'r>(
    id: &'r str,
    site_id: &'r str,
    site: Json<UpdateSite<'r>>,
    state: &'r State<AppState>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    modify_site(id, site_id, site.deref(), state)
        .await
        .map_err(|e| {
            error!("{}", e);

            e.into()
        })?;

    Ok(())
}

#[delete("/<id>/site/<site_id>")]
pub async fn delete_site(
    id: &str,
    site_id: &str,
    state: &State<AppState>,
    _jwt: JwtMiddleware,
) -> Result<(), Status> {
    category::delete_site(id, site_id, state)
        .await
        .map_err(|e| {
            error!("{}", e);

            e.into()
        })?;

    Ok(())
}

#[get("/<id>/sites")]
pub async fn get_sites(id: &str, state: &State<AppState>) -> Result<Json<Vec<Site>>, Status> {
    let sites = category::get_sites(id, state).await.map_err(|e| {
        error!("{}", e);

        e.into()
    })?;

    Ok(Json(sites))
}
