use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub sort_order: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
