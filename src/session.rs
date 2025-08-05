use anyhow::Result;
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestAssistantMessageContent,
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageContent,
        ChatCompletionRequestUserMessageArgs, ChatCompletionRequestUserMessageContent,
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
    let mut session = vec![
        ChatCompletionRequestUserMessageArgs::default()
            .content("Hello! I'm learning Rust and would like to ask you some questions about the language.")
            .build()?
            .into(),
    ];

    // Send first query and print answer
    let answer = send_query(&client, &session).await?;
    println!("Assistant: {}", answer);

    // Append the assistant's response to session history
    session.push(
        ChatCompletionRequestAssistantMessageArgs::default()
            .content(answer)
            .build()?
            .into(),
    );

    // Append a follow-up question to session history
    session.push(
        ChatCompletionRequestUserMessageArgs::default()
            .content("Can you tell me another fun fact about the same topic?")
            .build()?
            .into(),
    );

    // Call send_query with the updated session history and print the response
    let follow_up_answer = send_query(&client, &session).await?;
    println!("Assistant (follow-up): {}", follow_up_answer);

    println!("\nConversation History:");
    for message in &session {
        match message {
            ChatCompletionRequestMessage::User(user_msg) => {
                if let ChatCompletionRequestUserMessageContent::Text(content) = &user_msg.content {
                    println!(">>User: {}", content);
                }
            }
            ChatCompletionRequestMessage::Assistant(assistant_msg) => {
                if let Some(ChatCompletionRequestAssistantMessageContent::Text(content)) =
                    &assistant_msg.content
                {
                    println!(">>Assistant: {}", content);
                }
            }
            ChatCompletionRequestMessage::System(system_msg) => {
                if let ChatCompletionRequestSystemMessageContent::Text(content) =
                    &system_msg.content
                {
                    println!(">>System: {}", content);
                }
            }
            _ => println!("Unknown message type"),
        }
    }

    Ok(())
}
