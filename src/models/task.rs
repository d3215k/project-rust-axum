use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub completed: Option<bool>,
}