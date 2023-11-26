use sqlx::MySqlPool;

pub struct AppState {
    pub pool: MySqlPool,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
}
