use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Response};
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use thiserror::Error;

pub mod api;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("StatusCode::INTERNAL_SERVER_ERROR")]
    InternalServerError(#[from] DbErr),
    #[error("Entity not found")]
    NotFoundError(String, i32),
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let body = match self {
            DbError::InternalServerError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
            DbError::NotFoundError(entity_name, entity_id) => (
                StatusCode::NOT_FOUND,
                format!("Entity {} with id {} not found", entity_name, entity_id),
            ),
        };

        body.into_response()
    }
}
