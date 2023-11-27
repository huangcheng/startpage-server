use derive_more::Display;
use rocket::http::Status;

#[allow(dead_code)]
#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Database error: {}", _0)]
    DatabaseError(sqlx::Error),

    #[display(fmt = "Format error: {}", _0)]
    FormatError(String),

    #[display(fmt = "Not found")]
    NotFound,

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Internal server error")]
    InternalServerError,

    #[display(fmt = "Bad request: {}", _0)]
    BadRequest(String),
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

impl Into<Status> for ServiceError {
    fn into(self) -> Status {
        self.status()
    }
}

impl ServiceError {
    pub fn status(&self) -> Status {
        match self {
            ServiceError::DatabaseError(_) => Status::InternalServerError,
            ServiceError::FormatError(_) => Status::BadRequest,
            ServiceError::NotFound => Status::NotFound,
            ServiceError::Unauthorized => Status::Unauthorized,
            ServiceError::InternalServerError => Status::InternalServerError,
            ServiceError::BadRequest(_) => Status::BadRequest,
        }
    }
}
