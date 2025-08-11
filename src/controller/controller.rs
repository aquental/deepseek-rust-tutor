use crate::service::TutorService;
use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

pub struct TutorController {
    pub tutor_service: TutorService,
    pub test_session: HashMap<String, String>,
}

impl TutorController {
    pub fn new() -> Self {
        let mut tc = TutorController {
            tutor_service: TutorService::new(),
            test_session: HashMap::new(),
        };
        // ensure we have a student_id right away
        tc.ensure_student_session();
        tc
    }

    /// Ensure the student has a session ID in the test_session map.
    pub fn ensure_student_session(&mut self) -> String {
        self.test_session
            .entry("student_id".to_string())
            .or_insert_with(|| Uuid::new_v4().to_string())
            .clone()
    }

    /// Handle tutoring session creation request.
    pub fn create_session(&mut self) -> Value {
        let student_id = match self.test_session.get("student_id") {
            Some(id) => id.clone(),
            None => return Self::session_expired(),
        };

        let session_id = self.tutor_service.create_session(&student_id);
        Self::success_response(json!({
            "session_id": session_id,
            "message": "Tutoring session created successfully"
        }))
    }

    pub async fn send_query(&mut self, session_id: &str, student_query: &str) -> Value {
        // Retrieve student_id from test_session map
        let student_id = match self.test_session.get("student_id") {
            None => return Self::session_expired(),
            Some(id) => id.clone(),
        };

        // Validate session_id and student_query
        if session_id.is_empty() {
            return Self::error_response("Session ID is required", 400);
        }
        if student_query.is_empty() {
            return Self::error_response("Student query is required", 400);
        }

        // Process query using tutor_service
        match self
            .tutor_service
            .process_query(&student_id, session_id, student_query)
            .await
        {
            Ok(tutor_response) => Self::success_response(json!({
                "session_id": session_id,
                "response": tutor_response,
                "message": "Query processed successfully"
            })),
            Err(err) => match err.to_string().as_str() {
                "Session not found" => Self::error_response("Session not found", 404),
                _ => Self::error_response("Internal server error", 500),
            },
        }
    }

    // Helper methods for standardized responses
    fn success_response<T: serde::Serialize>(data: T) -> Value {
        json!({
            "status": "success",
            "data": data
        })
    }

    fn error_response(message: impl AsRef<str>, code: u16) -> Value {
        json!({
            "status": "error",
            "error": { "message": message.as_ref(), "code": code }
        })
    }

    fn session_expired() -> Value {
        Self::error_response("Session expired", 401)
    }
}
