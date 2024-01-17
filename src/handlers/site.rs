use rocket_db_pools::Connection;
use sqlx::{query, query_as, Row};

use crate::errors::ServiceError;
use crate::models::category::Category;
use crate::models::site::Site;
use crate::request::site::{CreateSite, UpdateSite};
use crate::response::site::Site as SiteResponse;
use crate::response::site::SiteWithCategory;
use crate::response::WithTotal;
use crate::MySQLDb;

pub async fn get_sites(
    page: i64,
    size: i64,
    search: Option<&str>,
    upload_url: &str,
    db: &mut Connection<MySQLDb>,
) -> Result<WithTotal<SiteWithCategory>, ServiceError> {
    let count = match search {
        Some(search) => {
            query(r#"SELECT COUNT(id) AS count FROM site WHERE NAME LIKE ? OR description LIKE ?"#)
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
            r#"SELECT site.id AS id, site.name AS name, site.url AS url, site.icon AS icon, site.description AS description, site.visit_count as visit_count, category.name AS category FROM site
                INNER JOIN category
                INNER JOIN category_site ON site.id = category_site.site_id AND category.id = category_site.category_id WHERE site.name LIKE ? OR site.description LIKE ? LIMIT ? OFFSET ?
                "#,
        )
        .bind(format!("%{}%", search))
        .bind(format!("%{}%", search))
        .bind(size)
        .bind(page * size)
        .fetch_all(&mut ***db)
        .await?,
        None => query_as::<_, SiteWithCategory>(
            r#"SELECT site.id AS id, site.name AS name, site.url AS url, site.icon AS icon, site.description AS description, site.visit_count as visit_count, category.name AS category FROM site
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
                    visit_count: site.visit_count,
                }
            })
            .collect(),
    })
}

pub async fn add_site(
    site: &CreateSite<'_>,
    db: &mut Connection<MySQLDb>,
) -> Result<(), ServiceError> {
    query_as::<_, Category>(
        r#"SELECT id, name, description, icon, sort_order, parent_id, created_at, updated_at FROM category WHERE id = ?"#,
    )
    .bind(site.category)
    .fetch_one(&mut ***db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ServiceError::BadRequest(String::from("Category not found")),
        _ => ServiceError::DatabaseError(e),
    })?;

    let order = match query(r#"SELECT MAX(sort_order) AS sort_order FROM site INNER JOIN category_site ON site.id = category_site.site_id WHERE category_site.category_id = ?"#)
         .bind(site.category)
        .fetch_one(&mut ***db)
        .await
    {
        Ok(row) => match row.try_get::<i64, &str>("sort_order") {
            Ok(order) => order + 1,
            Err(_) => 0,
        },
        Err(_) => 0,
    };

    let id = query(
        r#"INSERT INTO site (name, url, description, icon, sort_order) VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(site.name)
    .bind(site.url)
    .bind(site.description)
    .bind(site.icon)
    .bind(order)
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
    db: &mut Connection<MySQLDb>,
) -> Result<(), ServiceError> {
    let record = query_as::<_, Site>(
        r#"SELECT id, name, url, description, icon, sort_order, visit_count, created_at, updated_at FROM site WHERE id = ?"#,
    )
    .bind(site_id)
    .fetch_one(&mut ***db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ServiceError::BadRequest(String::from("Site not found")),
        _ => ServiceError::DatabaseError(e),
    })?;

    let name = match site.name {
        Some(name) => match name.len() {
            0 => record.name,
            _ => String::from(name),
        },
        None => record.name,
    };

    let url = match site.url {
        Some(url) => match url.len() {
            0 => record.url,
            _ => String::from(url),
        },
        None => record.url,
    };

    let description = match site.description {
        Some(description) => match description.len() {
            0 => record.description,
            _ => String::from(description),
        },
        None => record.description,
    };

    let icon = match site.icon {
        Some(icon) => match icon.len() {
            0 => record.icon,
            _ => String::from(icon),
        },
        None => record.icon,
    };

    let record = Site {
        id: record.id,
        name,
        url,
        description,
        icon,
        sort_order: record.sort_order,
        visit_count: record.visit_count,
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

        let order = match query(r#"SELECT MAX(sort_order) AS sort_order FROM site INNER JOIN category_site ON site.id = category_site.site_id WHERE category_site.category_id = ?"#).bind(category).fetch_one(&mut ***db).await
        {
            Ok(row) => match row.try_get::<i64, &str>("sort_order") {
                Ok(order) => order + 1,
                Err(_) => 0,
            },
            Err(_) => 0,
        };

        query(r#"UPDATE site SET sort_order = ? WHERE id = ?"#)
            .bind(order)
            .bind(record.id)
            .execute(&mut ***db)
            .await?;

        query(r#"UPDATE category_site SET category_id = ? WHERE site_id = ?"#)
            .bind(category)
            .bind(record.id)
            .execute(&mut ***db)
            .await?;
    }

    Ok(())
}

pub async fn delete_site(id: &str, db: &mut Connection<MySQLDb>) -> Result<(), ServiceError> {
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

pub async fn analytics(id: &str, db: &mut Connection<MySQLDb>) -> Result<(), ServiceError> {
    query(r#"UPDATE site SET visit_count = visit_count + 1 WHERE id = ?"#)
        .bind(id)
        .execute(&mut ***db)
        .await?;

    Ok(())
}

pub async fn get_site(id: i64, db: &mut Connection<MySQLDb>) -> Result<SiteResponse, ServiceError> {
    let record = query_as::<_, Site>(
        r#"SELECT id, name, url, description, icon, sort_order, visit_count, created_at, updated_at FROM site WHERE id = ?"#,
    )
    .bind(id)
    .fetch_one(&mut ***db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ServiceError::BadRequest(String::from("Site not found")),
        _ => ServiceError::DatabaseError(e),
    })?;

    Ok(record.into())
}
