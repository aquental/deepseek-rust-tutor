use axum::{Json, extract::Extension};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::controller::TutorController;
use crate::models::{
    AppError,
    CreateSessionRequest,
    CreateSessionResponse,
    SendQueryRequest,
    SendQueryResponse,
};

pub async fn root() -> &'static str {
    "Welcome to the Tutor API!"
}

pub async fn create_session(
    Extension(controller): Extension<Arc<Mutex<TutorController>>>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<CreateSessionResponse>, AppError> {
    // Lock the controller
    let mut controller = controller.lock().await;
    
    // Call create_session on the controller
    let session_id = controller.create_session(payload.student_id)?;
    
    // Return the response
    Ok(Json(CreateSessionResponse {
        session_id,
        message: "Session created successfully".to_string(),
    }))
}

pub async fn send_query(
    Extension(controller): Extension<Arc<Mutex<TutorController>>>,
    Json(payload): Json<SendQueryRequest>,
) -> Result<Json<SendQueryResponse>, AppError> {
    // Lock the controller
    let mut controller = controller.lock().await;
    
    // Call the controller's send_query method with the appropriate fields from the payload
    let response_message = controller.send_query(
        payload.student_id,
        payload.session_id,
        payload.query,
    ).await?;
    
    // Return a Json-wrapped SendQueryResponse
    Ok(Json(SendQueryResponse {
        message: response_message,
    }))
}
