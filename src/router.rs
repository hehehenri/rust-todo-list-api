use axum::{Router, routing::get};
use sqlx::Error;
use crate::repositories::todos;

pub fn new() -> Result<Router, Error> {
    let todos_router = Router::new()
        .route("/", get(todos::all()));

    let router = Router::new()
        .nest("/todos", todos_router);

    Ok(router)
}