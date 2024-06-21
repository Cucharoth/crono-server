use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{error::Error, service::group_service::GroupService, utils::jwt::Claims, AppState};

/// Gets all the groups
/// - ex: `/api/groups`
pub async fn get_groups(
    State(state): State<AppState>,
    _user: Claims
) -> Result<impl IntoResponse, Error> {
    match GroupService::find_all(&state).await {
        Ok(timer_groups) => Ok((StatusCode::OK, Json(timer_groups))),
        Err(why) => Err(why),
    }
}

/// Gets all the groups corresponding to the provided `user`
/// - ex: `/api/groups/user`
pub async fn get_groups_by_user(
    State(state): State<AppState>,
    user: Claims
) -> Result<impl IntoResponse, Error> {
    match GroupService::find_by_user_id(user.id, &state).await{
        Ok(timer_groups) => Ok((StatusCode::OK, Json(timer_groups))),
        Err(why) => Err(why),
    }
}