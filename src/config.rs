use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt {
    pub secret: String,
    pub expires_in: String,
}

impl Default for Jwt {
    fn default() -> Self {
        Self {
            secret: String::from("StartPage"),
            expires_in: String::from("1h"),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub jwt: Jwt,
    pub upload_dir: PathBuf,
    pub upload_url: String,
}
