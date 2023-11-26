use dotenvy::dotenv;
use rocket::{self, routes};
use sqlx::MySqlPool;

use startpage::routes::{auth, user};
use startpage::state::AppState;
use startpage::utils::calculate_expires;

fn drop_rocket(meta: &log::Metadata) -> bool {
    let name = meta.target();
    if name.starts_with("rocket") || name.eq("_") {
        return false;
    }
    true
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .filter(drop_rocket)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    if !cfg!(debug_assertions) {
        setup_logger().expect("Failed to setup logger");
    }

    dotenv().expect("Failed to read .env file");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET");

    let jwt_expire_in = std::env::var("JWT_EXPIRES_IN").expect("JWT_EXPIRES_IN");

    let jwt_expiration = calculate_expires(&jwt_expire_in).expect("Failed to calculate expires");

    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let state = AppState {
        pool,
        jwt_secret,
        jwt_expiration,
    };

    let _rok = rocket::build()
        .manage(state)
        .mount("/api/user", routes![user::me])
        .mount("/api/auth", routes![auth::login])
        .launch()
        .await?;

    Ok(())
}
