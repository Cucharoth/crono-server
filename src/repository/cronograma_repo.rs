use crate::{
    dto::cronograma_dto::{AddTimerToCronograma, CronogramaDto}, error::Result, model::{cronograma::Cronograma, timer::Timer}, AppState
};

impl Cronograma {
    pub async fn add_timer(add_timer_to_cronograma: AddTimerToCronograma, state: &AppState) -> Result<Timer> {
        let sql = format!(
            "
                INSERT INTO timer (name, seconds, cronograma_id)
                VALUES
                ($1, $2, $3)
                RETURNING *
            "
        );
        Ok(sqlx::query_as::<_, Timer>(&sql)
                .bind(add_timer_to_cronograma.timer.name)
                .bind(add_timer_to_cronograma.timer.seconds)
                .bind(add_timer_to_cronograma.cronograma_id)
                .fetch_one(&state.db)
                .await?)
    }

    pub async fn delete(cronograma: CronogramaDto, state: &AppState) -> Result<Cronograma> {
        let sql = format!(
            "
                DELETE FROM cronograma
                WHERE cronograma_id = $1
                RETURNING *
            "
        );
        Ok(sqlx::query_as::<_, Cronograma>(&sql).bind(cronograma.cronograma_id).fetch_one(&state.db).await?)
    }

    pub async fn update(cronograma: CronogramaDto, state: &AppState) -> Result<CronogramaDto> {
        let sql = format!(
            "
                UPDATE cronograma
                SET name = $1
                WHERE cronograma_id = $2
                RETURNING *
            "
        );
        Ok(sqlx::query_as::<_, CronogramaDto>(&sql).bind(cronograma.name).bind(cronograma.cronograma_id).fetch_one(&state.db).await?)
    }

    pub async fn create(user_id: i32, cronograma: CronogramaDto, state: &AppState) -> Result<CronogramaDto> {
        let sql = format!(
            "
                INSERT INTO cronograma (name, user_account_id)
                VALUES
                ($1, $2) 
                RETURNING *
            "
        );
        Ok(sqlx::query_as::<_, CronogramaDto>(&sql).bind(cronograma.name).bind(user_id).fetch_one(&state.db).await?)
    }

    pub async fn find_by_user_id(user_id: i32, state: &AppState) -> Result<Vec<Cronograma>> {
        let sql = format!("SELECT * FROM cronograma WHERE user_account_id = $1");
        Ok(sqlx::query_as::<_, Cronograma>(&sql).bind(user_id).fetch_all(&state.db).await?)
    }

    pub async fn find_by_id(cronograma_id: i32, state: &AppState) -> Result<Cronograma> {
        let sql = format!("SELECT * FROM cronograma WHERE cronograma_id = $1");
        Ok(sqlx::query_as::<_, Cronograma>(&sql).bind(cronograma_id).fetch_one(&state.db).await?)
    }
}