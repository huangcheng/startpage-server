use log::error;
use rocket_db_pools::Connection;
use sqlx::{query, query_as, Row};
use std::collections::HashMap;

use crate::errors::ServiceError;
use crate::models::category::Category;
use crate::request::category::{CreateCategory, UpdateCategory};
use crate::response;
use crate::response::WithTotal;
use crate::MySQLDb;

fn build_tree(arr: &Vec<Category>, upload_url: &str) -> Vec<response::category::Category> {
    let mut map: HashMap<Option<i64>, Vec<response::category::Category>> = HashMap::new();
    let mut categories = arr
        .clone()
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
                children: None,
            }
        })
        .collect::<Vec<response::category::Category>>();

    for category in arr {
        map.entry(category.parent_id)
            .or_default()
            .push(response::category::Category::from(category.clone()));
    }

    for category in &mut categories {
        if let Some(children) = map.get(&Some(category.id)) {
            category.children = Some(children.to_vec());
        }
    }

    let ids_to_be_deleted = arr
        .iter()
        .filter(|x| x.parent_id.is_some())
        .map(|x| x.id)
        .collect::<Vec<i64>>();

    for id in ids_to_be_deleted {
        categories.remove(categories.iter().position(|x| x.id == id).unwrap());
    }

    categories
}
pub async fn get_categories(
    page: i64,
    size: i64,
    search: Option<&str>,
    upload_url: &str,
    db: &mut Connection<MySQLDb>,
) -> Result<WithTotal<response::category::Category>, ServiceError> {
    let total = match search {
        Some(search) => query(
            r#"
              WITH RECURSIVE category_hierarchy AS (
                SELECT
                    id,
                    name,
                    description,
                    icon,
                    parent_id
                FROM
                    category
                WHERE
                    parent_id IS NULL
                UNION ALL
                SELECT
                    c.id,
                    c.name,
                    c.description,
                    c.icon,
                    c.parent_id
                FROM
                    category c
                INNER JOIN
                    category_hierarchy ch ON ch.id = c.parent_id
            )
            SELECT
                COUNT(id) AS count
            FROM
                category_hierarchy AS ch WHERE ch.name LIKE ? OR ch.description LIKE ?;
                "#,
        )
        .bind(format!("%{}%", search))
        .bind(format!("%{}%", search))
        .fetch_one(&mut ***db)
        .await?
        .get::<i64, &str>("count"),
        None => query(
            r#"
            WITH RECURSIVE category_hierarchy AS (
                SELECT
                    id,
                    name,
                    description,
                    icon,
                    parent_id
                FROM
                    category
                WHERE
                    parent_id IS NULL
                UNION ALL
                SELECT
                    c.id,
                    c.name,
                    c.description,
                    c.icon,
                    c.parent_id
                FROM
                    category c
                INNER JOIN
                    category_hierarchy ch ON ch.id = c.parent_id
            )
            SELECT
                COUNT(id) AS count
            FROM
                category_hierarchy
        "#,
        )
        .fetch_one(&mut ***db)
        .await?
        .get::<i64, &str>("count"),
    };

    let categories = match search {
        Some(search) => query_as::<_, Category>(
            r#"
            WITH RECURSIVE category_hierarchy AS (
                SELECT
                    id,
                    name,
                    description,
                    icon,
                    parent_id,
                    sort_order,
                    parent_id,
                    created_at,
                    updated_at
                FROM
                    category
                WHERE
                    parent_id IS NULL
                UNION ALL
                SELECT
                    c.id,
                    c.name,
                    c.description,
                    c.icon,
                    c.parent_id,
                    c.sort_order
                    c.created_at,
                    c.updated_at
                FROM
                    category c
                INNER JOIN
                    category_hierarchy ch ON ch.id = c.parent_id
            )
            SELECT
               id, name, description, icon, sort_order, parent_id, created_at, updated_at
            FROM
                category_hierarchy AS ch WHERE ch.name LIKE ? OR ch.description LIKE ? ORDER BY sort_order LIMIT ? OFFSET ?;
            "#,
        )
        .bind(format!("%{}%", search))
        .bind(format!("%{}%", search))
        .bind(size)
        .bind(page * size)
        .fetch_all(&mut ***db)
        .await?,
        None => query_as::<_, Category>(
            r#"
           WITH RECURSIVE category_hierarchy AS (
                SELECT
                    id,
                    name,
                    description,
                    icon,
                    parent_id,
                    sort_order,
                    created_at,
                    updated_at
                FROM
                    category
                WHERE
                    parent_id IS NULL
                UNION ALL
                SELECT
                    c.id,
                    c.name,
                    c.description,
                    c.icon,
                    c.parent_id,
                    c.sort_order,
                    c.created_at,
                    c.updated_at
                FROM
                    category c
                INNER JOIN
                    category_hierarchy ch ON ch.id = c.parent_id
            )
            SELECT
               id, name, description, icon, sort_order, parent_id, created_at, updated_at
            FROM
                category_hierarchy AS ch ORDER BY sort_order LIMIT ? OFFSET ?;
            "#,
        )
        .bind(size)
        .bind(page * size)
        .fetch_all(&mut ***db)
        .await?,
    };

    Ok(WithTotal {
        total,
        data: build_tree(&categories, upload_url),
    })
}

pub async fn update_category<'r>(
    id: &'r str,
    category: &'r UpdateCategory<'r>,
    db: &mut Connection<MySQLDb>,
) -> Result<(), ServiceError> {
    let record = query_as::<_, Category>(
        r#"SELECT id, name, description, icon, sort_order, created_at, updated_at FROM category WHERE id = ?"#,
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

    let parent_id = match category.parent_id {
        Some(parent_id) => Some(parent_id),
        None => record.parent_id,
    };

    let record = Category {
        id: record.id,
        name,
        description,
        icon,
        parent_id,
        sort_order: record.sort_order,
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

    let order = match category.parent_id {
        Some(parent_id) => match query( r#"SELECT MAX(c1.sort_order) AS sort_order FROM category AS c1 INNER JOIN category AS c2 ON c1.parent_id = c2.id WHERE c1.parent_id = ?"#).bind(parent_id).fetch_one(&mut ***db).await {
            Ok(row) => match row.try_get::<i64, &str>("sort_order") {
                Ok(order) => order + 1,
                Err(_) => 0,
            },
            Err(_) => 0,
        },
        None => match query(r#"SELECT MAX(sort_order) AS sort_order FROM category"#)
            .fetch_one(&mut ***db)
            .await
        {
            Ok(row) => match row.try_get::<i64, &str>("sort_order") {
                Ok(order) => order + 1,
                Err(_) => 0,
            },
            Err(_) => 0,
        },
    };

    query(r#"INSERT INTO category (name, description, icon, sort_order, parent_id) VALUES (?, ?, ?, ?, ?)"#)
        .bind(category.name)
        .bind(category.description)
        .bind(category.icon)
        .bind(order)
        .bind(category.parent_id)
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
            r#"SELECT site.id, site.name, site.url, site.description, site.icon, site.visit_count FROM site INNER JOIN category_site ON site.id = category_site.site_id WHERE category_site.category_id = ? AND (site.name LIKE ? OR site.description LIKE ?) ORDER BY site.sort_order"#,
        )
        .bind(category_id)
        .bind(format!("%{}%", search))
        .bind(format!("%{}%", search))
        .fetch_all(&mut ***db)
        .await?,
        None => query_as::<_, response::site::Site>(
            r#"SELECT site.id, site.name, site.url, site.description, site.icon, site.visit_count FROM site INNER JOIN category_site ON site.id = category_site.site_id WHERE category_site.category_id = ? ORDER BY site.sort_order"#,
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
                visit_count: site.visit_count,
            }
        })
        .collect())
}

pub async fn sort_categories(
    active_id: i64,
    over_id: Option<i64>,
    parent_id: Option<i64>,
    db: &mut Connection<MySQLDb>,
) -> Result<(), ServiceError> {
    let ids = match parent_id {
        Some(parent_id) => {
            query(r#"SELECT c1.id AS id FROM category AS c1 INNER JOIN category AS c2 ON c1.parent_id = c2.id WHERE c1.parent_id = ? ORDER BY c1.sort_order"#)
                .bind(parent_id)
                .fetch_all(&mut ***db)
                .await?
        }
        None => {
            query(r#"SELECT id FROM category ORDER BY sort_order"#)
                .fetch_all(&mut ***db)
                .await?
        }
    };

    let ids = ids
        .into_iter()
        .map(|row| row.get::<i64, &str>("id"))
        .collect::<Vec<i64>>();

    let old_index = ids.iter().position(|id| *id == active_id).unwrap();
    let new_index = match over_id {
        Some(over_id) => ids.iter().position(|id| *id == over_id).unwrap(),
        None => 0,
    };

    let mut ids = ids.clone();
    ids.remove(old_index);
    ids.insert(new_index, active_id);

    for (index, id) in ids.iter().enumerate() {
        query(r#"UPDATE category SET sort_order = ? WHERE id = ?"#)
            .bind(index as i64)
            .bind(id)
            .execute(&mut ***db)
            .await?;
    }

    Ok(())
}

pub async fn sort_category_sites(
    id: i64,
    active_id: i64,
    over_id: Option<i64>,
    db: &mut Connection<MySQLDb>,
) -> Result<(), ServiceError> {
    query(r#"
                SELECT site.id as site_id
                FROM site
                INNER JOIN category
                INNER JOIN category_site ON site.id = category_site.site_id AND category.id = category_site.category_id
                WHERE site.id = ? AND category.id = ?;
    "#)
        .bind(active_id)
        .bind(id)
        .fetch_one(&mut ***db)
        .await
        .map_err(|e| {
            error!("{}", e);

            ServiceError::BadRequest(String::from("Site not found"))
        })?;

    let ids = query(
        r#"
                SELECT site.id as site_id
                FROM site
                INNER JOIN category
                INNER JOIN category_site ON site.id = category_site.site_id AND category.id = category_site.category_id
                WHERE category.id = ? ORDER BY site.sort_order;"#,
    ).bind(id)
        .fetch_all(&mut ***db)
        .await?;

    let ids = ids
        .into_iter()
        .map(|row| row.get::<i64, &str>("site_id"))
        .collect::<Vec<i64>>();

    let old_index = ids.iter().position(|id| *id == active_id).unwrap();
    let new_index = match over_id {
        Some(over_id) => ids.iter().position(|id| *id == over_id).unwrap(),
        None => 0,
    };

    let mut ids = ids.clone();

    ids.remove(old_index);
    ids.insert(new_index, active_id);

    for (index, id) in ids.iter().enumerate() {
        query(r#"UPDATE site SET sort_order = ? WHERE id = ?"#)
            .bind(index as i64)
            .bind(id)
            .execute(&mut ***db)
            .await?;
    }

    Ok(())
}
