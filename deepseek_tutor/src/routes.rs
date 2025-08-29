use askama::Template;
use axum::{
    Json,
    extract::Extension,
    response::{Html, IntoResponse},
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::controller::TutorController;
use crate::models::{
    AppError, CreateSessionRequest, CreateSessionResponse, SendQueryRequest, SendQueryResponse,
};

#[derive(Template)]
#[template(path = "tutor.html")]
pub struct TutorTemplate;

pub async fn root() -> impl IntoResponse {
    let template = TutorTemplate {};
    Html(template.render().unwrap_or_else(|e| {
        eprintln!("Error rendering template: {}", e);
        "Error rendering page".to_string()
    }))
}

pub async fn create_session(
    Extension(controller): Extension<Arc<Mutex<TutorController>>>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<CreateSessionResponse>, AppError> {
    let session_id = {
        let mut controller_guard = controller.lock().await;
        controller_guard.create_session(payload.student_id)?
    };

    Ok(Json(CreateSessionResponse {
        session_id,
        message: "Tutoring session created successfully".into(),
    }))
}

pub async fn send_query(
    Extension(controller): Extension<Arc<Mutex<TutorController>>>,
    Json(payload): Json<SendQueryRequest>,
) -> Result<Json<SendQueryResponse>, AppError> {
    let message = {
        let mut controller_guard = controller.lock().await;
        controller_guard
            .send_query(payload.student_id, payload.session_id, payload.query)
            .await?
    };

    Ok(Json(SendQueryResponse { message }))
}
