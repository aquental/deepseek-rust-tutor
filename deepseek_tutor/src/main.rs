mod controller;
mod models;
mod routes;
mod service;
mod session;

use axum::{
    routing::{get, post},
    Router,
    extract::Extension,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use controller::TutorController;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {

    let controller = Arc::new(Mutex::new(TutorController::new()));

    // Define application routes and middleware
    let app = Router::new()
        .route("/", get(routes::root))
        .route("/api/create_session", post(routes::create_session))
        .route("/api/send_query", post(routes::send_query))
        .nest_service("/static", ServeDir::new("static"))
        .layer(Extension(controller));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");
    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
