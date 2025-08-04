use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    types::{ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};
use dotenv::dotenv;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get environment variables
    let api_key = env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY environment variable is not set");
    let base_url = env::var("OPENAI_BASE_URL")
        .expect("OPENAI_BASE_URL environment variable is not set");

    // Create config with explicit values
    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_api_base(base_url);

    // Initialize the OpenAI client with config
    let client = Client::with_config(config);

    // Define the prompt
    let prompt = "What are the main advantages of using Rust for web development?";

    // Build a user message from the prompt
    let user_message = ChatCompletionRequestUserMessageArgs::default()
        .content(prompt)
        .build()?;
    let message: ChatCompletionRequestMessage = user_message.into();

    // Create a chat completion request
    let request = CreateChatCompletionRequestArgs::default()
        .model("deepseek-ai/DeepSeek-V3")
        .messages(vec![message])
        .build()?;

    // Send the request and await response
    let response = client.chat().create(request).await?;

    // Display the query and answer
    println!("Prompt: {}", prompt);
    if let Some(choice) = response.choices.first() {
        println!(
            "Response: {}",
            choice.message.content.as_deref().unwrap_or_default()
        );
    } else {
        println!("Response: No choices returned");
    }

    Ok(())
}
