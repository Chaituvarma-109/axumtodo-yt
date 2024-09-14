use axum::Router;
use axum::routing::{get, post};

use crate::AppState;
use crate::handlers::{create_todo, delete_todo, health_check, list_todo, list_todos, update_todo};

pub async fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/api/health_check", get(health_check))
        .route("/api/todos/", post(create_todo))
        .route("/api/todos", get(list_todos))
        .route(
            "/api/todos/:id",
            get(list_todo)
                .patch(update_todo)
                .delete(delete_todo),
        )
        .with_state(app_state)
}
