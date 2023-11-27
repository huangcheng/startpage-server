use log::error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, State};

use crate::handlers::user::get_user_by_username;
use crate::middlewares::JwtMiddleware;
use crate::response::user::User;
use crate::state::AppState;

#[get("/")]
pub async fn me(jwt: JwtMiddleware, state: &State<AppState>) -> Result<Json<User>, Status> {
    let username = jwt.username;

    let user = get_user_by_username(&username, state).await.map_err(|e| {
        error!("{}", e);

        e.into()
    })?;

    Ok(Json(user))
}
