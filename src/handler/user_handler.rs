use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{dto::user_dto::AddGroupInput, error::Error, service::user_service::UserService, AppState};

/// Adds an existing user to an existing group
/// - ex: `/api/user/add-group`
/// 
/// returns the `Group`
pub async fn add_group_to_user(
    State(state): State<AppState>,
    Json(add_group_input): Json<AddGroupInput>
) -> Result<impl IntoResponse, Error> {
    match UserService::add_group(add_group_input.user_id, add_group_input.group_id, &state).await{
        Ok(group_dto) => Ok((StatusCode::CREATED, Json(group_dto))),
        Err(why) => Err(why),
    }
}