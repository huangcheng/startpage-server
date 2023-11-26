use crate::errors::ServiceError;
use crate::models;
use crate::response;
use crate::state::AppState;
use rocket::State;
use sqlx::query_as;

pub async fn get_user_by_username(
    username: &str,
    state: &State<AppState>,
) -> Result<response::user::User, ServiceError> {
    let user = query_as::<_, models::user::User>("SELECT * FROM user WHERE username = ?")
        .bind(username)
        .fetch_one(&state.pool)
        .await?;

    Ok(response::user::User {
        username: user.username,
        nickname: user.nickname,
        avatar: user.avatar,
        email: user.email,
    })
}
