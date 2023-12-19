use rocket_db_pools::deadpool_redis;
use rocket_db_pools::Database;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /*
     * The subject of the token, in the `username:uuid` form.
     */
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

#[derive(Database)]
#[database("startpage")]
pub struct MySQLDb(MySqlPool);

#[derive(Database)]
#[database("cache")]
pub struct RedisDb(deadpool_redis::Pool);

pub mod errors;
pub mod guards;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod state;

pub mod config;
pub mod request;
pub mod response;
pub mod utils;
