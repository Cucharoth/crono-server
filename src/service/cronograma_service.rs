use crate::{model::cronograma::Cronograma, error::Result, AppState};


pub struct CronogramaService;

impl CronogramaService {
    pub async fn find_by_user_id(user_id: i32, state: &AppState) -> Result<Vec<Cronograma>>{
        Ok(Cronograma::find_by_user_id(user_id, state).await?)
    }
}