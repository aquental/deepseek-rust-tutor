use axum::{Json, extract::Extension};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::controller::TutorController;
use crate::models::{AppError, CreateSessionRequest, CreateSessionResponse};

pub async fn root() -> &'static str {
    "Welcome to the Tutor API!"
}

pub async fn create_session(
    Extension(controller): Extension<Arc<Mutex<TutorController>>>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<CreateSessionResponse>, AppError> {
    let controller = controller.lock().await;
    let session_response = controller.create_session(payload).await?;
    Ok(Json(session_response))
}
