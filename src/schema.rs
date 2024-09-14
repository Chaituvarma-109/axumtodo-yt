use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodo {
    pub title: Option<String>,
    pub completed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTodo {
    pub title: String,
    pub completed: Option<bool>,
}
