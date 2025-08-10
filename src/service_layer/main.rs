mod session;
mod service;

use service::TutorService;
use anyhow::Result;

fn main() -> Result<()> {
    // Initialize the tutor service
    let mut tutor_service = TutorService::new();

    // Simulate a student ID
    let student_id = "student123";

    // Create a new tutoring session
    let session_id = tutor_service.create_session(student_id);
    println!("Tutoring session created with ID: {}", session_id);

    // Simulate sending a query and handle potential errors
    let query = "What is the Pythagorean theorem?";
    match tutor_service.process_query(student_id, &session_id, query) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error processing query: {}", e),
    }
    
    Ok(())
}
