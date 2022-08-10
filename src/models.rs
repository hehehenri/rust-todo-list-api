use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Todo {
    id: i32,
    name: String,
    is_checked: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TodoCreatePayload {
    pub name: String,
    pub is_checked: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TodoUpdatePayload {
  pub name: Option<String>,
  pub is_checked: Option<bool>,
}

#[allow(dead_code)]
pub struct CreateTodo {
    pub name: String,
    pub is_checked: bool,
}

impl Todo {
    pub fn new(id: i32, name: String, is_checked: bool) -> Self {
        Todo {
            id,
            name,
            is_checked
        }
    }
}