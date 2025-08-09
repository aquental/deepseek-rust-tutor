use anyhow::Result;
use async_openai::types::Role;
use std::fs;

mod session;
use session::SessionManager;

// Load the tutor system prompt from file, or fall back on a default
fn load_system_prompt(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|e| {
        eprintln!("Error loading system prompt: {}", e);
        "You are a helpful tutor.".to_string()
    })
}

fn main() -> Result<()> {
    // Load the system prompt
    let system_prompt = load_system_prompt("data/system_prompt.txt");

    // Initialize manager
    let mut manager = SessionManager::new();

    // Create a new session
    let student = "test_user";
    let session_id = "test_session";
    manager.create_session(student, session_id, system_prompt);

    // Define a vector of (Role, message) tuples representing a conversation
    let conversation = vec![
        (Role::User, "Hello, I need help with geometry."),
        (
            Role::Assistant,
            "Of course! What topic in geometry are you working on?",
        ),
        (Role::User, "Can you explain Pythagoras' theorem?"),
        (
            Role::Assistant,
            "Certainly! Pythagoras' theorem states that in a right-angled triangle, the square of the hypotenuse equals the sum of the squares of the other two sides.",
        ),
        (Role::User, "Can you give me an example?"),
        (
            Role::Assistant,
            "Sure! If one side is 3 and the other is 4, the hypotenuse is 5 because 3² + 4² = 9 + 16 = 25, and √25 = 5.",
        ),
    ];

    // TODO: Use a loop to add each message to the session
    for (role, content) in conversation {
        manager.add_message(student, session_id, role, content)?;
    }

    // TODO: Retrieve and print the full conversation
    let full_conversation = manager.get_conversation(student, session_id)?;
    for message in full_conversation {
        println!("{:?}", message);
    }

    Ok(())
}
