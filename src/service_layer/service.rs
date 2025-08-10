use std::{env, fs};
use uuid::Uuid;
use async_openai::{
    config::OpenAIConfig,
    Client
};
use async_openai::types::Role;
use crate::session::SessionManager;

#[allow(dead_code)]
pub struct TutorService {
    session_manager: SessionManager,
    client: Client<OpenAIConfig>,
    system_prompt: String,
}

impl TutorService {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let api_key = env::var("OPENAI_API_KEY").unwrap_or_default();
        let base_url = env::var("OPENAI_BASE_URL").unwrap_or_default();

        let config = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(base_url);
        let client = Client::with_config(config);

        let system_prompt = fs::read_to_string("data/system_prompt.txt")
            .unwrap_or_else(|e| {
                eprintln!("Error loading system prompt: {}", e);
                "You are a helpful tutor.".to_string()
            });

        Self {
            session_manager: SessionManager::new(),
            client,
            system_prompt,
        }
    }

    pub fn create_session(&mut self, student_id: &str) -> String {
        let session_id = Uuid::new_v4().to_string();
        self.session_manager
            .create_session(student_id, &session_id, &self.system_prompt);
        session_id
    }

    // Implement the process_query method
    pub fn process_query(
        &mut self,
        student_id: &str,
        session_id: &str,
        query: &str,
    ) -> Result<String, String> {
        // Check if the session exists
        let session = self
            .session_manager
            .get_session_mut(student_id, session_id)
            .ok_or_else(|| "Session not found".to_string())?;

        // Add the query to the session history
        self.session_manager
            .add_message(student_id, session_id, Role::User, query)
            .map_err(|e| e.to_string())?;

        // Return success message
        Ok("Query processed".to_string())
    }
}
