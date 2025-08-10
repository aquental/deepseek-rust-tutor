use crate::session::SessionManager;
use async_openai::types::Role;
use async_openai::{Client, config::OpenAIConfig};
use std::{env, fs};
use uuid::Uuid;

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

        let system_prompt = fs::read_to_string("data/system_prompt.txt").unwrap_or_else(|e| {
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
    pub async fn process_query(
        &mut self,
        student_id: &str,
        session_id: &str,
        query: &str,
    ) -> Result<String> {
        // Retrieve the session using SessionManager; return error if not found
        let session = self
            .session_manager
            .get_session(student_id, session_id)
            .ok_or_else(|| {
                anyhow!(
                    "Session not found for student_id: {}, session_id: {}",
                    student_id,
                    session_id
                )
            })?;

        // Add the student's query to the session history
        self.session_manager
            .add_message(student_id, session_id, "user", query);

        // Retrieve the full conversation using SessionManager
        let conversation = self
            .session_manager
            .get_conversation(student_id, session_id)
            .ok_or_else(|| {
                anyhow!(
                    "Failed to retrieve conversation for student_id: {}, session_id: {}",
                    student_id,
                    session_id
                )
            })?;

        // Use the DeepSeek client to generate a response
        let request = CreateChatCompletionRequestArgs::default()
            .model("deepseek-ai/DeepSeek-V3")
            .messages(conversation)
            .temperature(0.7)
            .max_tokens(500)
            .build()?;

        // Make the API call and handle potential errors
        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| anyhow!("DeepSeek API error: {}", e))?;

        // Extract the AI's response from the DeepSeek client response
        let ai_response = response
            .choices
            .first()
            .ok_or_else(|| anyhow!("No response choices returned from DeepSeek API"))?
            .message
            .content
            .as_ref()
            .ok_or_else(|| anyhow!("No content in DeepSeek API response"))?
            .to_string();

        // Add the AI's response to the session history
        self.session_manager
            .add_message(student_id, session_id, "assistant", &ai_response);

        // Return the AI's response
        Ok(ai_response)
    }
}
