use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct AddGroupInput{
    pub user_id: i32,
    pub group_id: i32,
}