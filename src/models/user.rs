use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub email: String,
    pub avatar: Option<String>,
}
