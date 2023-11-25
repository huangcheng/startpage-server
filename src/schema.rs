// @generated automatically by Diesel CLI.

diesel::table! {
    user (username) {
        #[max_length = 20]
        username -> Varchar,
        #[max_length = 20]
        nickname -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        avatar -> Nullable<Varchar>,
    }
}
