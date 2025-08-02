use anyhow::Result;
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
};
use dotenv::dotenv;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get environment variables
    let api_key =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable is not set");
    let base_url =
        env::var("OPENAI_BASE_URL").expect("OPENAI_BASE_URL environment variable is not set");

    // Create config with explicit values
    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_api_base(base_url);

    // Initialize the OpenAI client with config
    let client = Client::with_config(config);

    // TODO: Change the prompt to ask for a fun fact instead of a joke
    let prompt = "Can you tell me a joke?";

    // Build a user message from the prompt
    let user_msg = ChatCompletionRequestUserMessageArgs::default()
        .content(prompt)
        .build()?
        .into();

    // Create a chat completion request
    let request = CreateChatCompletionRequestArgs::default()
        .model("deepseek-ai/DeepSeek-V3")
        .messages([user_msg])
        .build()?;

    // Send the request and await response
    let response = client.chat().create(request).await?;

    // Display the query and answer
    println!("Query: {}", prompt);
    println!(
        "Answer: {}",
        response.choices[0]
            .message
            .content
            .as_deref()
            .unwrap_or_default()
    );

    Ok(())
}
