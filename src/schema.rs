// @generated automatically by Diesel CLI.

diesel::table! {
    app_users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        firstname -> Varchar,
        #[max_length = 255]
        lastname -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        is_active -> Bool,
    }
}
