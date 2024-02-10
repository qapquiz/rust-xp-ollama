use std::error::Error;

use ollama_rs::{
    generation::chat::{ChatMessage, MessageRole},
    Ollama,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ollama = Ollama::default();

    let prompts = &[
        "What is the best programming language?",
        "What is the second best language?",
        "What was my last question?",
    ];

    let system_message = ChatMessage::new(
        MessageRole::System,
        "The best programming language is Rust".to_string(),
    );
    let mut thread_messages: Vec<ChatMessage> = vec![system_message];

    Ok(())
}
