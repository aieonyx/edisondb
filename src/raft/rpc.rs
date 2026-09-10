// Copyright (c) 2026 Edison Lepiten / AIEONYX

// Raft consensus RPC handlers — implements Raft §5.1-§5.4.
// Mirrors the `pub mod proto` convention used in src/grpc.rs.

use tonic::{Request, Response, Status};

use super::{Role, SharedRaftNode};

pub mod proto {
    tonic::include_proto!("raft");
}

use proto::{
    raft_consensus_server::{RaftConsensus, RaftConsensusServer},
    AppendEntriesRequest, AppendEntriesResponse, RequestVoteRequest, RequestVoteResponse,
};

pub struct RaftService {
    pub node: SharedRaftNode,
}

impl RaftService {
    pub fn new(node: SharedRaftNode) -> Self {
        Self { node }
    }

    pub fn into_server(self) -> RaftConsensusServer<Self> {
        RaftConsensusServer::new(self)
    }
}

#[tonic::async_trait]
impl RaftConsensus for RaftService {
    /// Raft §5.2 + §5.4.1 — grant a vote only to a candidate whose log is
    /// at least as up-to-date as ours. This is what preserves the Leader
    /// Completeness property; without the log-freshness check a stale node
    /// could win an election and truncate committed entries.
    async fn request_vote(
        &self,
        request: Request<RequestVoteRequest>,
    ) -> Result<Response<RequestVoteResponse>, Status> {
        let req = request.into_inner();
        let mut node = self.node.write().await;

        // §5.1 — reject any RPC from a stale term.
        if req.term < node.current_term {
            return Ok(Response::new(RequestVoteResponse {
                term: node.current_term,
                vote_granted: false,
            }));
        }

        // §5.1 — a higher term always forces us back to Follower and
        // clears our vote for the new term.
        if req.term > node.current_term {
            node.become_follower(req.term);
        }

        // §5.4.1 — candidate log must be at least as up-to-date as ours.
        let our_last_term = node.log.last_term();
        let our_last_index = node.log.last_index();
        let candidate_log_ok = req.last_log_term > our_last_term
            || (req.last_log_term == our_last_term && req.last_log_index >= our_last_index);

        // §5.2 — one vote per term, but re-granting to the same candidate
        // is safe and makes retries idempotent.
        let can_vote = match &node.voted_for {
            None => true,
            Some(id) => id == &req.candidate_id,
        };

        let vote_granted = can_vote && candidate_log_ok;
        if vote_granted {
            node.voted_for = Some(req.candidate_id.clone());
            // TODO M1: persist (current_term, voted_for) via node.backend
            // BEFORE returning — Raft requires this durable or a crash-restart
            // could double-vote in the same term and elect two leaders.
        }

        Ok(Response::new(RequestVoteResponse {
            term: node.current_term,
            vote_granted,
        }))
    }

    /// Raft §5.3 — log replication, and heartbeat when `entries` is empty.
    async fn append_entries(
        &self,
        request: Request<AppendEntriesRequest>,
    ) -> Result<Response<AppendEntriesResponse>, Status> {
        let req = request.into_inner();
        let mut node = self.node.write().await;

        // §5.1 — reject stale-term leaders.
        if req.term < node.current_term {
            return Ok(Response::new(AppendEntriesResponse {
                term: node.current_term,
                success: false,
                match_index: 0,
            }));
        }

        // Valid leader for this term (or a newer one) — step down.
        if req.term > node.current_term || node.role != Role::Follower {
            node.become_follower(req.term);
        }
        // TODO M1: reset the election timer here — a valid AppendEntries is
        // exactly the heartbeat that must prevent a spurious election.

        // §5.3 — consistency check: our log must contain a matching entry
        // at prev_log_index, otherwise the leader backs up and retries.
        if req.prev_log_index > 0 {
            match node.log.term_at(req.prev_log_index) {
                Some(t) if t == req.prev_log_term => {}
                _ => {
                    return Ok(Response::new(AppendEntriesResponse {
                        term: node.current_term,
                        success: false,
                        match_index: node.log.last_index(),
                    }));
                }
            }
        }

        // §5.3 — log matching property: drop any conflicting suffix, then
        // append the leader's entries.
        if let Some(first) = req.entries.first() {
            node.log.truncate_from(first.index);
        }
        for e in &req.entries {
            node.log.append(super::LogEntry {
                index: e.index,
                term: e.term,
                command: e.command.clone(),
            });
        }
        // TODO M1: persist appended entries via node.backend before ack —
        // acking an unpersisted entry can lose a committed write on crash.

        // §5.3 — advance commit index, bounded by what we actually hold.
        if req.leader_commit > node.commit_index {
            node.commit_index = req.leader_commit.min(node.log.last_index());
        }
        // TODO M1: apply committed-but-unapplied entries to the state
        // machine (last_applied -> commit_index) via node.backend.

        Ok(Response::new(AppendEntriesResponse {
            term: node.current_term,
            success: true,
            match_index: node.log.last_index(),
        }))
    }
}
