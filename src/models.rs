use axum::{http::StatusCode, response::IntoResponse, Json};
use uuid::Uuid;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub is_active: bool,
}

#[derive(Debug)]
pub enum UserError {
    InternalServerError,
    NotFound(Uuid),
}

impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            UserError::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("User with id: {} not found", id),
            ),
            UserError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
        };
        (
            status,
            Json(json!({
                "resource": "UserModel",
                "message": err_msg,
                "happened": chrono::Utc::now().to_rfc3339()
            })),
        )
        .into_response()
    }
}
