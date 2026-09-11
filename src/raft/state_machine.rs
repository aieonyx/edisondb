// Copyright (c) 2026 Edison Lepiten / AIEONYX

use super::{ClusterConfig, LogEntry, RaftLog};
use crate::backends::StorageBackend;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Follower,
    Candidate,
    Leader,
}

// ── Handler outcomes ─────────────────────────────────────────────────────────
// Returned by the pure decision functions. `persist_required` tells the
// owning RaftNode that durable state changed and must be flushed BEFORE
// the RPC is acked (Raft §5.1 - persistent state).

#[derive(Debug, PartialEq, Eq)]
pub struct VoteOutcome {
    pub term: u64,
    pub vote_granted: bool,
    pub persist_required: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AppendOutcome {
    pub term: u64,
    pub success: bool,
    pub match_index: u64,
    pub persist_required: bool,
    pub reset_election_timer: bool,
}

// ── RaftState ────────────────────────────────────────────────────────────────
// Pure consensus state: no backend, no async, no I/O. All Raft decision
// logic lives here so it can be unit-tested synchronously and verified
// with Kani (see `edisondb-fv`).

pub struct RaftState {
    pub role: Role,
    pub node_id: String,

    // Persistent state (§5.1) — must be durable before acking any RPC
    // that mutates it.
    pub current_term: u64,
    pub voted_for: Option<String>,
    pub log: RaftLog,

    // Volatile state
    pub commit_index: u64,
    pub last_applied: u64,

    // Leader-only volatile state (reinitialized after election)
    pub next_index: HashMap<String, u64>,
    pub match_index: HashMap<String, u64>,
}

impl RaftState {
    pub fn new(node_id: String) -> Self {
        Self {
            role: Role::Follower,
            node_id,
            current_term: 0,
            voted_for: None,
            log: RaftLog::new(),
            commit_index: 0,
            last_applied: 0,
            next_index: HashMap::new(),
            match_index: HashMap::new(),
        }
    }

    /// §5.1 — advancing the term is the ONLY event that clears `voted_for`.
    /// Stepping down within a term must preserve the vote already cast, or
    /// a node could vote twice in one term and elect two leaders.
    pub fn advance_term(&mut self, term: u64) {
        debug_assert!(term > self.current_term, "advance_term must strictly increase the term");
        self.current_term = term;
        self.voted_for = None;
        self.role = Role::Follower;
    }

    /// Step down to Follower without touching the term or the vote.
    pub fn step_down(&mut self) {
        self.role = Role::Follower;
    }

    /// §5.1 — on observing a higher term from any RPC, advance and step down.
    /// Returns true if the term actually moved (i.e. persistence needed).
    pub fn observe_term(&mut self, term: u64) -> bool {
        if term > self.current_term {
            self.advance_term(term);
            true
        } else {
            false
        }
    }

    pub fn become_candidate(&mut self) {
        self.current_term += 1;
        self.voted_for = Some(self.node_id.clone());
        self.role = Role::Candidate;
    }

    pub fn become_leader(&mut self, peers: &[String]) {
        self.role = Role::Leader;
        let next = self.log.last_index() + 1;
        for peer in peers {
            self.next_index.insert(peer.clone(), next);
            self.match_index.insert(peer.clone(), 0);
        }
    }

    /// Election safety: at most one leader per term.
    pub fn invariant_single_leader_per_term(&self, other_leader_term: u64) -> bool {
        !(self.role == Role::Leader && other_leader_term == self.current_term)
    }

    /// §5.4.1 — is the candidate's log at least as up-to-date as ours?
    /// This check is what preserves Leader Completeness.
    fn candidate_log_is_current(&self, last_log_index: u64, last_log_term: u64) -> bool {
        let our_term = self.log.last_term();
        let our_index = self.log.last_index();
        last_log_term > our_term || (last_log_term == our_term && last_log_index >= our_index)
    }

    /// §5.2 + §5.4.1 — RequestVote decision.
    pub fn handle_request_vote(
        &mut self,
        term: u64,
        candidate_id: &str,
        last_log_index: u64,
        last_log_term: u64,
    ) -> VoteOutcome {
        // §5.1 — reject a stale term outright.
        if term < self.current_term {
            return VoteOutcome {
                term: self.current_term,
                vote_granted: false,
                persist_required: false,
            };
        }

        let mut persist_required = self.observe_term(term);

        // §5.2 — at most one vote per term; re-granting to the same
        // candidate keeps retries idempotent.
        let can_vote = match &self.voted_for {
            None => true,
            Some(id) => id == candidate_id,
        };

        let vote_granted = can_vote && self.candidate_log_is_current(last_log_index, last_log_term);

        if vote_granted && self.voted_for.is_none() {
            self.voted_for = Some(candidate_id.to_string());
            persist_required = true;
        }

        VoteOutcome {
            term: self.current_term,
            vote_granted,
            persist_required,
        }
    }

    /// §5.3 — AppendEntries decision (also the heartbeat path when
    /// `entries` is empty).
    pub fn handle_append_entries(
        &mut self,
        term: u64,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: &[LogEntry],
        leader_commit: u64,
    ) -> AppendOutcome {
        // §5.1 — reject a stale-term leader; do not reset our timer for it.
        if term < self.current_term {
            return AppendOutcome {
                term: self.current_term,
                success: false,
                match_index: 0,
                persist_required: false,
                reset_election_timer: false,
            };
        }

        // Legitimate leader for this term or newer.
        let mut persist_required = self.observe_term(term);
        if self.role != Role::Follower {
            // Same-term step-down: preserves `voted_for` deliberately.
            self.step_down();
        }

        // §5.3 — consistency check at prev_log_index.
        if prev_log_index > 0 {
            let matches = matches!(self.log.term_at(prev_log_index), Some(t) if t == prev_log_term);
            if !matches {
                return AppendOutcome {
                    term: self.current_term,
                    success: false,
                    match_index: self.log.last_index(),
                    persist_required,
                    // A valid leader contacted us, so the timer still resets
                    // even though the log check failed.
                    reset_election_timer: true,
                };
            }
        }

        // §5.3 — append, truncating ONLY on a real conflict (same index,
        // different term). Unconditional truncation would delete committed
        // entries when a shorter duplicate AppendEntries is retransmitted.
        for e in entries {
            match self.log.term_at(e.index) {
                Some(existing) if existing == e.term => continue, // already held
                Some(_) => {
                    self.log.truncate_from(e.index);
                    self.log.append(e.clone());
                    persist_required = true;
                }
                None => {
                    self.log.append(e.clone());
                    persist_required = true;
                }
            }
        }

        // §5.3 — advance commit index, bounded by what we actually hold.
        if leader_commit > self.commit_index {
            self.commit_index = leader_commit.min(self.log.last_index());
        }

        AppendOutcome {
            term: self.current_term,
            success: true,
            match_index: self.log.last_index(),
            persist_required,
            reset_election_timer: true,
        }
    }
}

// ── RaftNode ─────────────────────────────────────────────────────────────────
// Owns the pure state plus the durable backend and cluster config.

pub struct RaftNode {
    pub state: RaftState,
    pub config: ClusterConfig,

    // Mutex so RaftNode is Sync: StorageBackend guarantees Send but not
    // Sync, and tonic services must be Sync. The lock is only held for a
    // synchronous persist, never across an await point.
    #[allow(dead_code)]
    backend: Mutex<Box<dyn StorageBackend>>,
}

impl RaftNode {
    pub fn new(config: ClusterConfig, backend: Box<dyn StorageBackend>) -> Self {
        // TODO M1: restore current_term / voted_for / log from `backend`
        // instead of starting from zero — required for crash safety.
        let state = RaftState::new(config.node_id.clone());
        Self {
            state,
            config,
            backend: Mutex::new(backend),
        }
    }

    /// TODO M1: flush (current_term, voted_for, log) durably. Must be called
    /// before acking any RPC whose outcome had `persist_required = true`.
    pub fn persist(&self) -> Result<(), crate::EdisonError> {
        Ok(())
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────
// RaftState is pure, so these run synchronously with no backend, no tokio
// runtime and no network.

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(index: u64, term: u64) -> LogEntry {
        LogEntry { index, term, command: vec![] }
    }

    fn state_with_log(entries: &[(u64, u64)]) -> RaftState {
        let mut s = RaftState::new("self".to_string());
        for (i, t) in entries {
            s.log.append(entry(*i, *t));
        }
        s
    }

    // ── §5.1 term rules ─────────────────────────────────────────────────

    #[test]
    fn stale_term_vote_is_rejected() {
        let mut s = RaftState::new("self".into());
        s.current_term = 5;
        let out = s.handle_request_vote(3, "c1", 0, 0);
        assert!(!out.vote_granted);
        assert_eq!(out.term, 5);
    }

    #[test]
    fn higher_term_advances_and_clears_vote() {
        let mut s = RaftState::new("self".into());
        s.current_term = 2;
        s.voted_for = Some("old".into());
        s.role = Role::Leader;
        assert!(s.observe_term(7));
        assert_eq!(s.current_term, 7);
        assert_eq!(s.voted_for, None);
        assert_eq!(s.role, Role::Follower);
    }

    #[test]
    fn same_term_does_not_advance() {
        let mut s = RaftState::new("self".into());
        s.current_term = 4;
        assert!(!s.observe_term(4));
    }

    // ── §5.2 one vote per term ──────────────────────────────────────────

    #[test]
    fn second_candidate_denied_in_same_term() {
        let mut s = RaftState::new("self".into());
        assert!(s.handle_request_vote(1, "c1", 0, 0).vote_granted);
        assert!(!s.handle_request_vote(1, "c2", 0, 0).vote_granted);
    }

    #[test]
    fn repeat_request_from_same_candidate_is_idempotent() {
        let mut s = RaftState::new("self".into());
        assert!(s.handle_request_vote(1, "c1", 0, 0).vote_granted);
        let again = s.handle_request_vote(1, "c1", 0, 0);
        assert!(again.vote_granted);
        assert!(!again.persist_required, "no state change on a repeat vote");
    }

    // ── §5.4.1 log freshness ────────────────────────────────────────────

    #[test]
    fn candidate_with_stale_log_is_denied() {
        let mut s = state_with_log(&[(1, 1), (2, 2)]);
        let out = s.handle_request_vote(3, "c1", 1, 1);
        assert!(!out.vote_granted, "shorter/older log must not win a vote");
    }

    #[test]
    fn candidate_with_higher_last_term_wins() {
        let mut s = state_with_log(&[(1, 1), (2, 1)]);
        assert!(s.handle_request_vote(3, "c1", 1, 5).vote_granted);
    }

    // ── REGRESSION: step-down must not erase a vote in the same term ────
    // A candidate that voted for itself in term T, then receives a valid
    // AppendEntries from the term-T leader, must NOT be able to vote again
    // in term T. Erasing the vote here permits two leaders in one term.

    #[test]
    fn step_down_in_same_term_preserves_vote() {
        let mut s = RaftState::new("self".into());
        s.become_candidate(); // term 1, voted for self
        assert_eq!(s.current_term, 1);
        assert_eq!(s.voted_for, Some("self".to_string()));

        let out = s.handle_append_entries(1, 0, 0, &[], 0);
        assert!(out.success);
        assert_eq!(s.role, Role::Follower);
        assert_eq!(
            s.voted_for,
            Some("self".to_string()),
            "vote must survive a same-term step-down"
        );

        let second = s.handle_request_vote(1, "other", 0, 0);
        assert!(!second.vote_granted, "must not double-vote in term 1");
    }

    // ── §5.3 consistency check ──────────────────────────────────────────

    #[test]
    fn prev_log_mismatch_is_rejected() {
        let mut s = state_with_log(&[(1, 1), (2, 1)]);
        let out = s.handle_append_entries(1, 2, 9, &[entry(3, 1)], 0);
        assert!(!out.success);
        assert_eq!(out.match_index, 2);
        assert!(out.reset_election_timer, "a valid leader still resets the timer");
    }

    #[test]
    fn conflicting_entry_truncates_suffix() {
        let mut s = state_with_log(&[(1, 1), (2, 1), (3, 1)]);
        // Leader overwrites index 2 with a different term.
        let out = s.handle_append_entries(2, 1, 1, &[entry(2, 2)], 0);
        assert!(out.success);
        assert_eq!(s.log.last_index(), 2);
        assert_eq!(s.log.term_at(2), Some(2));
        assert_eq!(s.log.term_at(3), None, "conflicting suffix must be dropped");
    }

    // ── REGRESSION: duplicate AppendEntries must not delete entries ─────
    // A retransmitted, shorter AppendEntries carrying entries the follower
    // already holds must be a no-op. Unconditional truncation here would
    // delete already-committed entries.

    #[test]
    fn duplicate_append_does_not_delete_committed_entries() {
        let mut s = state_with_log(&[(1, 1), (2, 1), (3, 1)]);
        s.current_term = 1;
        s.commit_index = 3;

        let out = s.handle_append_entries(1, 0, 0, &[entry(1, 1)], 3);
        assert!(out.success);
        assert_eq!(s.log.last_index(), 3, "entries 2 and 3 must survive");
        assert_eq!(s.log.term_at(2), Some(1));
        assert_eq!(s.log.term_at(3), Some(1));
        assert!(!out.persist_required, "a pure duplicate changes nothing");
    }

    #[test]
    fn heartbeat_advances_commit_index_bounded_by_log() {
        let mut s = state_with_log(&[(1, 1), (2, 1)]);
        s.handle_append_entries(1, 2, 1, &[], 99);
        assert_eq!(s.commit_index, 2, "commit cannot exceed what we hold");
    }

    #[test]
    fn stale_leader_does_not_reset_timer() {
        let mut s = RaftState::new("self".into());
        s.current_term = 6;
        let out = s.handle_append_entries(2, 0, 0, &[], 0);
        assert!(!out.success);
        assert!(!out.reset_election_timer);
    }

    // ── election safety invariant ───────────────────────────────────────

    #[test]
    fn invariant_rejects_two_leaders_in_one_term() {
        let mut s = RaftState::new("self".into());
        s.current_term = 3;
        s.become_leader(&["p1".to_string()]);
        assert!(!s.invariant_single_leader_per_term(3));
        assert!(s.invariant_single_leader_per_term(4));
    }
}
