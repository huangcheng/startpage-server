use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateUser<'r> {
    pub username: Option<&'r str>,
    pub email: Option<&'r str>,
    pub avatar: Option<&'r str>,
    pub nickname: Option<&'r str>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePassword<'r> {
    pub password: &'r str,
    pub new_password: &'r str,
}
