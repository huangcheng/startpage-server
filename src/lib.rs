use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

pub mod errors;
pub mod handlers;
pub mod middlewares;
pub mod models;
pub mod routes;
pub mod state;

pub mod request;
pub mod response;
pub mod utils;
