use anyhow::Result;
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};
use dotenv::dotenv;
use std::env;
use tokio;

// Function to send the full session history and get the assistant’s reply
async fn send_query(
    client: &Client<OpenAIConfig>,
    session: &[ChatCompletionRequestMessage],
) -> Result<String> {
    // Create a chat completion request with the session history
    let request = CreateChatCompletionRequestArgs::default()
        .model("deepseek-ai/DeepSeek-V3")
        .messages(session.to_vec())
        .build()?;

    // Send the request
    let response = client.chat().create(request).await?;

    // Extract the assistant’s reply from the response
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
    // Load environment variables from .env
    dotenv().ok();

    // Read API key and base URL
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    // Build OpenAI client configuration
    let mut config = OpenAIConfig::new().with_api_key(api_key);

    // Add base URL if provided
    if let Ok(base_url) = env::var("OPENAI_BASE_URL") {
        if base_url != "xxx" && !base_url.is_empty() {
            config = config.with_api_base(base_url);
        }
    }

    // Initialize the OpenAI client
    let client = Client::with_config(config);

    // TODO: Start session with an initial user message
    // Add your initial user message here
    let session = vec![
        ChatCompletionRequestUserMessageArgs::default()
            .content("Hello! I'm learning Rust and would like to ask you some questions about the language.")
            .build()?
            .into(),
    ];

    // Send first query and print answer
    let answer = send_query(&client, &session).await?;
    println!("Assistant: {}", answer);

    Ok(())
}
