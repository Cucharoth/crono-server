use crate::{dto::cronograma_dto::{AddTimerToCronograma, CronogramaDto, CronogramaWithTimers}, error::{Error, Result}, model::{cronograma::{self, Cronograma}, timer::Timer}, AppState};


pub struct CronogramaService;

impl CronogramaService {
    pub async fn add_timer(user_id: i32, add_timer_to_cronograma: AddTimerToCronograma, state: &AppState) -> Result<Timer>{
        if let Ok(current_cronograma) = Cronograma::find_by_id(add_timer_to_cronograma.cronograma_id, state).await {
            if current_cronograma.user_account_id != user_id {
                return Err(Error::NotTheCronogramaOwner);
            }
        } else {
            return Err(Error::NotFound);
        }
        Ok(Cronograma::add_timer(add_timer_to_cronograma, state).await?)
    }

    pub async fn delete_cronograma(user_id: i32, cronograma: CronogramaDto, state: &AppState) -> Result<()>{
        if let Ok(current_cronograma) = Cronograma::find_by_id(cronograma.cronograma_id, state).await {
            if current_cronograma.user_account_id != user_id {
                return Err(Error::NotTheCronogramaOwner);
            }
        } else {
            return Err(Error::NotFound);
        }
        Cronograma::delete(cronograma, state).await?;
        Ok(())
    }

    pub async fn edit_cronograma(user_id: i32, cronograma: CronogramaDto, state: &AppState) -> Result<CronogramaDto>{
        if let Ok(current_cronograma) = Cronograma::find_by_id(cronograma.cronograma_id, state).await {
            if current_cronograma.user_account_id != user_id {
                return Err(Error::NotTheCronogramaOwner);
            }
        } else {
            return Err(Error::NotFound);
        }
        Ok(Cronograma::update(cronograma, state).await?)
    }

    pub async fn create_cronograma(user_id: i32, cronograma: CronogramaDto, state: &AppState) -> Result<CronogramaDto>{
        Ok(Cronograma::create(user_id, cronograma, state).await?)
    }

    pub async fn find_by_user_id(user_id: i32, state: &AppState) -> Result<Vec<CronogramaWithTimers>>{
        let cronogramas = Cronograma::find_by_user_id(user_id, state).await?;
        let mut cronograma_with_timers = vec![];
        for cronograma in cronogramas {
            cronograma_with_timers.push(CronogramaWithTimers {
                cronograma_id: cronograma.cronograma_id,
                name: cronograma.name,
                timers: Timer::find_by_cronograma_id(cronograma.cronograma_id, state).await?
            }
        )}
        Ok(cronograma_with_timers)
    }
}