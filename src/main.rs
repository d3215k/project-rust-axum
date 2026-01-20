mod error;
mod routes;
mod state;
mod layers;
mod models;

use crate::state::AppState;
use axum::routing::{patch, post};
use axum::{middleware, Router};
use sqlx::PgPool;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set");

    let db_pool = PgPool::connect(database_url.as_str())
        .await
        .expect("Can't connect to database");

    let app_state = AppState {
        db: db_pool,
    };

    let public_routes = Router::new()
        .route("/login", post(routes::auth::login))
        .route("/register", post(routes::user::create));


    let protected_routes = Router::new()
        .route("/tasks", post(routes::task::create).get(routes::task::list))
        .route("/tasks/{id}", patch(routes::task::update).delete(routes::task::delete).get(routes::task::show))
        .layer(middleware::from_fn(layers::auth::validate_token));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}