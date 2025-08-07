use anyhow::Result;
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
};
use dotenv::dotenv;
use std::{env, fs};
use tokio;

// Define a function to load the system prompt from a file
fn load_system_prompt(file_path: &str) -> String {
    // Try to read and return the file contents as a String
    match fs::read_to_string(file_path) {
        Ok(content) => content.trim().to_string(),
        // If an error occurs:
        Err(e) => {
            // Print the error message
            eprintln!("Error reading system prompt file: {}", e);
            // Return a default prompt
            "You are a helpful assistant.".to_string()
        }
    }
}
// Sends the full session history and returns the assistant’s reply
async fn send_query(
    client: &Client<OpenAIConfig>,
    session: &[ChatCompletionRequestMessage],
) -> Result<String> {
    let request = CreateChatCompletionRequestArgs::default()
        .model("deepseek-ai/DeepSeek-V3")
        .messages(session.to_vec())
        .build()?;

    let response = client.chat().create(request).await?;

    let reply = response.choices[0]
        .message
        .content
        .as_deref()
        .unwrap_or_default()
        .trim();

    Ok(reply.to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env and read config
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let base_url = env::var("OPENAI_BASE_URL").expect("OPENAI_BASE_URL not set");
    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_api_base(base_url);
    let client = Client::with_config(config);

    // TODO: Load the system prompt by calling your function with "data/system_prompt.txt"
    let system_prompt = load_system_prompt("data/system_prompt.txt");

    // TODO: Build the initial session using the loaded system prompt
    let mut session = Vec::new();
    session.push(
        ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
            .build()?
            .into(),
    );

    let user_question = "Who am I speaking with?";

    session.push(
        ChatCompletionRequestUserMessageArgs::default()
            .content(user_question)
            .build()?
            .into(),
    );

    // Send query and get response
    let response = send_query(&client, &session).await?;

    // Print the user's question and the chatbot's response
    println!("User:\n{}\n", user_question);
    println!("Chatbot:\n{}", response);

    Ok(())
}
