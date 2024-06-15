use crate::{
    model::timer::Timer, 
    error::Result,
    AppState
};


pub struct TimerService;

impl TimerService {
    pub async fn find_by_group_id(group_id: i32, state: &AppState) -> Result<Vec<Timer>>{
        Ok(Timer::find_by_group_id(group_id, state).await?)
    }

    pub async fn find_by_cronograma_id(cronograma_id: i32, state: &AppState) -> Result<Vec<Timer>>{
        Ok(Timer::find_by_cronograma_id(cronograma_id, state).await?)
    }
}