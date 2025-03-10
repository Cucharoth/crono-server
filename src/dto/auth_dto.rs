use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::model::user::User;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginInput {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterInput {
    #[validate(length(min = 4, max = 15))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 15))]
    pub password: String,
}

#[derive(Debug)]
pub struct _AuthPayload {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize)]
pub struct TokenPayload {
    pub access_token: String,
    pub token_type: String,
    pub user_name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SocialLogin {
    pub email: String,
    pub name: String,
}