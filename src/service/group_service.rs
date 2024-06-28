use crate::{dto::groups_dto::{CreateGroupsDto, GroupsDto, TimerGroupDto}, error::{Error, Result}, model::{timer::Timer, timer_group::TimerGroup, user::User}, AppState};

pub struct GroupService;

impl GroupService {
    pub async fn delete_group(user_id: i32, timer_group_dto: TimerGroupDto, state: &AppState) -> Result<()> {
        let current_group = TimerGroup::find_by_id(timer_group_dto.timer_group_id, &state).await?;
        if current_group.owner != user_id {
            return Err(Error::NotTheGroupOwner);
        }
        TimerGroup::delete(timer_group_dto.timer_group_id, &state).await?;
        Ok(())
    }

    pub async fn create_group(user_id: i32, create_group_dto: CreateGroupsDto, state: &AppState) -> Result<GroupsDto> {
        if TimerGroup::find_by_group_name_and_owner_id(user_id, &create_group_dto.name, &state).await.is_ok() {
            return Err(Error::DuplicateUserGroupName);
        }
        let timer_group = TimerGroup::create(user_id, &create_group_dto.name, &state).await?;
        for timer in &create_group_dto.timers {
            Timer::insert_into_timer_timer_group(timer.timer_id, timer_group.timer_group_id, &state).await?;
        }
        let group_dto = GroupsDto{ 
            timer_group_id: timer_group.timer_group_id,
            name: timer_group.name,
            timers: create_group_dto.timers,
            is_owner: timer_group.owner == user_id,
        };
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
                    is_owner: timer_group.owner == user_id,
                    timers: Timer::find_by_group_id(timer_group.timer_group_id, &state).await?,
                }
            )    
        }      
        Ok(timer_groups_dto)
    }

    pub async fn find_all(state: &AppState) -> Result<Vec<TimerGroupDto>>{
        Ok(TimerGroup::find_all(state).await?)
    }
}