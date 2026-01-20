use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use uuid::Uuid;
use crate::{AppState, models::task::{CreateTask, Task, UpdateTask}};
use crate::error::AppError;
use crate::models::auth::UserContext;

pub async fn create(
    State(app_state): State<AppState>,
    Extension(user_ctx): Extension<UserContext>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, AppError> {
    let data_result = sqlx::query_as!(
        Task,
        "INSERT INTO tasks (title, user_id) VALUES ($1, $2) RETURNING id, title, completed",
        payload.title,
        user_ctx.id
    ).fetch_one(&app_state.db).await?;

    Ok(Json(data_result))
}

pub async fn show(
    State(app_state): State<AppState>,
    Extension(user_ctx): Extension<UserContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<Task>, AppError> {
    let data_result = sqlx::query_as!(
        Task,
        r#"
        SELECT id, title, completed
        FROM tasks
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        user_ctx.id
    )
        .fetch_optional(&app_state.db)
        .await?;

    let task = data_result.ok_or(AppError::NotFound("Task not found".to_owned()))?;

    Ok(Json(task))
}

pub async fn list(
    State(app_state): State<AppState>,
    Extension(user_ctx): Extension<UserContext>,
) -> Result<Json<Vec<Task>>, AppError> {
    let data_result = sqlx::query_as!(
        Task,
        "SELECT id, title, completed FROM tasks WHERE user_id = $1 ORDER BY id DESC",
        user_ctx.id
    ).fetch_all(&app_state.db).await?;

    Ok(Json(data_result))
}

pub async fn update(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
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
    Path(id): Path<Uuid>,
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