pub mod consts {
    pub const MODEL: &str = "tinyllama";
}

pub mod gen {
    use std::error::Error;

    use futures::StreamExt;
    use ollama_rs::{
        generation::completion::{
            request::GenerationRequest, GenerationFinalResponseData, GenerationResponse,
        },
        Ollama,
    };
    use tokio::io::AsyncWriteExt;

    pub async fn generation_stream_print(
        ollama: &Ollama,
        generation_request: GenerationRequest,
    ) -> Result<Vec<GenerationFinalResponseData>, Box<dyn Error>> {
        let mut stream = ollama.generate_stream(generation_request).await?;

        let mut stdout = tokio::io::stdout();

        let mut final_data_responses = Vec::new();

        while let Some(response) = stream.next().await {
            let stream_responses: Vec<GenerationResponse> = response.unwrap();

            for stream_response in stream_responses {
                stdout.write_all(stream_response.response.as_bytes()).await?;
                stdout.flush().await?;

                if let Some(final_data) = stream_response.final_data {
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;

                    final_data_responses.push(final_data)
                }
            }
        }

        Ok(final_data_responses)
    }
}
