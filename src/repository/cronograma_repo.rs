use crate::{
    error::Result, model::{cronograma::Cronograma}, AppState
};

impl Cronograma {
    pub async fn find_by_user_id(user_id: i32, state: &AppState) -> Result<Vec<Cronograma>> {
        let sql = format!("SELECT * FROM cronograma WHERE user_account_id = $1");
        Ok(sqlx::query_as::<_, Cronograma>(&sql).bind(user_id).fetch_all(&state.db).await?)
    }
}