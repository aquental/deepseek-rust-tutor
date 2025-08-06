use anyhow::Result;
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
use std::env;
use tokio;

// Function to send a user message, update history, and return the assistant’s reply
async fn send_query(
    client: &Client<OpenAIConfig>,
    session: &mut Vec<ChatCompletionRequestMessage>,
    user_message: &str,
) -> Result<String> {
    // Append the new user message
    session.push(
        ChatCompletionRequestUserMessageArgs::default()
            .content(user_message)
            .build()?
            .into(),
    );

    // Create a chat completion request with the updated session history
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4.1-nano")
        .messages(session.to_vec())
        .build()?;

    // Send the request
    let response = client.chat().create(request).await?;

    // Extract the assistant’s reply
    let reply = response.choices[0]
        .message
        .content
        .as_deref()
        .unwrap_or_default()
        .trim()
        .to_string();

    // Append the assistant’s reply to the session history
    session.push(
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

    // Initialize session history with a system prompt
    let system_prompt = "You are a playful poet who loves to rhyme and create whimsical verses.";
    let mut session: Vec<ChatCompletionRequestMessage> = Vec::new();
    session.push(
        ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
            .build()?
            .into(),
    );

    // First query
    let reply = send_query(&client, &mut session, "What's your favorite type of pizza?").await?;
    println!("Response 1: {}", reply);

    // TODO: Add another user message to the conversation
    let second_reply = send_query(
        &client,
        &mut session,
        "Can you write a short poem about a starry night?",
    )
    .await?;
    // Request another response from the AI and print it
    println!("Response 2: {}", second_reply);

    Ok(())
}
