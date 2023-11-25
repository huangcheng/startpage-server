
use rocket::{self, routes};
use sqlx::MySqlPool;
use dotenvy::dotenv;

use startpage::state::AppState;

use startpage::routes::{user, auth};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().expect("Failed to read .env file");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");

    let pool = MySqlPool::connect(&database_url).await.expect("Failed to connect to MySQL");

    let state = AppState { pool };

    let _rok = rocket::build()
        .manage(state)
        .mount("/api/user", routes![user::update])
        .mount("/api/auth", routes![auth::login])
        .launch()
        .await?;


    Ok(())
}
