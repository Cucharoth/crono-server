use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::model::timer::Timer;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct GroupsDto {
    pub timer_group_id: i32,
    pub name: String,
    pub is_owner: bool,
    pub timers: Vec<Timer>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct CreateGroupsDto {
    pub timer_group_id: i32,
    pub name: String,
    pub timers: Vec<Timer>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TimerGroupDto {
    pub timer_group_id: i32,
    pub name: String,
    pub owner: i32,
    pub owner_name: String
}