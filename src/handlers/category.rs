use rocket_db_pools::Connection;
use sqlx::{query, query_as, Row};

use crate::errors::ServiceError;
use crate::models::category::Category;
use crate::models::site::Site;
use crate::request::category::{CreateCategory, UpdateCategory};
use crate::request::site::{CreateSite, UpdateSite};
use crate::response;
use crate::response::WithTotal;
use crate::Db;

pub async fn get_categories(
    page: i64,
    size: i64,
    db: &mut Connection<Db>,
) -> Result<WithTotal<response::category::Category>, ServiceError> {
    let total = query(r#"SELECT COUNT(id) AS count FROM category"#)
        .fetch_one(&mut ***db)
        .await?
        .get::<i64, &str>("count");

    let categories = sqlx::query_as::<_, Category>(
        r#"SELECT id, name, description, created_at, updated_at FROM category LIMIT ? OFFSET ?"#,
    )
    .bind(size)
    .bind(page * size)
    .fetch_all(&mut ***db)
    .await?;

    Ok(WithTotal {
        total,
        data: categories
            .into_iter()
            .map(|category| category.into())
            .collect(),
    })
}

pub async fn update_category<'r>(
    id: &'r str,
    category: &'r UpdateCategory<'r>,
    db: &mut Connection<Db>,
) -> Result<Category, ServiceError> {
    let record = query_as::<_, Category>(
        r#"SELECT id, name, description, created_at, updated_at FROM category WHERE id = ?"#,
    )
    .bind(id)
    .fetch_one(&mut ***db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ServiceError::BadRequest(String::from("Category not found")),
        _ => ServiceError::DatabaseError(e),
    })?;

    let name = match category.name {
        Some(name) => String::from(name),
        None => record.name,
    };

    let description = match category.description {
        Some(description) => String::from(description),
        None => record.description,
    };

    let record = Category {
        id: record.id,
        name,
        description,
        created_at: record.created_at,
        updated_at: record.updated_at,
    };

    query(r#"UPDATE category SET name = ?, description = ? WHERE id = ?"#)
        .bind(&record.name)
        .bind(&record.description)
        .bind(record.id)
        .execute(&mut ***db)
        .await?;

    Ok(record)
}

pub async fn add_category(
    category: &CreateCategory<'_>,
    db: &mut Connection<Db>,
) -> Result<(), ServiceError> {
    query(r#"INSERT INTO category (name, description) VALUES (?, ?)"#)
        .bind(category.name)
        .bind(category.description)
        .execute(&mut ***db)
        .await?;

    Ok(())
}

pub async fn delete_category(id: &str, db: &mut Connection<Db>) -> Result<(), ServiceError> {
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
    db: &mut Connection<Db>,
) -> Result<Vec<response::site::Site>, ServiceError> {
    let sites = query_as::<_, response::site::Site>(
        r#"SELECT id, name, url, description, icon FROM site WHERE id IN (SELECT site_id FROM category_site WHERE category_id = ?)"#,
    )
    .bind(category_id)
    .fetch_all(&mut ***db)
    .await?;

    Ok(sites)
}
