use std::{error::Error, io::stdout};

use futures::StreamExt;
use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, MessageRole},
    Ollama,
};
use tokio::io::AsyncWriteExt;
use xp_ollama::consts::MODEL;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ollama = Ollama::default();

    let prompts = &[
        "What is the best programming language? (be concise)",
        "Why the sky is red?",
        "What was my last question?",
    ];

    let system_message = ChatMessage::new(
        MessageRole::System,
        "The best programming language is Rust".to_string(),
    );

    let mut thread_messages: Vec<ChatMessage> = vec![system_message];

    for prompt in prompts {
        println!("\nprompt: {}", prompt);

        let prompt_message = ChatMessage::new(MessageRole::User, prompt.to_string());

        thread_messages.push(prompt_message);

        let chat_request = ChatMessageRequest::new(MODEL.to_string(), thread_messages.clone());

        let message_content = run_chat_request(&ollama, chat_request).await?;

        if let Some(content) = message_content {
            let assistant_message = ChatMessage::new(MessageRole::Assistant, content.to_string());

            thread_messages.push(assistant_message);
        }
    }

    Ok(())
}

async fn run_chat_request(
    ollama: &Ollama,
    chat_request: ChatMessageRequest,
) -> Result<Option<String>, Box<dyn Error>> {
    let mut stream = ollama.send_chat_messages_stream(chat_request).await?;

    let mut stdout = tokio::io::stdout();
    let mut current_assistant_messages: Vec<String> = Vec::new();

    while let Some(response) = stream.next().await {
        let chat_response = response.map_err(|_| "stream.next error")?;

        if let Some(message) = chat_response.message {
            let message_content = message.content;

            stdout.write_all(message_content.as_bytes()).await?;
            stdout.flush().await?;

            current_assistant_messages.push(message_content);
        }

        if let Some(_final_response) = chat_response.final_data {
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;

            let assistant_content = current_assistant_messages.join("");
            return Ok(Some(assistant_content));
        }
    }

    stdout.write_all(b"\n").await?;
    stdout.flush().await?;
        
    Ok(None)
}
