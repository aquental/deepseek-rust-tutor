use std::fs;

mod session;
use session::SessionManager;

fn load_system_prompt(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|e| {
        eprintln!("Error loading system prompt: {}", e);
        "You are a helpful tutor.".to_string()
    })
}

fn main() {
    // Load the system prompt
    let system_prompt = load_system_prompt("data/system_prompt.txt");

    // TODO: Instantiate the SessionManager
    let mut session_manager = SessionManager::new();

    // TODO: Define student_id and session_id variables
    // - Set student_id to a test value, e.g., "test_student"
    // - Set session_id to a test value, e.g., "test_session"
    let student_id = "test_student";
    let session_id = "test_session";

    // TODO: Use create_session method to create a new session
    // - Call the create_session method on the SessionManager instance
    // - Pass student_id, session_id, and system_prompt as arguments
    session_manager.create_session(student_id, session_id, system_prompt);

    // TODO: Use get_session_mut method to check if the session exists
    // - Retrieve the session using the get_session_mut method with student_id and session_id
    // - If the session is found, print "Session successfully created!"
    // - If the session is not found, print "Failed to create session."
    if session_manager
        .get_session_mut(student_id, session_id)
        .is_some()
    {
        println!("Session successfully created!");
    } else {
        println!("Failed to create session.");
    }
}
