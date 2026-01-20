use axum::extract::{Path, State};
use axum::{Extension, Json};
use uuid::Uuid;
use crate::error::AppError;
use crate::models::auth::UserContext;
use crate::models::user::{CreateUser, User, UserWithTasks};
use crate::models::task::Task;
use crate::state::AppState;

pub async fn create(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateUser>
) -> Result<Json<User>, AppError> {

    let password_hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)
        .map_err(|_| AppError::InternalServerError())?;

    let data_result = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email",
        payload.name,
        payload.email,
        password_hash
    )
        .fetch_one(&app_state.db).await
        .map_err(|e| match &e {
            sqlx::Error::Database(db) if db.code() == Some("23505".into()) =>
                AppError::Conflict("Email already exists".into()),
            _ => AppError::DatabaseError(e)
        })?;

    Ok(Json(data_result))
}

pub async fn list(
    State(app_state): State<AppState>,
    Extension(user_ctx): Extension<UserContext>,
) -> Result<Json<Vec<User>>, AppError> {
    let data_result = sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users"
    ).fetch_all(&app_state.db).await?;

    Ok(Json(data_result))
}

pub async fn show(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserWithTasks>, AppError> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users WHERE id = $1",
        id,
    )
        .fetch_one(&app_state.db)
        .await?;

    let tasks = sqlx::query_as!(
        Task,
        "SELECT id, title, completed FROM tasks WHERE user_id = $1",
        user.id
    )
        .fetch_all(&app_state.db).await?;

    let response = UserWithTasks {
        user,
        tasks
    };

    Ok(Json(response))
}
