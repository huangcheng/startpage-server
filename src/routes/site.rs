use log::error;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::handlers::site::get_sites;
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
