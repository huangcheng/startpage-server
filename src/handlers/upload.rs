use std::fs;
use std::io;
use std::path::PathBuf;

use log::error;
use rocket::fs::TempFile;
use sha2::{Digest, Sha256};

use crate::errors::ServiceError;

pub async fn upload(file: &TempFile<'_>, path: &PathBuf) -> Result<String, ServiceError> {
    let content_type = match file.content_type() {
        Some(content_type) => content_type,
        None => {
            return Err(ServiceError::BadRequest(String::from(
                "Invalid content type",
            )))
        }
    };

    let tmp_file = match file.path() {
        Some(path) => path,
        None => return Err(ServiceError::BadRequest(String::from("Invalid file path"))),
    };

    let mut file = fs::File::open(tmp_file).map_err(|e| {
        error!("Failed to open file: {}", e);

        ServiceError::InternalServerError
    })?;

    let mut hasher = Sha256::new();

    io::copy(&mut file, &mut hasher).map_err(|e| {
        error!("Failed to copy file: {}", e);

        ServiceError::InternalServerError
    })?;

    let hash = hasher.finalize();
    let hash = format!("{:x}", hash);

    let extension = match content_type.extension() {
        Some(extension) => extension,
        None => return Err(ServiceError::BadRequest(String::from("Invalid extension"))),
    };

    let result = format!("{}.{}", hash, extension);

    let target = path.join(result.clone());

    if target.exists() {
        return Ok(result);
    }

    fs::copy(tmp_file, &target).map_err(|e| {
        error!("Failed to copy file: {}", e);

        ServiceError::InternalServerError
    })?;

    Ok(result)
}
