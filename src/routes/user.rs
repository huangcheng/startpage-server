use actix_web::web::{self, ServiceConfig};

use crate::handlers;
pub fn user_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/", web::post().to(handlers::user::update_user))
    );
}
