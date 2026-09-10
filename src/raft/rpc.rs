// Copyright (c) 2026 Edison Lepiten / AIEONYX

use super::SharedRaftNode;

// TODO: once `tonic-build` picks up proto/raft.proto (add to build.rs
// alongside the existing edisondb.proto compilation), this becomes:
//   use crate::raft_proto::raft_consensus_server::RaftConsensus;
//   use crate::raft_proto::{RequestVoteRequest, RequestVoteResponse, ...};

pub struct RaftService {
    pub node: SharedRaftNode,
}

impl RaftService {
    pub fn new(node: SharedRaftNode) -> Self {
        Self { node }
    }

    // TODO M1: implement request_vote handler per Raft §5.2/5.4
    // TODO M1: implement append_entries handler per Raft §5.3
}
