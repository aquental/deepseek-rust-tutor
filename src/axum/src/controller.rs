use crate::models::AppError;
use crate::service::TutorService;
use anyhow::Result;

pub struct TutorController {
    service: TutorService,
}

impl TutorController {
    pub fn new() -> Self {
        Self {
            service: TutorService::new(),
        }
    }

    /// Handle tutoring session creation request.
    pub fn create_session(&mut self, student_id: String) -> Result<String, AppError> {
        // Validate input
        if student_id.is_empty() {
            return Err(AppError::Unauthorized("Session expired".to_string()));
        }

        // Create the session
        let session_id = self.service.create_session(&student_id);
        Ok(session_id)
    }

    /// Handle student query processing request.
    pub async fn send_query(
        &mut self,
        student_id: String,
        session_id: String,
        query: String,
    ) -> Result<String, AppError> {
        // Validate inputs - check for empty strings
        if student_id.trim().is_empty() {
            return Err(AppError::BadRequest("Student ID cannot be empty".to_string()));
        }
        
        if session_id.trim().is_empty() {
            return Err(AppError::BadRequest("Session ID cannot be empty".to_string()));
        }
        
        if query.trim().is_empty() {
            return Err(AppError::BadRequest("Query cannot be empty".to_string()));
        }
        
        // Call the service to process the query and handle errors
        match self.service.process_query(&student_id, &session_id, &query).await {
            Ok(response) => Ok(response),
            Err(e) => {
                // Handle specific error cases
                if e.to_string().contains("Session not found") {
                    Err(AppError::NotFound(format!("Session not found for student {} with session {}", student_id, session_id)))
                } else {
                    Err(AppError::Internal(format!("Failed to process query: {}", e)))
                }
            }
        }
    }
}
