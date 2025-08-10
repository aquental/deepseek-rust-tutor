mod service;
mod session;

use anyhow::Result;
use service::TutorService;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tutor service
    let mut tutor_service = TutorService::new();

    // Simulate a student ID
    let student_id = "student123";

    // Create a new tutoring session
    let session_id = tutor_service.create_session(student_id);
    println!("Tutoring session created with ID: {}", session_id);

    // Simulate sending a tutoring query
    let student_query = "Can you explain the principles of supply and demand in economics?";
    println!("Student Query: {}", student_query);

    // Call process_query and handle any errors
    match tutor_service
        .process_query(student_id, &session_id, student_query)
        .await
    {
        Ok(tutor_response) => {
            println!("Tutor Response: {}", tutor_response);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    // Create a second tutoring session for the same student
    let second_session_id = tutor_service.create_session(student_id);
    println!(
        "Second tutoring session created with ID: {}",
        second_session_id
    );

    // Simulate sending a query in the second session
    let second_query = "Can you explain the concept of opportunity cost?";
    println!("Second Session Query: {}", second_query);

    // Process the query in the second session
    match tutor_service
        .process_query(student_id, &second_session_id, second_query)
        .await
    {
        Ok(tutor_response) => {
            println!("Second Session Tutor Response: {}", tutor_response);
        }
        Err(e) => {
            eprintln!("Error in second session query: {}", e);
        }
    }

    Ok(())
}
