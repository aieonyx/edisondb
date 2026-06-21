// Copyright (c) 2026 Edison Lepiten / AIEONYX
// EdisonDB P3-M4 — Sovereign offline embedder
// Pure Rust, zero external dependencies, zero network calls.
//
// Algorithm: weighted bag-of-words with sovereign hash projection
// - Tokenize text into unigrams and bigrams
// - Project each token to a 128-dim unit vector via deterministic hash
// - Accumulate with TF weighting
// - L2-normalize the result
//
// Properties:
// - Deterministic: same text always produces same vector
// - Offline: no network, no model files, no Ollama
// - Consistent: dimension matches HNSW index (configurable)
// - Sovereign: no external crate dependency

pub const EMBED_DIM: usize = 128;

/// Sovereign offline embedder — pure Rust, no external deps.
#[derive(Debug, Clone)]
pub struct SovereignEmbedder {
    pub dim: usize,
    /// Seed for the hash projection (allows different embedding spaces)
    pub seed: u64,
}

impl SovereignEmbedder {
    pub fn new() -> Self {
        Self { dim: EMBED_DIM, seed: 0x4149454f4e595800 } // "AIEONYX\0" as u64
    }

    pub fn with_dim(dim: usize) -> Self {
        Self { dim, seed: 0x4149454f4e595800 }
    }

    /// Embed text into a fixed-dimension f32 vector.
    pub fn embed(&self, text: &str) -> Vec<f32> {
        let mut acc = vec![0.0f32; self.dim];
        let tokens = tokenize(text);
        let n = tokens.len();
        if n == 0 {
            return acc;
        }

        // Unigrams with TF weight
        for (i, tok) in tokens.iter().enumerate() {
            let tf = 1.0 + (1.0 / (i as f32 + 1.0)); // position-weighted TF
            let proj = hash_project(tok, self.dim, self.seed);
            for (a, p) in acc.iter_mut().zip(proj.iter()) {
                *a += p * tf;
            }
        }

        // Bigrams with half weight (capture co-occurrence)
        for i in 0..n.saturating_sub(1) {
            let bigram = format!("{} {}", tokens[i], tokens[i + 1]);
            let proj = hash_project(&bigram, self.dim, self.seed);
            for (a, p) in acc.iter_mut().zip(proj.iter()) {
                *a += p * 0.5;
            }
        }

        l2_normalize(&mut acc);
        acc
    }

    /// Cosine similarity between two embeddings.
    pub fn similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() { return 0.0; }
        // Both are L2-normalized, so dot product = cosine similarity
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }

    pub fn is_available(&self) -> bool { true } // always available offline
}

impl Default for SovereignEmbedder {
    fn default() -> Self { Self::new() }
}

// ── Tokenizer ─────────────────────────────────────────────────────────────────

/// Tokenize text: lowercase, split on non-alphanumeric, filter short tokens.
fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| s.len() >= 2)
        .map(|s| s.to_string())
        .collect()
}

// ── Hash projection ───────────────────────────────────────────────────────────

/// Project a token string to a unit vector in R^dim via deterministic hashing.
/// Uses FNV-1a variant seeded with sovereign seed + dimension index.
fn hash_project(token: &str, dim: usize, seed: u64) -> Vec<f32> {
    let mut proj = vec![0.0f32; dim];
    for d in 0..dim {
        // Hash: FNV-1a of (seed XOR d XOR token_bytes)
        let mut h: u64 = 0xcbf29ce484222325u64 ^ seed ^ (d as u64).wrapping_mul(0x9e3779b97f4a7c15_u64);
        for &b in token.as_bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x00000100000001b3u64);
        }
        h ^= d as u64;
        h = h.wrapping_mul(0x00000100000001b3u64);
        // Map to [-1, 1] via sign bit + magnitude
        let sign = if h & 1 == 0 { 1.0f32 } else { -1.0f32 };
        let mag = ((h >> 1) & 0xFFFF) as f32 / 65535.0;
        proj[d] = sign * (0.3 + 0.7 * mag); // bias away from zero
    }
    l2_normalize(&mut proj);
    proj
}

/// L2-normalize a vector in place. No-op if norm is ~0.
fn l2_normalize(v: &mut Vec<f32>) {
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 1e-8 {
        for x in v.iter_mut() { *x /= norm; }
    }
}

// ── EmbeddingBackend enum ─────────────────────────────────────────────────────

use crate::embedding::EmbeddingClient;
use crate::EdisonError;

/// Unified embedding backend — Ollama (online) or Sovereign (offline).
#[derive(Debug, Clone)]
pub enum EmbeddingBackend {
    Ollama(EmbeddingClient),
    Sovereign(SovereignEmbedder),
}

impl EmbeddingBackend {
    /// Auto-select: use Ollama if available, fall back to Sovereign.
    pub fn auto() -> Self {
        let client = EmbeddingClient::default_ollama();
        if client.is_available() {
            Self::Ollama(client)
        } else {
            Self::Sovereign(SovereignEmbedder::new())
        }
    }

    pub fn sovereign() -> Self {
        Self::Sovereign(SovereignEmbedder::new())
    }

    pub fn ollama(url: &str, model: &str) -> Self {
        Self::Ollama(EmbeddingClient::new(url, model))
    }

    /// Generate embedding for text.
    pub fn embed(&self, text: &str) -> Result<Vec<f32>, EdisonError> {
        match self {
            Self::Ollama(c)     => c.embed(text),
            Self::Sovereign(s)  => Ok(s.embed(text)),
        }
    }

    pub fn is_available(&self) -> bool {
        match self {
            Self::Ollama(c)    => c.is_available(),
            Self::Sovereign(s) => s.is_available(),
        }
    }

    pub fn dim(&self) -> usize {
        match self {
            Self::Ollama(_)    => 768, // nomic-embed-text
            Self::Sovereign(s) => s.dim,
        }
    }

    pub fn backend_name(&self) -> &'static str {
        match self {
            Self::Ollama(_)    => "ollama",
            Self::Sovereign(_) => "sovereign",
        }
    }
}
