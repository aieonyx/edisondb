// Copyright (c) 2026 Edison Lepiten / AIEONYX

use rand::Rng;
use std::time::Duration;
use tokio::time::Instant;

/// Standard Raft election timeout range (150-300ms) to make split votes
/// statistically rare without needing a symmetry-breaking tiebreak scheme.
const MIN_TIMEOUT_MS: u64 = 150;
const MAX_TIMEOUT_MS: u64 = 300;

pub struct ElectionTimer {
    deadline: Instant,
}

impl ElectionTimer {
    pub fn new() -> Self {
        Self { deadline: Instant::now() + Self::random_timeout() }
    }

    fn random_timeout() -> Duration {
        let ms = rand::thread_rng().gen_range(MIN_TIMEOUT_MS..=MAX_TIMEOUT_MS);
        Duration::from_millis(ms)
    }

    pub fn reset(&mut self) {
        self.deadline = Instant::now() + Self::random_timeout();
    }

    pub fn expired(&self) -> bool {
        Instant::now() >= self.deadline
    }
}
