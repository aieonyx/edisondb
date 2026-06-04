#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tier {
    Critical,
    Personal,
    Noise,
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tier::Critical => write!(f, "CRITICAL"),
            Tier::Personal => write!(f, "PERSONAL"),
            Tier::Noise    => write!(f, "NOISE"),
        }
    }
}

// f32 does not implement Eq so Statement derives PartialEq only
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Write  { id: String, tier: Tier, payload: String, auto_embed: bool },
    Read   { id: String },
    List   { tier: Option<Tier> },
    Delete { id: String },
    Audit  { id: Option<String> },
    Embed  { id: String, embedding: Vec<f32> },
    Search { query: Vec<f32>, k: usize, min_similarity: Option<f32> },
    AutoEmbed { id: String },
}
