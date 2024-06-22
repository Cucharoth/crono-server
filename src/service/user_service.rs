use crate::{dto::groups_dto::GroupsDto, error::Result, model::{timer::Timer, timer_group::TimerGroup, user::User}, AppState};


pub struct UserService;

impl UserService {
    pub async fn add_group(user_id: i32, group_id: i32, state: &AppState) -> Result<GroupsDto> {
        let user_timer_group = User::add_group(user_id, group_id, state).await?;
        let timer_group = TimerGroup::find_by_id(group_id, state).await?;
        let timers = Timer::find_by_group_id(group_id, state).await?;
        Ok(GroupsDto { timer_group_id: user_timer_group.timer_group_id, name: timer_group.name, timers, owner: timer_group.owner })
    }
}