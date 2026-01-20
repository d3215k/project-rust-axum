use std::env;
use axum::extract::State;
use axum::Json;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use crate::error::AppError;
use crate::models::auth::{Claim, LoginRequest, LoginResponse};
use crate::state::AppState;

pub async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let row = sqlx::query!(
        "SELECT id, email, name, password FROM users WHERE email = $1",
        payload.email
    )
        .fetch_one(&app_state.db)
        .await?;

    let verified = bcrypt::verify(payload.password, &row.password)
        .map_err(|_| AppError::InvalidLogin("Invalid login".to_string()))?;

    if !verified {
        return Err(AppError::InvalidLogin("Invalid login".to_string()));
    }

    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(60))
        .ok_or_else(|| AppError::InvalidLogin("Invalid login".to_string()))?;

    let claims = Claim {
        sub: row.id,
        exp: expiration.timestamp() as usize,
    };

    let jwt_secret = env::var("JWT_SECRET")
        .map_err(|_| AppError::InvalidLogin("Invalid login".to_string()))?;

    let header = Header::default();

    let key = EncodingKey::from_secret(jwt_secret.as_ref());

    let jwt = jsonwebtoken::encode(
        &header,
        &claims,
        &key,
    ).map_err(|_| AppError::InvalidLogin("Invalid login".to_string()))?;

    Ok(Json(LoginResponse {
        token: jwt
    }))


}