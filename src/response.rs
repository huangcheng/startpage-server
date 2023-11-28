use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WithTotal<T> {
    pub total: i64,
    pub data: Vec<T>,
}

pub mod auth;
pub mod category;
pub mod site;
pub mod user;
