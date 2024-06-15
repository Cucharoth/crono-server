use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{error::Error, service::timer_service::TimerService, AppState};

/// Gets all the timers corresponding to the provided group id
/// - ex: `/api/timers/group/1`
pub async fn get_timers_by_group_id(
    Path(timer_group_id): Path<i32>,
    State(state): State<AppState>
) -> Result<impl IntoResponse, Error> {
    match TimerService::find_by_group_id(timer_group_id, &state).await{
        Ok(timer) => Ok((StatusCode::OK, Json(timer))),
        Err(why) => Err(why),
    }
}

/// Gets all the timers corresponding to the provided cronograma id
/// - ex: `/api/timers/cronograma/1`
pub async fn get_timers_by_cronograma_id(
    Path(timer_cronograma_id): Path<i32>,
    State(state): State<AppState>
) -> Result<impl IntoResponse, Error> {
    match TimerService::find_by_cronograma_id(timer_cronograma_id, &state).await{
        Ok(timer) => Ok((StatusCode::OK, Json(timer))),
        Err(why) => Err(why),
    }
}