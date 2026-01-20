use axum::extract::State;
use axum::{Extension, Json};
use crate::error::AppError;
use crate::models::auth::UserContext;
use crate::models::user::{CreateUser, User};
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