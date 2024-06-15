use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{error::Error, service::group_service::GroupService, AppState};

/// Gets all the groups
/// - ex: `/api/groups`
pub async fn get_groups(
    State(state): State<AppState>
) -> Result<impl IntoResponse, Error> {
    match GroupService::find_all(&state).await {
        Ok(timer_groups) => Ok((StatusCode::OK, Json(timer_groups))),
        Err(why) => Err(why),
    }
}

/// Gets all the groups corresponding to the provided user id
/// - ex: `/api/groups/user/1`
pub async fn get_groups_by_user_id(
    Path(user_id): Path<i32>,
    State(state): State<AppState>
) -> Result<impl IntoResponse, Error> {
    match GroupService::find_by_user_id(user_id, &state).await{
        Ok(timer_groups) => Ok((StatusCode::OK, Json(timer_groups))),
        Err(why) => Err(why),
    }
}