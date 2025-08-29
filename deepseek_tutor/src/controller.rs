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

    pub fn create_session(&mut self, student_id: String) -> Result<String, AppError> {
        if student_id.is_empty() {
            return Err(AppError::BadRequest("student_id cannot be empty".to_string()));
        }
        // Create the session
        let session_id = self.service.create_session(&student_id);
        Ok(session_id)
    }

    pub async fn send_query(
        &mut self,
        student_id: String,
        session_id: String,
        query: String,
    ) -> Result<String, AppError> {
        if student_id.is_empty() {
            return Err(AppError::BadRequest("student_id cannot be empty".to_string()));
        }

        if session_id.is_empty() || query.is_empty() {
            return Err(AppError::BadRequest("Missing session_id or query".to_string()));
        }

        self.service
            .process_query(&student_id, &session_id, &query)
            .await
            .map_err(|err| {
                if err.to_string().contains("Session not found") {
                    AppError::NotFound(err.to_string())
                } else {
                    AppError::Internal(err.to_string())
                }
            })
    }
}
