use actix_web::HttpResponse;
pub async fn update_user() -> HttpResponse {
    HttpResponse::Ok().body("update_user")
}
