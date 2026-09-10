// Copyright (c) 2026 Edison Lepiten / AIEONYX

#[derive(Clone, Debug, PartialEq)]
pub struct LogEntry {
    pub index: u64,
    pub term: u64,
    pub command: Vec<u8>,
}

/// In-memory log for M1 scaffolding. TODO before M1 is done: back this with
/// the fjall `Backend` under a dedicated "raft_log" keyspace so entries
/// survive restart — reuse the same pattern as `src/backends/fjall.rs`
/// rather than a second storage engine.
pub struct RaftLog {
    entries: Vec<LogEntry>,
}

impl RaftLog {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn last_index(&self) -> u64 {
        self.entries.last().map(|e| e.index).unwrap_or(0)
    }

    pub fn last_term(&self) -> u64 {
        self.entries.last().map(|e| e.term).unwrap_or(0)
    }

    pub fn term_at(&self, index: u64) -> Option<u64> {
        self.entries.iter().find(|e| e.index == index).map(|e| e.term)
    }

    pub fn append(&mut self, entry: LogEntry) {
        self.entries.push(entry);
    }

    /// Raft log matching property: truncate any conflicting entries at and
    /// after `index` before appending new ones from a leader.
    pub fn truncate_from(&mut self, index: u64) {
        self.entries.retain(|e| e.index < index);
    }

    pub fn entries_from(&self, index: u64) -> Vec<LogEntry> {
        self.entries.iter().filter(|e| e.index >= index).cloned().collect()
    }
}
