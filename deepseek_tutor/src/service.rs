use crate::session::SessionManager;
use anyhow::{Result, anyhow};
use async_openai::{Client, config::OpenAIConfig, types::CreateChatCompletionRequestArgs};
use std::{env, fs};
use uuid::Uuid;

pub struct TutorService {
    session_manager: SessionManager,
    client: Client<OpenAIConfig>,
    system_prompt: String,
}

impl TutorService {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
        let base_url = format!(
            "{}v1",
            env::var("OPENAI_BASE_URL").expect("OPENAI_BASE_URL not set")
        );

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

    pub async fn process_query(
        &mut self,
        student_id: &str,
        session_id: &str,
        query: &str,
    ) -> Result<String> {
        self.session_manager
            .get_session(student_id, session_id)
            .ok_or_else(|| anyhow!("Session not found"))?;

        self.session_manager
            .add_message(student_id, session_id, "user", query)?;

        let conversation = self
            .session_manager
            .get_conversation(student_id, session_id);

        let request = CreateChatCompletionRequestArgs::default()
            .model("deepseek-ai/DeepSeek-V3")
            .messages(conversation)
            .temperature(0.6)
            .max_tokens(500_u32)
            .build()?;
        let response = self.client.chat().create(request).await?;

        let tutor_response = response.choices[0]
            .message
            .content
            .as_deref()
            .unwrap_or_default()
            .trim()
            .to_string();

        self.session_manager
            .add_message(student_id, session_id, "assistant", &tutor_response)?;

        Ok(tutor_response)
    }
}
