use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct Ip(pub(crate) Option<String>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Ip {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let ip = request.headers().get_one("CF-Connecting-IP");

        match ip {
            Some(ip) => Outcome::Success(Ip(Some(ip.to_string()))),
            None => Outcome::Success(Ip(None)),
        }
    }
}
