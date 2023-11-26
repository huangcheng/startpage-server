use cookie::time::Duration;
use rocket::http::{ContentType, Cookie};
use rocket::response::Responder;

const COOKIE_MAX_AGE: i64 = 2147483647;

#[derive(Debug)]
pub struct JwtToken {
    pub token: String,
}

impl<'r> Responder<'r, 'static> for JwtToken {
    fn respond_to(self, _: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut cookie = Cookie::new("token", self.token);

        cookie.set_max_age(Duration::seconds(COOKIE_MAX_AGE));
        cookie.set_path("/");

        rocket::Response::build()
            .header(ContentType::JSON)
            .header(cookie)
            .sized_body(0, std::io::Cursor::new(""))
            .ok()
    }
}

pub struct Logout;

impl<'r> Responder<'r, 'static> for Logout {
    fn respond_to(self, _: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut cookie = Cookie::new("token", "");

        cookie.set_max_age(Duration::seconds(0));
        cookie.set_path("/");

        rocket::Response::build()
            .header(ContentType::JSON)
            .header(cookie)
            .sized_body(0, std::io::Cursor::new(""))
            .ok()
    }
}
