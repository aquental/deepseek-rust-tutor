mod controller;
mod models;
mod routes;
mod service;
mod session;

use axum::{
    Router,
    extract::Extension,
    routing::{get, post},
};
use controller::TutorController;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let controller = Arc::new(Mutex::new(TutorController::new()));

    let app = Router::new()
        .route("/", get(routes::root))
        .route("/api/create_session", post(routes::create_session))
        .layer(Extension(controller));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind");
    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
