use axum::{debug_handler, extract, Extension, Json};

pub mod todo_controller {
    use http::StatusCode;

    use super::*;
    use crate::{
        models::{Todo, TodoCreatePayload, TodoUpdatePayload},
        repositories::TodoRepository
    };

    #[debug_handler]
    pub async fn all(repository: Extension<TodoRepository>) -> Result<Json<Vec<Todo>>, StatusCode> {
        println!("Request: GET -> /todos/");

        match repository.all().await {
            Ok(todos) => Ok(Json(todos)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }

    #[debug_handler]
    pub async fn create(
        extract::Json(payload): extract::Json<TodoCreatePayload>,
        repository: Extension<TodoRepository>,
    ) -> Result<Json<Todo>, StatusCode> {
        println!("Request: POST -> /todos/");

        match repository.create(payload).await {
            Ok(todo) => Ok(Json(todo)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    #[debug_handler]
    pub async fn find(
        extract::Path(todo_id): extract::Path<u32>,
        repository: Extension<TodoRepository>,
    ) -> Result<Json<Todo>, StatusCode> {
        println!("Request: GET -> /todos/{}", todo_id);

        match repository.find(todo_id).await {
            Ok(todo) => Ok(Json(todo)),
            Err(error) => match error {
                sqlx::Error::RowNotFound => Err(StatusCode::NOT_FOUND),
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
        }
    }

    #[debug_handler]
    pub async fn update(
        extract::Path(todo_id): extract::Path<u32>,
        extract::Json(payload): extract::Json<TodoUpdatePayload>,
        repository: Extension<TodoRepository>
    ) -> Result<Json<Todo>, StatusCode> {
        println!("Request: PUT -> /todos/{}", todo_id);

        match repository.update(todo_id, payload).await {
            Ok(todo) => Ok(Json(todo)),
            Err(error) => match error {
                sqlx::Error::RowNotFound => Err(StatusCode::NOT_FOUND),
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }

    #[debug_handler]
    pub async fn delete(
        extract::Path(todo_id): extract::Path<u32>,
        repository: Extension<TodoRepository>
    ) -> Result<StatusCode, StatusCode> {
        println!("Request: DELETE -> /todos/{}", todo_id);

        match repository.delete(todo_id).await {
            Ok(_) => Ok(StatusCode::OK),
            Err(error) => match error {
                sqlx::Error::RowNotFound => Err(StatusCode::NOT_FOUND),
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }
}
