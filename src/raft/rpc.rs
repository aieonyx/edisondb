// Copyright (c) 2026 Edison Lepiten / AIEONYX

// Raft consensus RPC surface. This module is a thin translation layer:
// proto types in, domain types out, decisions delegated to RaftState.
// Mirrors the `pub mod proto` convention used in src/grpc.rs.

use tonic::{Request, Response, Status};

use super::{LogEntry, SharedRaftNode};

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
    async fn request_vote(
        &self,
        request: Request<RequestVoteRequest>,
    ) -> Result<Response<RequestVoteResponse>, Status> {
        let req = request.into_inner();
        let mut node = self.node.write().await;

        let outcome = node.state.handle_request_vote(
            req.term,
            &req.candidate_id,
            req.last_log_index,
            req.last_log_term,
        );

        if outcome.persist_required {
            node.persist()
                .map_err(|e| Status::internal(format!("raft persist failed: {e}")))?;
        }

        Ok(Response::new(RequestVoteResponse {
            term: outcome.term,
            vote_granted: outcome.vote_granted,
        }))
    }

    async fn append_entries(
        &self,
        request: Request<AppendEntriesRequest>,
    ) -> Result<Response<AppendEntriesResponse>, Status> {
        let req = request.into_inner();
        let mut node = self.node.write().await;

        let entries: Vec<LogEntry> = req
            .entries
            .iter()
            .map(|e| LogEntry {
                index: e.index,
                term: e.term,
                command: e.command.clone(),
            })
            .collect();

        let outcome = node.state.handle_append_entries(
            req.term,
            req.prev_log_index,
            req.prev_log_term,
            &entries,
            req.leader_commit,
        );

        if outcome.persist_required {
            node.persist()
                .map_err(|e| Status::internal(format!("raft persist failed: {e}")))?;
        }

        // TODO M1: if outcome.reset_election_timer, reset the node's
        // ElectionTimer once the timer is owned by RaftNode.

        Ok(Response::new(AppendEntriesResponse {
            term: outcome.term,
            success: outcome.success,
            match_index: outcome.match_index,
        }))
    }
}
