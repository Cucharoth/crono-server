use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{dto::groups_dto::{CreateGroupsDto, TimerGroupDto}, error::Error, service::group_service::GroupService, utils::jwt::Claims, AppState};

/// Deletes the provided `TimerGroup`
/// - ex: `/api/groups/delete`
pub async fn delete_group(
    State(state): State<AppState>,
    user: Claims,
    Json(timer_group_dto): Json<TimerGroupDto>
) -> Result<impl IntoResponse, Error> {
    match GroupService::delete_group(user.id, timer_group_dto, &state).await {
        Ok(_) => Ok((StatusCode::OK, "¡Borrado exitosamente!")),
        Err(why) => Err(why),
    }
}

/// Creates new group with provided Schedule
/// - ex: `/api/groups/new`
pub async fn create_group(
    State(state): State<AppState>,
    user: Claims,
    Json(create_group_dto): Json<CreateGroupsDto>
) -> Result<impl IntoResponse, Error> {
    match GroupService::create_group(user.id, create_group_dto, &state).await {
        Ok(new_group_dto) => Ok((StatusCode::OK, Json(new_group_dto))),
        Err(why) => Err(why),
    }
}

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