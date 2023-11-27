use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CategorySite {
    pub category_id: i64,
    pub site_id: i64,
}
