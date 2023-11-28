use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Site {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SiteWithCategory {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub description: String,
    pub icon: String,
    pub category: String,
}
