use rocket::http::Status;
use rocket::log::private::error;
use sqlx::query_as;
use bcrypt::verify;

use crate::response;
use crate::request;
use crate::models;
use crate::state::AppState;

pub async fn login(user: &request::auth::User<'_>, state: &AppState) -> Result<response::auth::User, Status> {
   let record = query_as::<_, models::user::User>(
       r#"SELECT username, nickname, password, avatar, email FROM user WHERE username = ?"#,
   )
       .bind(user.username)
       .fetch_one(&state.pool).await.map_err(|e| {
            error!("Failed to query user: {}", e);

            Status::InternalServerError
        })?;

    let valid = verify(user.password, &record.password).map_err(|e| {
        error!("Failed to verify password: {}", e);

        Status::InternalServerError
    })?;

    if valid {
        Ok(response::auth::User {
            username: record.username,
            nickname: record.nickname,
            email: record.email,
            avatar: record.avatar,
        })
    } else {
        Err(Status::Unauthorized)
    }
}
