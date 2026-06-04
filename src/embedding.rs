//! EdisonDB Auto-embedding Pipeline
//!
//! Connects to a local Ollama instance to generate text embeddings.
//! Default model: nomic-embed-text (768-dim, runs fully locally).

use crate::EdisonError;

const DEFAULT_URL:   &str = "http://localhost:11434";
const DEFAULT_MODEL: &str = "nomic-embed-text";

/// HTTP client for local Ollama embedding generation.
#[derive(Debug, Clone)]
pub struct EmbeddingClient {
    base_url: String,
    model:    String,
}

impl EmbeddingClient {
    /// Create a new client with custom URL and model.
    pub fn new(base_url: &str, model: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            model:    model.to_string(),
        }
    }

    /// Create a client using default Ollama settings.
    pub fn default_ollama() -> Self {
        Self::new(DEFAULT_URL, DEFAULT_MODEL)
    }

    /// Generate an embedding for the given text.
    pub fn embed(&self, text: &str) -> Result<Vec<f32>, EdisonError> {
        let url = format!("{}/api/embeddings", self.base_url);
        let body = serde_json::json!({
            "model": self.model,
            "prompt": text,
        });
        let client = reqwest::blocking::Client::new();
        let resp = client
            .post(&url)
            .json(&body)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .map_err(|_| EdisonError::EmbeddingUnavailable)?;
        let json: serde_json::Value = resp
            .json()
            .map_err(|_| EdisonError::EmbeddingUnavailable)?;
        let embedding = json["embedding"]
            .as_array()
            .ok_or(EdisonError::EmbeddingUnavailable)?
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect();
        Ok(embedding)
    }

    /// Check if the embedding service is available.
    pub fn is_available(&self) -> bool {
        self.embed("ping").is_ok()
    }
}

impl Default for EmbeddingClient {
    fn default() -> Self {
        Self::default_ollama()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedding_client_unavailable() {
        let client = EmbeddingClient::new("http://localhost:19999", "nomic-embed-text");
        assert!(client.embed("test").is_err());
    }

    #[test]
    fn embedding_client_returns_vec() {
        let client = EmbeddingClient::default_ollama();
        if !client.is_available() {
            eprintln!("Skipping: Ollama not available");
            return;
        }
        let emb = client.embed("sovereign data").unwrap();
        assert!(!emb.is_empty());
        assert!(emb.len() > 100);
    }
}
