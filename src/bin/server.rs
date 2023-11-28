use rocket::fairing::AdHoc;
use rocket::figment::providers::{Format, Serialized, Toml};
use rocket::figment::{Figment, Profile};
use rocket::fs::FileServer;
use rocket::{self, routes};
use rocket_db_pools::Database;

use startpage::config::Config;
use startpage::routes::upload::upload;
use startpage::routes::{auth, category, site, user};
use startpage::state::AppState;
use startpage::utils::calculate_expires;
use startpage::Db;

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

    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(Config::default()))
        .merge(Toml::file("Rocket.toml").nested())
        .select(Profile::from_env_or("ROCKET_PROFILE", "default"));

    let config = figment
        .extract::<Config>()
        .expect("Failed to extract app config");

    let jwt_expiration =
        calculate_expires(&config.jwt.expires_in).expect("Failed to parse duration");

    let upload_url = figment
        .extract::<Config>()
        .expect("Failed to extract app config")
        .upload_url;

    let upload_dir = figment
        .extract::<Config>()
        .expect("Failed to extract app config")
        .upload_dir;

    let state = AppState { jwt_expiration };

    let _rok = rocket::custom(figment)
        .manage(state)
        .attach(Db::init())
        .mount(
            "/api/user",
            routes![user::me, user::update, user::update_password],
        )
        .mount("/api/auth", routes![auth::login, auth::logout])
        .mount("/api/categories", routes![category::all])
        .mount(
            "/api/category",
            routes![
                category::update,
                category::add,
                category::delete,
                category::get_sites,
            ],
        )
        .mount("/api/sites", routes![site::all])
        .mount("/api/site", routes![site::add, site::update, site::delete])
        .mount("/api/upload", routes![upload])
        .mount(upload_url, FileServer::from(upload_dir))
        .attach(AdHoc::config::<Config>())
        .launch()
        .await?;

    Ok(())
}
