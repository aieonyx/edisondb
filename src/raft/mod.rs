// Copyright (c) 2026 Edison Lepiten / AIEONYX

mod election;
mod log;
#[cfg(feature = "server")]
mod rpc;
mod state_machine;

pub use election::ElectionTimer;
pub use log::{RaftLog, LogEntry};
#[cfg(feature = "server")]
pub use rpc::RaftService;
pub use state_machine::{RaftNode, Role};

use std::sync::Arc;
use tokio::sync::RwLock;

/// Static cluster membership for M1 — dynamic join/leave deferred to P4-M3.
#[derive(Clone, Debug)]
pub struct ClusterConfig {
    pub node_id: String,
    pub peers: Vec<String>, // "host:port" for each other node
}

pub type SharedRaftNode = Arc<RwLock<RaftNode>>;
