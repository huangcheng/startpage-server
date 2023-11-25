use actix_web::{self, App, HttpServer, web};
use diesel::prelude::*;
use dotenvy::dotenv;


use startpage::state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Failed to load .env file");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let host = std::env::var("SERVER_HOST").expect("HOST must be set");
    let port = std::env::var("SERVER_PORT").expect("PORT must be set");

    let server_addr = format!("{}:{}", host, port);

    HttpServer::new(move || {
        let connection = MysqlConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));

        App::new()
            .app_data(web::Data::new(AppState { connection }))
            .configure(startpage::routes::user::user_routes)
    })
        .bind(server_addr)?
        .run()
        .await
}
