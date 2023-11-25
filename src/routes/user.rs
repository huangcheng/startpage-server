use rocket::put;

#[put("/<username>", format = "json")]
pub async fn update(username: &str) -> String {
    format!("update user: {}", username)
}
