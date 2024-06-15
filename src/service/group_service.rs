use crate::{model::timer_group::TimerGroup, error::Result, AppState};

pub struct GroupService;

impl GroupService {
    pub async fn find_by_user_id(user_id: i32, state: &AppState) -> Result<Vec<TimerGroup>>{
        Ok(TimerGroup::find_by_user_id(user_id, state).await?)
    }

    pub async fn find_all(state: &AppState) -> Result<Vec<TimerGroup>>{
        Ok(TimerGroup::find_all(state).await?)
    }
}