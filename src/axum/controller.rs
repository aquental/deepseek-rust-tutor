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
        // TODO: Validate the student_id (return an Unauthorized error if empty)
        // TODO: Call the service to create a session and return the session_id
        todo!()
    }
}
