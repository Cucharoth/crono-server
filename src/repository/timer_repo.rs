

use crate::{
    error::Result, model::{timer::Timer, timer_group::TimerTimerGroup}, AppState
};

impl Timer {
    pub async fn insert_into_timer_timer_group(timer_id: i32, timer_group_id: i32, state: &AppState) -> Result<()> {
        let sql = format!(
            "
                INSERT INTO timer_timer_group (timer_group_id, timer_id)
                VALUES
                ($1, $2)
                RETURNING *
            " 
        );
        let _response = sqlx::query_as::<_, TimerTimerGroup>(&sql).bind(timer_group_id).bind(timer_id).fetch_one(&state.db).await?;
        Ok(())
    }

    pub async fn find_by_group_id(group_id: i32, state: &AppState) -> Result<Vec<Timer>> {
        let sql = format!(
            "
                SELECT * 
                FROM timer_timer_group
                JOIN timer_group USING (timer_group_id)
                JOIN timer USING (timer_id)
                WHERE timer_group_id = $1
            " 
            );
        Ok(sqlx::query_as::<_, Timer>(&sql).bind(group_id).fetch_all(&state.db).await?)
    }

    pub async fn find_by_cronograma_id(cronograma_id: i32, state: &AppState) -> Result<Vec<Timer>> {
        let sql = format!("SELECT * FROM timer WHERE cronograma_id = $1");
        Ok(sqlx::query_as::<_, Timer>(&sql).bind(cronograma_id).fetch_all(&state.db).await?)
    }


    // todo: query as vector for better performance
    // pub async fn insert_into_timer_timer_group_by_id_vector(timers_id: Vec<i32>, timer_group_id: i32, state: &AppState) -> Result<Vec<Timer>> {
    //     let sql = format!(
    //         "
    //             INSERT INTO timer_timer_group (timer_group_id, timer_id)
    //             VALUES
    //         " 
    //         );
    //     let mut tx = state.db.begin().await?;
    //     let mut timers = Vec::new();
        
    //     for timer_id in timers_id {
    //         let timer = sqlx::query_as::<_, Timer>(&sql)
    //             .bind(timer_group_id)
    //             .bind(timer_id)
    //             .fetch_one(&mut tx)
    //             .await?;
    //         timers.push(timer);
    //     }
    
    //     tx.commit().await?;
    //     Ok(sqlx::query_as::<_, Timer>(&sql).bind(timer_group_id).bind(timers_id).fetch_all(&state.db).await?)
    // }
}