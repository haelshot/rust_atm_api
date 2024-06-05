// id  uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
// username varchar(255) NOT NULL,
// firstname varchar(255) NOT NULL,
// lastname varchar(255) NOT NULL,
// email varchar(255) NOT NULL,
// password varchar(255) NOT NULL,
// is_active BOOLEAN NOT NULL DEFAULT FALSE
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub is_active: bool
}

#[derive(Debug)]
pub enum UserError {
    InternalServerError,
    NotFound(Uuid),
}

impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            StatusCode::NotFound(id) => (
                StatusCode::NotFound,
                format!("User with id: {} not found", id),
            ),

            _ => (
                StatusCode::InternalServerError,
                String::from("Internal server error"),
            ),

        };
        (
            status,
            Json(
                json!({"resource": "UserModel", "message": "err_msg", "happened": chrono::Utc::now() }),
            )
        )
        .into_response()
    }
}