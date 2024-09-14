use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Todo {
    id: i32,
    title: String,
    completed: bool,
}
