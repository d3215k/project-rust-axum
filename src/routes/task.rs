use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::{AppState, models::task::{CreateTask, Task, UpdateTask}};
use crate::error::AppError;

pub async fn create(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, AppError> {
    let data_result = sqlx::query_as!(
        Task,
        "INSERT INTO tasks (title) VALUES ($1) RETURNING id, title, completed",
        payload.title
    ).fetch_one(&app_state.db).await?;

    Ok(Json(data_result))
}

pub async fn list(
    State(app_state): State<AppState>
) -> Result<Json<Vec<Task>>, AppError> {
    let data_result = sqlx::query_as!(
        Task,
        "SELECT id, title, completed FROM tasks ORDER BY id DESC"
    ).fetch_all(&app_state.db).await?;

    Ok(Json(data_result))
}

pub async fn update(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTask>,
) -> Result<StatusCode, AppError> {
    let updated = sqlx::query!(
        "UPDATE tasks SET title = COALESCE($1, title), completed = COALESCE($2, completed) WHERE id = $3",
        payload.title,
        payload.completed,
        id
    ).execute(&app_state.db).await?;

    if updated.rows_affected() > 0 {
        Ok(StatusCode::OK)
    } else {
        Err(AppError::NotFound(format!("Task with id {} not found", id)))
    }
}

pub async fn delete(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let deleted = sqlx::query!(
        "DELETE FROM tasks WHERE id = $1",
        id
    ).execute(&app_state.db).await?;

    if deleted.rows_affected() > 0 {
        Ok(StatusCode::OK)
    } else {
        Err(AppError::NotFound(format!("Task with id {} not found", id)))
    }
}