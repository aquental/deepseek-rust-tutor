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

    // Get API key and base URL
    let api_key =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable is not set");
    let base_url =
        env::var("OPENAI_BASE_URL").expect("OPENAI_BASE_URL environment variable is not set");

    // Build client config
    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_api_base(base_url);

    // Initialize the client
    let client = Client::with_config(config);

    // Define the prompt
    let prompt = "Describe a sunset over the ocean";

    // Build the user message
    let user_msg = ChatCompletionRequestUserMessageArgs::default()
        .content(prompt)
        .build()?
        .into();

    // Create the chat-completion request with only model and messages parameters
    let request = CreateChatCompletionRequestArgs::default()
        .model("deepseek-ai/DeepSeek-V3")
        .messages([user_msg])
        .max_tokens(100_u32) // limits the length of the explanation
        .temperature(0.2) // temperature parameter and set it to a low value
        .build()?;

    // Send and await response
    let response = client.chat().create(request).await?;

    // Extract and print the reply
    let reply = response.choices[0]
        .message
        .content
        .as_deref()
        .unwrap_or_default()
        .trim();

    println!("Query:  {}", prompt);
    println!("Answer: {}", reply);

    Ok(())
}
