use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{error::Error, service::cronograma_service::CronogramaService, AppState};

/// Gets all the cronogramas corresponding to the provided user ID
/// - ex: `/api/cronograma/user/1`
pub async fn get_cronograma_by_user_id(
    Path(user_id): Path<i32>,
    State(state): State<AppState>
) -> Result<impl IntoResponse, Error> {
    match CronogramaService::find_by_user_id(user_id, &state).await{
        Ok(cronogramas) => Ok((StatusCode::OK, Json(cronogramas))),
        Err(why) => Err(why),
    }
}