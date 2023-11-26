use derive_more::Display;

#[allow(dead_code)]
#[derive(Debug, Display)]
pub enum ServiceError {
    DatabaseError(sqlx::Error),

    FormatError(String),

    NotFound,

    Unauthorized,

    InternalServerError,
}

impl From<sqlx::Error> for ServiceError {
    fn from(error: sqlx::Error) -> Self {
        ServiceError::DatabaseError(error)
    }
}

impl From<regex::Error> for ServiceError {
    fn from(error: regex::Error) -> Self {
        ServiceError::FormatError(error.to_string())
    }
}

impl From<std::num::ParseIntError> for ServiceError {
    fn from(error: std::num::ParseIntError) -> Self {
        ServiceError::FormatError(error.to_string())
    }
}
