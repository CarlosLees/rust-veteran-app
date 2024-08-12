use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LitemallInfoVeteran {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub address: String,
}
