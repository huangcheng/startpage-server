use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::handlers::category::get_all_categories;
use crate::models::category::Category;
use crate::state::AppState;

#[get("/")]
pub async fn all(state: &State<AppState>) -> Result<Json<Vec<Category>>, Status> {
    let categories = get_all_categories(state).await.map_err(|e| e.status())?;

    Ok(Json(categories))
}
