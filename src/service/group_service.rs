use crate::{dto::groups_dto::GroupsDto, error::Result, model::{timer::Timer, timer_group::TimerGroup}, AppState};

pub struct GroupService;

impl GroupService {
    pub async fn find_by_user_id(user_id: i32, state: &AppState) -> Result<Vec<GroupsDto>>{
        let timer_groups = TimerGroup::find_by_user_id(user_id, state).await?;
        let mut timer_groups_dto = vec![];
        for timer_group in timer_groups {
            timer_groups_dto.push(
                GroupsDto { 
                    name: timer_group.name, 
                    timer_group_id: timer_group.timer_group_id,
                    timers: Timer::find_by_group_id(timer_group.timer_group_id, &state).await?,
                }
            )    
        }      
        Ok(timer_groups_dto)
    }

    pub async fn find_all(state: &AppState) -> Result<Vec<TimerGroup>>{
        Ok(TimerGroup::find_all(state).await?)
    }
}