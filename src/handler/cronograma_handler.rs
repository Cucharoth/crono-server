use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{dto::cronograma_dto::{AddTimerToCronograma, CronogramaDto}, error::Error, service::cronograma_service::CronogramaService, utils::jwt::Claims, AppState};

/// Adds a new timer with the provided `TimerDto`
/// - ex: /api/cronograma/add-timer
pub async fn add_timer(
    State(state): State<AppState>,
    user: Claims,
    Json(add_timer_to_cronograma): Json<AddTimerToCronograma>
) -> Result<impl IntoResponse, Error> {
    match CronogramaService::add_timer(user.id, add_timer_to_cronograma, &state).await{
        Ok(timer) => Ok((StatusCode::OK, Json(timer))),
        Err(why) => Err(why),
    }
}

/// Deletes the cronograma with the provided `CronogramaDto`
/// - ex: /api/cronograma/delete
pub async fn delete_cronograma(
    State(state): State<AppState>,
    user: Claims,
    Json(cronograma): Json<CronogramaDto>
) -> Result<impl IntoResponse, Error> {
    match CronogramaService::delete_cronograma(user.id, cronograma, &state).await{
        Ok(_) => Ok((StatusCode::OK, "Borrado exitosamente")),
        Err(why) => Err(why),
    }
}

/// Edits the cronograma with the provided `CronogramaDto`
/// - ex: /api/cronograma/update
pub async fn edit_cronograma(
    State(state): State<AppState>,
    user: Claims,
    Json(cronograma): Json<CronogramaDto>
) -> Result<impl IntoResponse, Error> {
    match CronogramaService::edit_cronograma(user.id, cronograma, &state).await{
        Ok(cronograma) => Ok((StatusCode::OK, Json(cronograma))),
        Err(why) => Err(why),
    }
}

/// Creates a new cronograma with the provided `CronogramaDto`
/// - ex: /api/cronograma/new
pub async fn create_cronograma(
    State(state): State<AppState>,
    user: Claims,
    Json(cronograma): Json<CronogramaDto>
) -> Result<impl IntoResponse, Error> {
    match CronogramaService::create_cronograma(user.id, cronograma, &state).await{
        Ok(cronograma) => Ok((StatusCode::OK, Json(cronograma))),
        Err(why) => Err(why),
    }
}

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