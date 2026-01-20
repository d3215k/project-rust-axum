use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::task::Task;

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct UserWithTasks {
    #[serde(flatten)]
    pub user: User,
    pub tasks: Vec<Task>,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdatePassword {
    pub password: String,
}