use crate::{repositories::TodoRepository, controllers::todo_controller};
use axum::{Extension, Router, routing::{ get, post, put, delete }};
use sqlx::{Error, MySqlPool};

pub fn new(pool: MySqlPool) -> Result<Router, Error> {
    let router = Router::new()
        .nest("/todos", todos(pool));

    Ok(router)
}

fn todos(pool: MySqlPool) -> Router {
    Router::new()
        .route("/", get(todo_controller::all))
        .route("/", post(todo_controller::create))
        .route("/:id", get(todo_controller::find))
        .route("/:id", put(todo_controller::update))
        .route("/:id", delete(todo_controller::delete))
        .layer(Extension(TodoRepository::new(pool)))
}
