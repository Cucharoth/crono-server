use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{error::Error, service::cronograma_service::CronogramaService, utils::jwt::Claims, AppState};

/// Gets all the cronogramas corresponding to the provided `user`
/// - ex: `/api/cronograma/user`
pub async fn get_cronograma_by_user(
    State(state): State<AppState>,
    user: Claims
) -> Result<impl IntoResponse, Error> {
    match CronogramaService::find_by_user_id(user.id, &state).await{
        Ok(cronogramas) => Ok((StatusCode::OK, Json(cronogramas))),
        Err(why) => Err(why),
    }
}