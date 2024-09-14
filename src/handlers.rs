use crate::{
    model::Todo,
};

use axum::{
    response::IntoResponse,
    extract::{Path, State},
    http::StatusCode,
    Json
};

use crate::AppState;
use crate::schema::CreateTodo;

pub async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn list_todos(State(data): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        Todo,
        r#"SELECT * FROM todo ORDER BY id"#
    )
        .fetch_all(&data.db)
        .await;

    match query_result {
        Ok(todos) => {
            let json_response = serde_json::json!({
                "status": "success",
                "todos":todos
            });
            Ok(Json(json_response))
        }
        Err(err) => {
            let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all note items",
        });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

pub async fn create_todo(Json(body): Json<CreateTodo>, State(data): State<AppState>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result  = sqlx::query_as!(
        Todo,
        "INSERT INTO todo (title, completed) VALUES ($1, $2) RETURNING *",
        body.title.unwrap().to_string(),
        body.completed
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(todo) =>  {
            let todo_response = serde_json::json!({
                "status": "success",
                "data" : serde_json::json!({
                    "todo": todo,
                }),
            });

            Ok((StatusCode::CREATED, Json(todo_response)))
        }
        Err(err) => {
            if err.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Note with that title already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"status": "error","message": format!("{:?}", err)})),
            ))
        }
    }
}

pub async fn list_todo(Path(id): Path<i32>, app_state: AppState) -> impl IntoResponse {
    todo!()
}

pub async fn update_todo(Path(id): Path<i32>, app_state: AppState) -> impl IntoResponse {
    todo!()
}

pub async fn delete_todo(Path(id): Path<i32>, app_state: AppState) -> impl IntoResponse {
    todo!()
}
