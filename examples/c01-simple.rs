use futures::StreamExt;
use tokio::io::AsyncWriteExt;
use xp_ollama::consts::MODEL;

use ollama_rs::{
    generation::completion::{request::GenerationRequest, GenerationFinalResponseData, GenerationResponse},
    Ollama,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // By default localhost:11434
    let ollama = Ollama::default();
    let model = MODEL.to_string();
    let prompt = "What is the best programming language? (Be concise)".to_string();

    let generation_request = GenerationRequest::new(model, prompt);

    // Single Response Generation
    let response = ollama.generate(generation_request).await?;
    println!("response: {}", response.response);

    Ok(())
}
