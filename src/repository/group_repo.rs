use crate::{
    error::Result, model::timer_group::TimerGroup, AppState
};

impl TimerGroup {
    pub async fn find_by_id(group_id: i32, state: &AppState) -> Result<TimerGroup> {
        let sql = format!("SELECT * FROM timer_group WHERE timer_group_id = $1");
        Ok(sqlx::query_as::<_, TimerGroup>(&sql).bind(group_id).fetch_one(&state.db).await?)
    }

    pub async fn find_by_user_id(user_id: i32, state: &AppState) -> Result<Vec<TimerGroup>> {
        let sql = format!(
            "
                SELECT utg.timer_group_id, tg.name 
                FROM user_timer_group utg
                JOIN timer_group tg USING (timer_group_id)
                JOIN user_account ua USING (user_account_id)
                WHERE user_account_id = $1
            "
        );
        Ok(sqlx::query_as::<_, TimerGroup>(&sql).bind(user_id).fetch_all(&state.db).await?)
    }

    pub async fn find_all(state: &AppState) -> Result<Vec<TimerGroup>> {
        let sql = format!("SELECT * FROM timer_group");
        Ok(sqlx::query_as::<_, TimerGroup>(&sql).fetch_all(&state.db).await?)
    }
}