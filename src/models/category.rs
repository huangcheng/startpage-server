use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: String,
}
