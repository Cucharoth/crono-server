use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

use crate::model::timer::Timer;

use super::timer_dto::TimerDto;


#[derive(Debug, FromRow, Deserialize, Serialize, Validate)]
pub struct CronogramaDto {
    pub cronograma_id: i32,
    #[validate(length(min = 4, max = 15))]
    pub name: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize, Validate)]
pub struct AddTimerToCronograma {
    pub cronograma_id: i32,
    pub timer: TimerDto,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct CronogramaWithTimers {
    pub cronograma_id: i32,
    pub name: String,
    pub timers: Vec<Timer>,
}