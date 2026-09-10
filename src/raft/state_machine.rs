// Copyright (c) 2026 Edison Lepiten / AIEONYX

use super::{ClusterConfig, RaftLog};
use crate::backends::StorageBackend;
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Follower,
    Candidate,
    Leader,
}

pub struct RaftNode {
    pub role: Role,
    pub config: ClusterConfig,

    // Persistent state (must survive restart — written via `backend` before
    // responding to any RPC that changes them, per Raft §5.1/§5.2).
    pub current_term: u64,
    pub voted_for: Option<String>,
    pub log: RaftLog,

    // Volatile state
    pub commit_index: u64,
    pub last_applied: u64,

    // Leader-only volatile state (reinitialized after election)
    pub next_index: std::collections::HashMap<String, u64>,
    pub match_index: std::collections::HashMap<String, u64>,

    // Wrapped in Mutex so RaftNode is Sync: StorageBackend guarantees
    // Send but not Sync, and tonic handlers require a Sync service.
    // Lock is only ever held for a synchronous persist — never across
    // an await point.
    backend: Mutex<Box<dyn StorageBackend>>,
}

impl RaftNode {
    pub fn new(config: ClusterConfig, backend: Box<dyn StorageBackend>) -> Self {
        // TODO M1: restore current_term / voted_for / log from backend on
        // startup instead of defaulting — required for crash safety.
        Self {
            role: Role::Follower,
            config,
            current_term: 0,
            voted_for: None,
            log: RaftLog::new(),
            commit_index: 0,
            last_applied: 0,
            next_index: Default::default(),
            match_index: Default::default(),
            backend: Mutex::new(backend),
        }
    }

    /// Election safety invariant: at most one leader per term.
    /// Mirrors the invariant style in `verification.rs` — candidate for a
    /// Kani harness once this stabilizes (see `edisondb-fv` FV track).
    pub fn invariant_single_leader_per_term(&self, other_leader_term: u64) -> bool {
        !(self.role == Role::Leader && other_leader_term == self.current_term)
    }

    pub fn become_follower(&mut self, term: u64) {
        self.current_term = term;
        self.voted_for = None;
        self.role = Role::Follower;
        // TODO: persist term + voted_for reset via self.backend before returning
    }

    pub fn become_candidate(&mut self) {
        self.current_term += 1;
        self.voted_for = Some(self.config.node_id.clone());
        self.role = Role::Candidate;
        // TODO: persist term + vote via self.backend before sending RequestVote
    }

    pub fn become_leader(&mut self) {
        self.role = Role::Leader;
        let next = self.log.last_index() + 1;
        for peer in &self.config.peers {
            self.next_index.insert(peer.clone(), next);
            self.match_index.insert(peer.clone(), 0);
        }
    }
}
