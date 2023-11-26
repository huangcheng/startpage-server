use rocket::State;
use sqlx::{query, query_as};

use crate::errors::ServiceError;
use crate::models::category::Category;
use crate::request::category::{CreateCategory, UpdateCategory};
use crate::state::AppState;

pub async fn get_all_categories(state: &State<AppState>) -> Result<Vec<Category>, ServiceError> {
    let categories = sqlx::query_as::<_, Category>(r#"SELECT id, name, description FROM category"#)
        .fetch_all(&state.pool)
        .await?;

    Ok(categories)
}

pub async fn update_category<'r>(
    id: &'r str,
    category: &'r UpdateCategory<'r>,
    state: &State<AppState>,
) -> Result<Category, ServiceError> {
    let record =
        query_as::<_, Category>(r#"SELECT id, name, description FROM category WHERE id = ?"#)
            .bind(id)
            .fetch_one(&state.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ServiceError::NotFound,
                _ => ServiceError::DatabaseError(e.into()),
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
    };

    query(r#"UPDATE category SET name = ?, description = ? WHERE id = ?"#)
        .bind(&record.name)
        .bind(&record.description)
        .bind(&record.id)
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
