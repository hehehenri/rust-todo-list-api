use std::sync::Arc;
use sqlx::MySqlPool;
use crate::models::{Todo, TodoCreatePayload, TodoUpdatePayload};

#[derive(Clone)]
pub struct TodoRepository {
    db: Arc<MySqlPool>,
}

impl TodoRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            db: Arc::new(pool)
        }
    }

    pub fn db(&self) -> Arc<MySqlPool> {
        Arc::clone(&self.db)
    }

    pub async fn all(&self) -> Result<Vec<Todo>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM todos"
        ).fetch_all(self.db().as_ref())
        .await
    }

    pub async fn create(&self, payload: TodoCreatePayload) -> Result<Todo, sqlx::Error> {
        let created_id = sqlx::query!(
                "INSERT INTO todos (name, is_checked) VALUES (?, ?);",
                payload.name,
                payload.is_checked
            ).execute(self.db().as_ref()).await?
            .last_insert_id();

        Ok(Todo::new(created_id as i32, payload.name, payload.is_checked))
    }

    pub async fn find(&self, id: u32) -> Result<Todo, sqlx::Error> {
        sqlx::query_as("SELECT * FROM todos WHERE id = ?")
            .bind(id)
            .fetch_one(self.db().as_ref()).await
    }

    pub async fn update(&self, id: u32, payload: TodoUpdatePayload) -> Result<Todo, sqlx::Error> {
        sqlx::query!(
            "UPDATE todos SET name = ?, is_checked = ? WHERE id = ?",
            payload.name,
            payload.is_checked,
            id
        ).execute(self.db().as_ref()).await?;

        sqlx::query_as("SELECT * FROM todos WHERE id = ?")
            .bind(id)
            .fetch_one(self.db().as_ref()).await
    }

    pub async fn delete(&self, id: u32) -> Result<(), sqlx::Error> {
        match sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(self.db().as_ref()).await {
            Ok(_) => Ok(()),
            Err(error) => Err(error)
        }
    }
}
