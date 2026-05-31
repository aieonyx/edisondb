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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Write  { id: String, tier: Tier, payload: String },
    Read   { id: String },
    List   { tier: Option<Tier> },
    Delete { id: String },
    Audit  { id: Option<String> },
}
