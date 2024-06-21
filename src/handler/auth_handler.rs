use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};


use crate::{
    dto::auth_dto::{LoginInput, RegisterInput, SocialLogin, TokenPayload},
    error::Error,
    service::auth_service::AuthService,
    utils::{jwt, validate_payload},
    AppState
};

/// Grants access to an existing user using the provided `SocialLogin`.
/// 
/// If the user doesn't previously exists, it's registered and validated.
/// - ex: `/api/user/social-login`
/// returns a `TokenPayload`
pub async fn social_login(
    State(state): State<AppState>,
    Json(social_login): Json<SocialLogin>,
) -> Result<impl IntoResponse, Error> {
    let user = AuthService::social_login(social_login.email, social_login.name, &state).await?;
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

/// Grants access to an existing user validating the provided `LoginInput`.
/// - ex: `/api/user/login`
/// returns a `TokenPayload`
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


/// Registers a new user into the system.
/// - ex: `/api/user/register`
pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterInput>,
) -> Result<impl IntoResponse, Error> {
    validate_payload(&input)?;
    let _user = AuthService::sign_up(input, &state).await?;
    Ok(StatusCode::CREATED)
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    _client_id: String,
    _client_secret: String,
}

#[derive(Debug, Serialize)]
struct _AuthBody {
    access_token: String,
    token_type: String,
}

