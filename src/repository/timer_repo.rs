use crate::{
    error::Result, model::timer::Timer, AppState
};

impl Timer {
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
}