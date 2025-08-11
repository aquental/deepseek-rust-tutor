mod session;
mod controller;
mod service;

use anyhow::Result;
use crate::controller::TutorController;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the TutorController
    let mut tutor_controller = TutorController::new();

    // Ensure a student session for testing
    let student_id = tutor_controller.ensure_student_session();
    println!("Student ID: {}", student_id);

    // Create a new tutoring session
    let session_response = tutor_controller.create_session();

    // Handle session creation response
    if session_response["status"] == "error" {
        println!("Error: {}", session_response["error"]["message"]);
        return Ok(());
    }

    // Extract session_id from the response
    let session_id = session_response["data"]["session_id"].as_str().unwrap();
    println!("Session created: {}", session_id);

    // Example query handling
    let student_query = "What are the key differences between mitosis and meiosis?";

    // Send the student query and get the response
    let query_response = tutor_controller.send_query(session_id, student_query).await;

    // Handle query response
    match query_response.get("status").and_then(|s| s.as_str()) {
        Some("error") => {
            if let Some(error) = query_response.get("error") {
                if let Some(message) = error.get("message") {
                    println!("Error: {}", message.as_str().unwrap_or("Unknown error"));
                }
            }
        }
        Some("success") => {
            if let Some(data) = query_response.get("data") {
                if let Some(response) = data.get("response") {
                    println!("Tutor's response: {}", response.as_str().unwrap_or("No response"));
                }
            }
        }
        _ => println!("Unexpected response format"),
    }

    Ok(())
}
