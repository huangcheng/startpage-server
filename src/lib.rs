use rocket_db_pools::Database;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

#[derive(Database)]
#[database("startpage")]
pub struct Db(MySqlPool);

pub mod errors;
pub mod handlers;
pub mod middlewares;
pub mod models;
pub mod routes;
pub mod state;

pub mod config;
pub mod request;
pub mod response;
pub mod utils;
