// / Gets all the timers corresponding to the provided group id
// / - ex: `/api/timers/group/1`
// pub async fn get_timers_by_group_id(
//     Path(timer_group_id): Path<i32>,
//     State(state): State<AppState>
// ) -> Result<impl IntoResponse, Error> {
//     match TimerService::find_by_group_id(timer_group_id, &state).await{
//         Ok(timer) => Ok((StatusCode::OK, Json(timer))),
//         Err(why) => Err(why),
//     }
// }