use futures::StreamExt;
use simple_fs::{ensure_dir, ensure_file_dir, save_json};
use tokio::io::AsyncWriteExt;
use xp_ollama::consts::MODEL;
use xp_ollama::gen::generation_stream_print;

use ollama_rs::{
    generation::completion::{
        request::GenerationRequest, GenerationContext, GenerationFinalResponseData,
        GenerationResponse,
    },
    Ollama,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // By default localhost:11434
    let ollama = Ollama::default();
    let model = MODEL.to_string();
    let prompts = &["Why the sky is red?", "What was my first question?"];

    let mut last_contexts: Option<Vec<GenerationContext>> = None;

    for prompt in prompts {
        println!("prompt >> {}", prompt);
        let mut generation_request = GenerationRequest::new(model.to_owned(), prompt.to_string());

        if let Some(last_contexts) = last_contexts.take() {
            for last_context in last_contexts {
                generation_request = generation_request.context(last_context);
            }
        }

        let final_data = generation_stream_print(&ollama, generation_request).await;

        if let Ok(final_data) = final_data {
            let contexts: Vec<GenerationContext> =
                final_data.into_iter().map(|data| data.context).collect();
            last_contexts = Some(contexts);
        }
    }

    if let Some(last_contexts) = last_contexts {
        let ctx_file_path = ".c02-data/ctx.json";
        ensure_file_dir(ctx_file_path)?;
        save_json(ctx_file_path, &last_contexts)?;
    }

    // Single Response Generation
    // generation_stream_print(&ollama, generation_request).await?;
    // let response = ollama.generate(generation_request).await?;
    // println!("response: {}", response.response);

    Ok(())
}
