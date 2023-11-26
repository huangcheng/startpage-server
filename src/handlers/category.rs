use rocket::State;

use crate::errors::ServiceError;
use crate::models::category::Category;
use crate::state::AppState;

pub async fn get_all_categories(state: &State<AppState>) -> Result<Vec<Category>, ServiceError> {
    let categories = sqlx::query_as::<_, Category>(r#"SELECT id, name, description FROM category"#)
        .fetch_all(&state.pool)
        .await?;

    Ok(categories)
}
