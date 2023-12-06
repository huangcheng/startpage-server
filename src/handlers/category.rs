use rocket_db_pools::Connection;
use sqlx::{query, query_as, Row};

use crate::errors::ServiceError;
use crate::models::category::Category;
use crate::request::category::{CreateCategory, UpdateCategory};
use crate::response;
use crate::response::WithTotal;
use crate::MySQLDb;

pub async fn get_categories(
    page: i64,
    size: i64,
    search: Option<&str>,
    upload_url: &str,
    db: &mut Connection<MySQLDb>,
) -> Result<WithTotal<response::category::Category>, ServiceError> {
    let total = match search {
        Some(search) => query(
            r#"SELECT COUNT(id) AS count FROM category WHERE name LIKE ? OR description LIKE ?"#,
        )
        .bind(format!("%{}%", search))
        .bind(format!("%{}%", search))
        .fetch_one(&mut ***db)
        .await?
        .get::<i64, &str>("count"),
        None => query(r#"SELECT COUNT(id) AS count FROM category"#)
            .fetch_one(&mut ***db)
            .await?
            .get::<i64, &str>("count"),
    };

    let categories = match search {
        Some(search) => query_as::<_, Category>(
            r#"SELECT id, name, description, icon, created_at, updated_at FROM category WHERE name LIKE ? OR description LIKE ? LIMIT ? OFFSET ?"#,
        )
        .bind(format!("%{}%", search))
        .bind(format!("%{}%", search))
        .bind(size)
        .bind(page * size)
        .fetch_all(&mut ***db)
        .await?,
        None => query_as::<_, Category>(
            r#"SELECT id, name, description, icon, created_at, updated_at FROM category LIMIT ? OFFSET ?"#,
        )
        .bind(size)
        .bind(page * size)
        .fetch_all(&mut ***db)
        .await?,
    };

    Ok(WithTotal {
        total,
        data: categories
            .iter()
            .map(|category| {
                let icon = category.icon.clone();

                let icon = if icon.starts_with("http") || icon.starts_with("https") {
                    icon
                } else {
                    format!("{}/{}", upload_url, icon)
                };

                response::category::Category {
                    id: category.id,
                    name: category.name.clone(),
                    description: category.description.clone(),
                    icon,
                }
            })
            .collect(),
    })
}

pub async fn update_category<'r>(
    id: &'r str,
    category: &'r UpdateCategory<'r>,
    db: &mut Connection<MySQLDb>,
) -> Result<(), ServiceError> {
    let record = query_as::<_, Category>(
        r#"SELECT id, name, description, icon, created_at, updated_at FROM category WHERE id = ?"#,
    )
    .bind(id)
    .fetch_one(&mut ***db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ServiceError::BadRequest(String::from("Category not found")),
        _ => ServiceError::DatabaseError(e),
    })?;

    let name = match category.name {
        Some(name) => match name.len() {
            0 => record.name,
            _ => String::from(name),
        },
        None => record.name,
    };

    let description = match category.description {
        Some(description) => match description.len() {
            0 => record.description,
            _ => String::from(description),
        },
        None => record.description,
    };

    let icon = match category.icon {
        Some(icon) => match icon.len() {
            0 => record.icon,
            _ => String::from(icon),
        },
        None => record.icon,
    };

    let record = Category {
        id: record.id,
        name,
        description,
        icon,
        created_at: record.created_at,
        updated_at: record.updated_at,
    };

    query(r#"UPDATE category SET name = ?, description = ?, icon = ? WHERE id = ?"#)
        .bind(&record.name)
        .bind(&record.description)
        .bind(&record.icon)
        .bind(record.id)
        .execute(&mut ***db)
        .await?;

    Ok(())
}

pub async fn add_category(
    category: &CreateCategory<'_>,
    db: &mut Connection<MySQLDb>,
) -> Result<(), ServiceError> {
    let id = query(r#"SELECT id FROM category WHERE name = ?"#)
        .bind(category.name)
        .fetch_optional(&mut ***db)
        .await?
        .map(|row| row.get::<i64, &str>("id"));

    if id.is_some() {
        return Err(ServiceError::AlreadyExists(String::from(
            "Category already exists",
        )));
    }

    query(r#"INSERT INTO category (name, description, icon) VALUES (?, ?, ?)"#)
        .bind(category.name)
        .bind(category.description)
        .bind(category.icon)
        .execute(&mut ***db)
        .await?;

    Ok(())
}

pub async fn delete_category(id: &str, db: &mut Connection<MySQLDb>) -> Result<(), ServiceError> {
    let sites_count =
        query(r#"SELECT COUNT(site_id) AS count FROM category_site WHERE category_id = ?"#)
            .bind(id)
            .fetch_one(&mut ***db)
            .await?
            .get::<i64, &str>("count");

    if sites_count > 0 {
        return Err(ServiceError::BadRequest(String::from("Category is in use")));
    }

    query(r#"DELETE FROM category WHERE id = ?"#)
        .bind(id)
        .execute(&mut ***db)
        .await?;

    Ok(())
}

pub async fn get_sites(
    category_id: &str,
    search: Option<&str>,
    upload_url: &str,
    db: &mut Connection<MySQLDb>,
) -> Result<Vec<response::site::Site>, ServiceError> {
    let sites = match search {
        Some(search) => query_as::<_, response::site::Site>(
            r#"SELECT site.id, site.name, site.url, site.description, site.icon FROM site INNER JOIN category_site ON site.id = category_site.site_id WHERE category_site.category_id = ? AND (site.name LIKE ? OR site.description LIKE ?)"#,
        )
        .bind(category_id)
        .bind(format!("%{}%", search))
        .bind(format!("%{}%", search))
        .fetch_all(&mut ***db)
        .await?,
        None => query_as::<_, response::site::Site>(
            r#"SELECT site.id, site.name, site.url, site.description, site.icon FROM site INNER JOIN category_site ON site.id = category_site.site_id WHERE category_site.category_id = ?"#,
        )
        .bind(category_id)
        .fetch_all(&mut ***db)
        .await?,
    };

    Ok(sites
        .iter()
        .map(|site| {
            let icon = site.icon.clone();

            let icon = if icon.starts_with("http") || icon.starts_with("https") {
                icon
            } else {
                format!("{}/{}", upload_url, icon)
            };

            response::site::Site {
                id: site.id,
                name: site.name.clone(),
                url: site.url.clone(),
                description: site.description.clone(),
                icon,
            }
        })
        .collect())
}
