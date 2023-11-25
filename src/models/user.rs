use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub email: String,
    pub avatar: Option<String>,
}
