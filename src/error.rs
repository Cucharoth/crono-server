use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    TokioRecvError(#[from] tokio::sync::oneshot::error::RecvError),
    #[error(transparent)]
    AxumExtensionError(#[from] axum::extract::rejection::ExtensionRejection),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("wrong credentials")]
    WrongCredentials,
    #[error("password doesn't match")]
    WrongPassword,
    #[error("el correo ya existe")]
    DuplicateUserEmail,
    #[error("el nombre ya existe")]
    DuplicateUserName,
    #[error("¡Ya has creado un grupo con ese nombre!")]
    DuplicateUserGroupName,
    #[error("¡El usuario no es el autor del grupo!")]
    NotTheGroupOwner,
}
pub type Result<T> = std::result::Result<T, Error>;

pub type ApiError = (StatusCode, Json<Value>);
pub type _ApiResult<T> = std::result::Result<T, ApiError>;

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        let status = match err {
            Error::WrongCredentials => StatusCode::UNAUTHORIZED,
            Error::ValidationError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let payload = json!({"message": err.to_string()});
        (status, Json(payload))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            Error::WrongCredentials => (StatusCode::UNAUTHORIZED, "Credenciales invalidas".to_string()),
            Error::WrongPassword => (StatusCode::INTERNAL_SERVER_ERROR, "JWT signing error".to_string()),
            Error::SqlxError(why) => (StatusCode::BAD_REQUEST, why.to_string()),
            Error::DuplicateUserEmail => (StatusCode::BAD_REQUEST, Error::DuplicateUserEmail.to_string()),
            Error::DuplicateUserName => (StatusCode::BAD_REQUEST, Error::DuplicateUserName.to_string()),
            Error::DuplicateUserGroupName => (StatusCode::BAD_REQUEST, Error::DuplicateUserGroupName.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };
        (status, body).into_response()
    }
}