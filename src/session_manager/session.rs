use std::collections::HashMap;

#[allow(dead_code)]
pub struct SessionData {
    pub system_prompt: String,
    pub messages: Vec<String>,
}

#[allow(dead_code)]
pub struct SessionManager {
    sessions: HashMap<String, HashMap<String, SessionData>>,
}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager { sessions: HashMap::new() }
    }

    // Implement the create_session method
    pub fn create_session(&mut self, student_id: &str, session_id: &str, system_prompt: String) {
        // Ensure the student_id exists in the sessions map
        self.sessions.entry(student_id.to_string())
            .or_insert_with(HashMap::new)
            .insert(session_id.to_string(), SessionData {
                system_prompt,
                messages: Vec::new(),
            });
    }

    // Implement the get_session_mut method
    pub fn get_session_mut(&mut self, student_id: &str, session_id: &str) -> Option<&mut SessionData> {
        self.sessions
            .get_mut(student_id)
            .and_then(|student_sessions| student_sessions.get_mut(session_id))
    }
}
