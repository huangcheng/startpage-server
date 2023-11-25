use diesel::prelude::*;

pub struct AppState {
    pub connection: MysqlConnection,
}
