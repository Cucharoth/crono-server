use std::env;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};


use crate::error::{Error, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub id: i32,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(id: i32) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            id: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

#[async_trait]
impl<B> FromRequestParts<B> for Claims
where
    B: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &B) -> std::result::Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| Error::WrongCredentials)?;
        let claims = verify(bearer.token())?;
        Ok(claims)
    }
}

pub fn sign(id: i32) -> Result<String> {
    
    let jwt_secret: String = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(id),
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )?)
}

pub fn verify(token: &str) -> Result<Claims> {
    let jwt_secret: String = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ).map_err(|_| Error::WrongCredentials)?;
    Ok(token_data.claims)
}