use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Site {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub description: String,
    pub icon: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
