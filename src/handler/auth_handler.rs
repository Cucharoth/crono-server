use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::{
    dto::auth_dto::{LoginInput, RegisterInput, TokenPayload},
    error::{ApiError, ApiResult, Error},
    service::auth_service::AuthService,
    utils::{jwt, validate_payload},
    AppState
};

/// User login
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginInput>,
) -> Result<impl IntoResponse, Error> {
    validate_payload(&payload)?;
    let user = AuthService::sign_in(payload, &state)
        .await
        .map_err(|_| Error::WrongCredentials)?;
    let token = jwt::sign(user.user_account_id)?;
    Ok((
        StatusCode::OK,
        Json(TokenPayload {
            access_token: token,
            token_type: "Bearer".to_string(),
            user_name: user.name,
        }),
    ))
}

pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterInput>,
) -> Result<impl IntoResponse, Error> {
    validate_payload(&input)?;
    let user = AuthService::sign_up(input, &state).await?;
    Ok(StatusCode::CREATED)
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

