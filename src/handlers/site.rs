use rocket_db_pools::Connection;
use sqlx::{query, query_as, Row};

use crate::errors::ServiceError;
use crate::models::category::Category;
use crate::models::site::Site;
use crate::request::site::{CreateSite, UpdateSite};
use crate::response::site::SiteWithCategory;
use crate::response::WithTotal;
use crate::Db;

pub async fn get_sites(
    page: i64,
    size: i64,
    search: Option<&str>,
    upload_url: &str,
    db: &mut Connection<Db>,
) -> Result<WithTotal<SiteWithCategory>, ServiceError> {
    let count = match search {
        Some(search) => {
            query(r#"SELECT COUNT(id) AS count FROM site WHERE NAME LIKE % OR description LIKE %"#)
                .bind(format!("%{}%", search))
                .bind(format!("%{}%", search))
                .fetch_one(&mut ***db)
                .await?
                .get::<i64, &str>("count")
        }
        None => query(r#"SELECT COUNT(id) AS count FROM site"#)
            .fetch_one(&mut ***db)
            .await?
            .get::<i64, &str>("count"),
    };

    let sites = match search {
        Some(search) => query_as::<_, SiteWithCategory>(
            r#"SELECT site.id AS id, site.name AS name, site.url AS url, site.icon AS icon, site.description AS description, category.name AS category FROM site
                INNER JOIN category
                INNER JOIN category_site ON site.id = category_site.site_id AND category.id = category_site.category_id WHERE site.name LIKE % OR site.description LIKE % LIMIT ? OFFSET ?
                "#,
        )
        .bind(format!("%{}%", search))
        .bind(format!("%{}%", search))
        .bind(size)
        .bind(page * size)
        .fetch_all(&mut ***db)
        .await?,
        None => query_as::<_, SiteWithCategory>(
            r#"SELECT site.id AS id, site.name AS name, site.url AS url, site.icon AS icon, site.description AS description, category.name AS category FROM site
                INNER JOIN category
                INNER JOIN category_site ON site.id = category_site.site_id AND category.id = category_site.category_id LIMIT ? OFFSET ?
                "#,
        )
        .bind(size)
        .bind(page * size)
        .fetch_all(&mut ***db)
        .await?,
    };

    Ok(WithTotal {
        total: count,
        data: sites
            .iter()
            .map(|site| {
                let icon = site.icon.clone();

                let icon = if icon.starts_with("http") || icon.starts_with("https") {
                    icon
                } else {
                    format!("{}/{}", upload_url, icon)
                };

                SiteWithCategory {
                    id: site.id,
                    name: site.name.clone(),
                    url: site.url.clone(),
                    icon,
                    description: site.description.clone(),
                    category: site.category.clone(),
                }
            })
            .collect(),
    })
}

pub async fn add_site(site: &CreateSite<'_>, db: &mut Connection<Db>) -> Result<(), ServiceError> {
    query_as::<_, Category>(
        r#"SELECT id, name, description, icon, created_at, updated_at FROM category WHERE id = ?"#,
    )
    .bind(site.category)
    .fetch_one(&mut ***db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ServiceError::BadRequest(String::from("Category not found")),
        _ => ServiceError::DatabaseError(e),
    })?;

    let id = query(r#"INSERT INTO site (name, url, description, icon) VALUES (?, ?, ?, ?)"#)
        .bind(site.name)
        .bind(site.url)
        .bind(site.description)
        .bind(site.icon)
        .execute(&mut ***db)
        .await?
        .last_insert_id();

    query(r#"INSERT INTO category_site (category_id, site_id) VALUES (?, ?)"#)
        .bind(site.category)
        .bind(id)
        .execute(&mut ***db)
        .await?;

    Ok(())
}

pub async fn update_site(
    site_id: &str,
    site: &UpdateSite<'_>,
    db: &mut Connection<Db>,
) -> Result<(), ServiceError> {
    let record = query_as::<_, Site>(
        r#"SELECT id, name, url, description, icon, created_at, updated_at FROM site WHERE id = ?"#,
    )
    .bind(site_id)
    .fetch_one(&mut ***db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ServiceError::BadRequest(String::from("Site not found")),
        _ => ServiceError::DatabaseError(e),
    })?;

    let name = match site.name {
        Some(name) => String::from(name),
        None => record.name,
    };

    let url = match site.url {
        Some(url) => String::from(url),
        None => record.url,
    };

    let description = match site.description {
        Some(description) => String::from(description),
        None => record.description,
    };

    let icon = match site.icon {
        Some(icon) => String::from(icon),
        None => record.icon,
    };

    let record = Site {
        id: record.id,
        name,
        url,
        description,
        icon,
        created_at: record.created_at,
        updated_at: record.updated_at,
    };

    query(r#"UPDATE site SET name = ?, url = ?, description = ?, icon = ? WHERE id = ?"#)
        .bind(&record.name)
        .bind(&record.url)
        .bind(&record.description)
        .bind(&record.icon)
        .bind(record.id)
        .execute(&mut ***db)
        .await?;

    if let Some(category) = site.category {
        query(r#"SELECT id FROM category WHERE id = ?"#)
            .bind(category)
            .fetch_one(&mut ***db)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    ServiceError::BadRequest(String::from("Category not found"))
                }
                _ => ServiceError::DatabaseError(e),
            })?;

        query(r#"UPDATE category_site SET category_id = ? WHERE site_id = ?"#)
            .bind(category)
            .bind(record.id)
            .execute(&mut ***db)
            .await?;
    }

    Ok(())
}

pub async fn delete_site(id: &str, db: &mut Connection<Db>) -> Result<(), ServiceError> {
    query(r#"DELETE FROM category_site WHERE site_id = ?"#)
        .bind(id)
        .execute(&mut ***db)
        .await?;

    query(r#"DELETE FROM site WHERE id = ?"#)
        .bind(id)
        .execute(&mut ***db)
        .await?;

    Ok(())
}
