use anyhow::{Result, anyhow};
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};
use dotenv::dotenv;
use std::{collections::HashMap, env};
use tokio;
use uuid::Uuid;

// Create a new tutoring session and seed it with the system prompt
fn create_session(
    sessions: &mut HashMap<Uuid, Vec<ChatCompletionRequestMessage>>,
    system_prompt: &str,
) -> Result<Uuid> {
    let session_id = Uuid::new_v4();
    let system_msg = ChatCompletionRequestSystemMessageArgs::default()
        .content(system_prompt)
        .build()?
        .into();
    sessions.insert(session_id, vec![system_msg]);
    Ok(session_id)
}

// Send a user query in the given session, update history, and return the assistant’s reply
async fn send_query(
    client: &Client<OpenAIConfig>,
    sessions: &mut HashMap<Uuid, Vec<ChatCompletionRequestMessage>>,
    session_id: Uuid,
    user_message: &str,
) -> Result<String> {
    let history = sessions
        .get_mut(&session_id)
        .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;

    // Append the new user message
    history.push(
        ChatCompletionRequestUserMessageArgs::default()
            .content(user_message)
            .build()?
            .into(),
    );

    // Create a chat completion request with the updated session history
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4.1-nano")
        .messages(history.clone())
        .build()?;

    // Send the request
    let response = client.chat().create(request).await?;

    // Extract the assistant’s reply
    let reply = response
        .choices
        .first()
        .and_then(|c| c.message.content.as_deref())
        .unwrap_or_default()
        .trim()
        .to_string();

    // Append the assistant’s reply to the session history
    history.push(
        ChatCompletionRequestAssistantMessageArgs::default()
            .content(reply.as_str())
            .build()?
            .into(),
    );

    Ok(reply)
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
    let system_prompt = "You are an experienced and patient tutor specialized in math and science, providing clear and concise explanations.";

    // TODO: Create the first chat session and store its id
    let session_id1 = create_session(&mut sessions, system_prompt)?;
    // TODO: Send a first message in Session 1 and print the response
    let reply1_1 = send_query(
        &client,
        &mut sessions,
        session_id1,
        "Can you explain the concept of derivatives in calculus?",
    )
    .await?;
    println!("Session 1, First Query: {}", reply1_1);
    // TODO: Send a follow-up message in Session 1 and print the response
    let reply1_2 = send_query(
        &client,
        &mut sessions,
        session_id1,
        "Can you give an example of a derivative calculation?",
    )
    .await?;
    println!("Session 1, Follow-up Query: {}", reply1_2);

    // TODO: Create the second chat session and store its id
    let session_id2 = create_session(&mut sessions, system_prompt)?;
    // TODO: Send a first message in Session 2 and print the response
    let reply2_1 = send_query(
        &client,
        &mut sessions,
        session_id2,
        "What is the difference between a molecule and a compound?",
    )
    .await?;
    println!("Session 2, First Query: {}", reply2_1);
    // TODO: Send a follow-up message in Session 2 and print the response
    let reply2_2 = send_query(
        &client,
        &mut sessions,
        session_id2,
        "Can you provide an example of a molecule that is not a compound?",
    )
    .await?;
    println!("Session 2, Follow-up Query: {}", reply2_2);
    // TODO: Print both conversation histories to confirm they are separate
    println!("\nSession 1 Conversation History:");
    if let Some(history) = sessions.get(&session_id1) {
        for message in history {
            match message {
                ChatCompletionRequestMessage::System(msg) => println!("System: {:?}", msg.content),
                ChatCompletionRequestMessage::User(msg) => println!("User: {:?}", msg.content),
                ChatCompletionRequestMessage::Assistant(msg) => {
                    println!("Assistant: {:?}", msg.content)
                }
                _ => println!("Other message type"),
            }
        }
    }

    println!("\nSession 2 Conversation History:");
    if let Some(history) = sessions.get(&session_id2) {
        for message in history {
            match message {
                ChatCompletionRequestMessage::System(msg) => println!("System: {:?}", msg.content),
                ChatCompletionRequestMessage::User(msg) => println!("User: {:?}", msg.content),
                ChatCompletionRequestMessage::Assistant(msg) => {
                    println!("Assistant: {:?}", msg.content)
                }
                _ => println!("Other message type"),
            }
        }
    }

    Ok(())
}
