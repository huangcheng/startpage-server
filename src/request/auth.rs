use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct User<'r> {
    pub username: &'r str,
    pub password: &'r str,
}

