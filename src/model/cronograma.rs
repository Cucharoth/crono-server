use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Cronograma{
    pub cronograma_id: i32,
    pub name: String,
}