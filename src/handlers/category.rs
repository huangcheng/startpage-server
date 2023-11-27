use rocket::State;
use sqlx::{query, query_as, Row};

use crate::errors::ServiceError;
use crate::models::category::Category;
use crate::models::site::Site;
use crate::request::category::{CreateCategory, UpdateCategory};
use crate::request::site::{CreateSite, UpdateSite};
use crate::response;
use crate::state::AppState;

pub async fn get_all_categories(
    state: &State<AppState>,
) -> Result<Vec<response::category::Category>, ServiceError> {
    let categories = sqlx::query_as::<_, Category>(
        r#"SELECT id, name, description, created_at, updated_at FROM category"#,
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(categories
        .into_iter()
        .map(|category| category.into())
        .collect())
}

pub async fn update_category<'r>(
    id: &'r str,
    category: &'r UpdateCategory<'r>,
    state: &State<AppState>,
) -> Result<Category, ServiceError> {
    let record = query_as::<_, Category>(
        r#"SELECT id, name, description, created_at, updated_at FROM category WHERE id = ?"#,
    )
    .bind(id)
    .fetch_one(&state.pool)
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
        .execute(&state.pool)
        .await?;

    Ok(record)
}

pub async fn add_category(
    category: &CreateCategory<'_>,
    state: &State<AppState>,
) -> Result<(), ServiceError> {
    query(r#"INSERT INTO category (name, description) VALUES (?, ?)"#)
        .bind(category.name)
        .bind(category.description)
        .execute(&state.pool)
        .await?;

    Ok(())
}

pub async fn delete_category(id: &str, state: &State<AppState>) -> Result<(), ServiceError> {
    let sites_count =
        query(r#"SELECT COUNT(site_id) AS count FROM category_site WHERE category_id = ?"#)
            .bind(id)
            .fetch_one(&state.pool)
            .await?
            .get::<i64, &str>("count");

    if sites_count > 0 {
        return Err(ServiceError::BadRequest(String::from("Category is in use")));
    }

    query(r#"DELETE FROM category WHERE id = ?"#)
        .bind(id)
        .execute(&state.pool)
        .await?;

    Ok(())
}

pub async fn add_site(
    category_id: &str,
    site: &CreateSite<'_>,
    state: &State<AppState>,
) -> Result<(), ServiceError> {
    query_as::<_, Category>(
        r#"SELECT id, name, description, created_at, updated_at FROM category WHERE id = ?"#,
    )
    .bind(category_id)
    .fetch_one(&state.pool)
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
        .execute(&state.pool)
        .await?
        .last_insert_id();

    query(r#"INSERT INTO category_site (category_id, site_id) VALUES (?, ?)"#)
        .bind(category_id)
        .bind(id)
        .execute(&state.pool)
        .await?;

    Ok(())
}

pub async fn get_sites(
    category_id: &str,
    state: &State<AppState>,
) -> Result<Vec<response::site::Site>, ServiceError> {
    let sites = query_as::<_, Site>(
        r#"SELECT id, name, url, description, icon, created_at, updated_at FROM site WHERE id IN (SELECT site_id FROM category_site WHERE category_id = ?)"#,
    )
    .bind(category_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(sites.into_iter().map(|site| site.into()).collect())
}

pub async fn modify_site(
    category_id: &str,
    site_id: &str,
    site: &UpdateSite<'_>,
    state: &State<AppState>,
) -> Result<(), ServiceError> {
    let record = query_as::<_, Site>(
        r#"SELECT id, name, url, description, icon, created_at, updated_at FROM site WHERE id = ?"#,
    )
    .bind(site_id)
    .fetch_one(&state.pool)
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
        .execute(&state.pool)
        .await?;

    let category_id = match site.category_id {
        Some(category_id) => format!("{}", category_id),
        None => category_id.to_string(),
    };

    query(r#"SELECT id FROM category WHERE id = ?"#)
        .bind(&category_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                ServiceError::BadRequest(String::from("Category not found"))
            }
            _ => ServiceError::DatabaseError(e),
        })?;

    query(r#"UPDATE category_site SET category_id = ? WHERE site_id = ?"#)
        .bind(&category_id)
        .bind(record.id)
        .execute(&state.pool)
        .await?;

    Ok(())
}
