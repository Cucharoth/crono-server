use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TimerGroup{
    pub timer_group_id: i32,
    pub name: String,
    pub owner: i32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct UserTimerGroup{
    pub user_account_id: i32,
    pub timer_group_id: i32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TimerTimerGroup{
    pub timer_group_id: i32,
    pub timer_id: i32,
}