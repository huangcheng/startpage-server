use rocket_db_pools::Connection;
use sqlx::{query, query_as, Row};

use crate::errors::ServiceError;
use crate::response::site::SiteWithCategory;
use crate::response::WithTotal;
use crate::Db;

pub async fn get_sites(
    page: i64,
    size: i64,
    db: &mut Connection<Db>,
) -> Result<WithTotal<SiteWithCategory>, ServiceError> {
    let count = query(r#"SELECT COUNT(id) AS count FROM site"#)
        .fetch_one(&mut ***db)
        .await?
        .get::<i64, &str>("count");

    let sites = query_as::<_, SiteWithCategory>(
        r#"SELECT site.id AS id, site.name AS name, site.url AS url, site.icon AS icon, site.description AS description, category.name AS category FROM site INNER JOIN category INNER JOIN category_site ON site.id = category_site.site_id AND category.id = category_site.category_id LIMIT ? OFFSET ?"#,
    ).bind(size).bind(page * size).fetch_all(&mut ***db).await?;

    Ok(WithTotal {
        total: count,
        data: sites,
    })
}
