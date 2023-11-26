use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub avatar: Option<String>,
}
