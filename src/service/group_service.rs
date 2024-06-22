use crate::{dto::groups_dto::{CreateGroupsDto, GroupsDto}, error::Result, model::{timer::Timer, timer_group::TimerGroup, user::User}, AppState};

pub struct GroupService;

impl GroupService {
    pub async fn create_group(user_id: i32, create_group_dto: CreateGroupsDto, state: &AppState) -> Result<GroupsDto> {
        let timer_group = TimerGroup::create(user_id, &create_group_dto.name, &state).await?;
        for timer in &create_group_dto.timers {
            Timer::insert_into_timer_timer_group(timer.timer_id, timer_group.timer_group_id, &state).await?;
        }
        let group_dto = GroupsDto{ timer_group_id: timer_group.timer_group_id, name: timer_group.name, timers: create_group_dto.timers, owner: user_id};
        User::add_group(user_id, group_dto.timer_group_id, &state).await?;
        Ok(group_dto)
    }

    pub async fn find_by_user_id(user_id: i32, state: &AppState) -> Result<Vec<GroupsDto>>{
        let timer_groups = TimerGroup::find_by_user_id(user_id, state).await?;
        let mut timer_groups_dto = vec![];
        for timer_group in timer_groups {
            timer_groups_dto.push(
                GroupsDto { 
                    name: timer_group.name, 
                    timer_group_id: timer_group.timer_group_id,
                    owner: timer_group.owner,
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