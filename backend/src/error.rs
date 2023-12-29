use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(serde::Serialize)]
pub struct Error {
    pub status: u16,
    pub message: String,
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self {
            status: 500,
            message: value.to_string(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::from_u16(self.status).unwrap(),
            Json(json!({
                "message": self.message
            })),
        )
            .into_response()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ExtractorError {
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

impl IntoResponse for ExtractorError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ExtractorError::JsonExtractorRejection(json_rejection) => {
                (json_rejection.status(), json_rejection.body_text())
            }
        };

        Error {
            status: status.as_u16(),
            message,
        }
        .into_response()
    }
}

pub type ResponseResult<T> = Result<T, Error>;
