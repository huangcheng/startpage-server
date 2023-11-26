use rocket::http::{ContentType, Cookie};
use rocket::response::Responder;

#[derive(Debug)]
pub struct JwtToken {
    pub token: String,
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
