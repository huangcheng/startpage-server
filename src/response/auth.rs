use serde::{Serialize};
use rocket::http::{ContentType, Status, Cookie};
use rocket::response::Responder;
use serde_json;

#[derive(Debug, Serialize)]
pub struct User {
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub avatar: Option<String>,
}

pub struct JwtToken {
    pub token: String,
}

impl<'r> Responder<'r, 'static> for User {
    fn respond_to(self, _: &rocket::Request<'_>) -> rocket::response::Result<'static> {
    let content = serde_json::to_string(&self).map_err(|_| Status::InternalServerError)?;

        rocket::Response::build()
            .header(ContentType::JSON)
            .sized_body(content.len(), std::io::Cursor::new(content))
            .ok()
    }
}

impl<'r> Responder<'r, 'static> for JwtToken {
    fn respond_to(self, _: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        let cookie = Cookie::new("token", self.token);

        rocket::Response::build()
            .header(ContentType::JSON)
            .header(cookie)
            .sized_body(0, std::io::Cursor::new(""))
            .ok()
    }
}
