use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}


pub mod models;
pub mod routes;
pub mod handlers;
pub mod state;
pub mod errors;
pub mod middlewares;

pub mod request;
pub mod response;
pub mod utils;
