use anyhow::Result;
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs},
};
use dotenv::dotenv;
use std::{collections::HashMap, env};
use tokio;
use uuid::Uuid;

// Create a new function to generate a unique chat session
fn create_new_session(
    sessions: &mut HashMap<Uuid, Vec<ChatCompletionRequestMessage>>,
    system_prompt: &str,
) -> Result<Uuid> {
    // Generate unique session identifier
    let session_id = Uuid::new_v4();

    // Create system prompt message
    let system_message = ChatCompletionRequestSystemMessageArgs::default()
        .content(system_prompt)
        .build()?;

    // Initialize empty conversation history with system prompt
    let conversation_history = vec![ChatCompletionRequestMessage::System(system_message)];

    // Insert into sessions HashMap
    sessions.insert(session_id, conversation_history);

    Ok(session_id)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")?;
    let base_url = env::var("OPENAI_BASE_URL")?;

    // Build OpenAI client
    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_api_base(base_url);
    let client = Client::with_config(config);

    // Shared in‑memory session store
    let mut sessions: HashMap<Uuid, Vec<ChatCompletionRequestMessage>> = HashMap::new();

    // System prompt for all sessions
    let system_prompt = "You are a friendly and efficient customer service attendant eager to assist customers with their inquiries and concerns.";

    // Create new session and print results
    let session_id = create_new_session(&mut sessions, system_prompt)?;
    println!("New session created with ID: {}", session_id);

    // Print initial conversation history
    if let Some(history) = sessions.get(&session_id) {
        println!("Initial conversation history:");
        for message in history {
            match message {
                ChatCompletionRequestMessage::System(msg) => {
                    // Convert content to string for printing
                    let content = match &msg.content {
                        async_openai::types::ChatCompletionRequestSystemMessageContent::Text(
                            text,
                        ) => text,
                        async_openai::types::ChatCompletionRequestSystemMessageContent::Array(
                            parts,
                        ) => {
                            // Handle array of content parts if needed
                            &parts.iter()
                                .filter_map(|part| match part {
                                    async_openai::types::ChatCompletionRequestSystemMessageContentPart::Text(t) => Some(t.text.as_str()),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                                .join(" ")
                        }
                    };
                    println!("System: {}", content);
                }
                _ => println!("Unexpected message type"),
            }
        }
    }

    Ok(())
}
