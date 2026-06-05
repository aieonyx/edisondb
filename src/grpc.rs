// src/grpc.rs
// Copyright 2026 Edison Lepiten — Apache 2.0
//
// EdisonDB gRPC server — tonic 0.14
// Transport layer only. All encryption, decryption, and business logic
// lives in EqlExecutor and StorageBackend. This file is pure routing.
//
// Security properties:
//   - x-password metadata never logged, zeroized after use
//   - owner_id enforces Inverted Admin Model (same as REST X-Owner-ID)
//   - No unsafe blocks
//   - TLS deferred to P3-M6

use std::sync::Arc;
use tonic::{Request, Response, Status};
use zeroize::Zeroizing;

use crate::backends::Router;
use crate::executor::EqlExecutor;

// ── Proto generated code ────────────────────────────────────────────────────

pub mod proto {
    tonic::include_proto!("edisondb");
}

use proto::{
    edison_db_server::{EdisonDb, EdisonDbServer},
    AuditRequest, AuditResponse,
    DeleteRequest, DeleteResponse,
    EmbedRequest, EmbedResponse,
    ListRequest, ListResponse,
    ReadRequest, ReadResponse,
    SearchRequest, SearchResponse, SearchHit,
    WriteRequest, WriteResponse,
};

// ── Server struct ────────────────────────────────────────────────────────────

pub struct EdisonDbGrpc {
    router: Arc<Router>,
}

impl EdisonDbGrpc {
    pub fn new(router: Arc<Router>) -> Self {
        Self { router }
    }

    /// Extract x-password from gRPC metadata.
    /// Wrapped in Zeroizing so the secret is wiped from memory when dropped.
    fn extract_password(
        metadata: &tonic::metadata::MetadataMap,
    ) -> Result<Zeroizing<String>, Status> {
        metadata
            .get("x-password")
            .and_then(|v| v.to_str().ok())
            .map(|s| Zeroizing::new(s.to_string()))
            .ok_or_else(|| Status::unauthenticated("x-password metadata header is required"))
    }
}

// ── Tier helpers ─────────────────────────────────────────────────────────────

fn map_tier(tier_int: i32) -> Result<crate::DataTier, Status> {
    match tier_int {
        0 => Ok(crate::DataTier::Critical),
        1 => Ok(crate::DataTier::Personal),
        2 => Ok(crate::DataTier::Noise),
        _ => Err(Status::invalid_argument(format!(
            "Invalid tier value: {}. Expected 0 (CRITICAL), 1 (PERSONAL), or 2 (NOISE)",
            tier_int
        ))),
    }
}

fn tier_str(tier: &crate::DataTier) -> &'static str {
    match tier {
        crate::DataTier::Critical => "CRITICAL",
        crate::DataTier::Personal => "PERSONAL",
        crate::DataTier::Noise    => "NOISE",
    }
}

// ── EdisonDb trait implementation ────────────────────────────────────────────

#[tonic::async_trait]
impl EdisonDb for EdisonDbGrpc {

    // ── WRITE ────────────────────────────────────────────────────────────────

    async fn write(
        &self,
        request: Request<WriteRequest>,
    ) -> Result<Response<WriteResponse>, Status> {
        let password = Self::extract_password(request.metadata())?;
        let req = request.into_inner();

        if req.owner_id.is_empty() {
            return Err(Status::unauthenticated("owner_id is required"));
        }
        if req.record_id.is_empty() {
            return Err(Status::invalid_argument("record_id is required"));
        }
        if req.payload.is_empty() {
            return Err(Status::invalid_argument("payload must not be empty"));
        }

        let tier = map_tier(req.tier)?;
        let payload_str = String::from_utf8(req.payload)
            .map_err(|_| Status::invalid_argument("payload must be valid UTF-8"))?;

        // EQL: WRITE <id> TIER <tier> VALUE "<payload>"
        let eql = format!(
            "WRITE {} TIER {} VALUE \"{}\"",
            req.record_id,
            tier_str(&tier),
            payload_str.replace('"', "\\\""),
        );

        let mut executor = EqlExecutor::new(
            self.router.clone(),
            req.owner_id,
            password.as_str().to_string(),
        );

        executor
            .execute(&eql)
            .map(|_| Response::new(WriteResponse {
                success: true,
                message: "ok".into(),
            }))
            .map_err(|e| Status::internal(e.to_string()))
    }

    // ── READ ─────────────────────────────────────────────────────────────────

    async fn read(
        &self,
        request: Request<ReadRequest>,
    ) -> Result<Response<ReadResponse>, Status> {
        let password = Self::extract_password(request.metadata())?;
        let req = request.into_inner();

        if req.owner_id.is_empty() {
            return Err(Status::unauthenticated("owner_id is required"));
        }

        let tier = map_tier(req.tier)?;

        let eql = format!(
            "READ {} TIER {}",
            req.record_id,
            tier_str(&tier),
        );

        let mut executor = EqlExecutor::new(
            self.router.clone(),
            req.owner_id,
            password.as_str().to_string(),
        );

        match executor.execute(&eql) {
            Ok(Some(value)) => Ok(Response::new(ReadResponse {
                found:   true,
                payload: value.into_bytes(),
                message: "ok".into(),
            })),
            Ok(None) => Ok(Response::new(ReadResponse {
                found:   false,
                payload: vec![],
                message: "not found".into(),
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    // ── LIST ─────────────────────────────────────────────────────────────────

    async fn list(
        &self,
        request: Request<ListRequest>,
    ) -> Result<Response<ListResponse>, Status> {
        let password = Self::extract_password(request.metadata())?;
        let req = request.into_inner();

        if req.owner_id.is_empty() {
            return Err(Status::unauthenticated("owner_id is required"));
        }

        let tier = map_tier(req.tier)?;

        let eql = format!("LIST TIER {}", tier_str(&tier));

        let mut executor = EqlExecutor::new(
            self.router.clone(),
            req.owner_id,
            password.as_str().to_string(),
        );

        executor
            .execute_list(&eql)
            .map(|ids| Response::new(ListResponse { record_ids: ids }))
            .map_err(|e| Status::internal(e.to_string()))
    }

    // ── DELETE ───────────────────────────────────────────────────────────────

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let password = Self::extract_password(request.metadata())?;
        let req = request.into_inner();

        if req.owner_id.is_empty() {
            return Err(Status::unauthenticated("owner_id is required"));
        }
        if req.record_id.is_empty() {
            return Err(Status::invalid_argument("record_id is required"));
        }

        let tier = map_tier(req.tier)?;

        let eql = format!(
            "DELETE {} TIER {}",
            req.record_id,
            tier_str(&tier),
        );

        let mut executor = EqlExecutor::new(
            self.router.clone(),
            req.owner_id,
            password.as_str().to_string(),
        );

        executor
            .execute(&eql)
            .map(|_| Response::new(DeleteResponse {
                success: true,
                message: "ok".into(),
            }))
            .map_err(|e| Status::internal(e.to_string()))
    }

    // ── AUDIT ────────────────────────────────────────────────────────────────

    async fn audit(
        &self,
        request: Request<AuditRequest>,
    ) -> Result<Response<AuditResponse>, Status> {
        let password = Self::extract_password(request.metadata())?;
        let req = request.into_inner();

        if req.owner_id.is_empty() {
            return Err(Status::unauthenticated("owner_id is required"));
        }
        if req.record_id.is_empty() {
            return Err(Status::invalid_argument("record_id is required"));
        }

        let eql = format!("AUDIT {}", req.record_id);

        let mut executor = EqlExecutor::new(
            self.router.clone(),
            req.owner_id,
            password.as_str().to_string(),
        );

        executor
            .execute_audit(&eql)
            .map(|entries| Response::new(AuditResponse { entries }))
            .map_err(|e| Status::internal(e.to_string()))
    }

    // ── EMBED ────────────────────────────────────────────────────────────────

    async fn embed(
        &self,
        request: Request<EmbedRequest>,
    ) -> Result<Response<EmbedResponse>, Status> {
        let password = Self::extract_password(request.metadata())?;
        let req = request.into_inner();

        if req.owner_id.is_empty() {
            return Err(Status::unauthenticated("owner_id is required"));
        }

        let tier = map_tier(req.tier)?;
        let payload_str = String::from_utf8(req.payload)
            .map_err(|_| Status::invalid_argument("payload must be valid UTF-8"))?;

        let eql = format!(
            "EMBED {} TIER {} VALUE \"{}\"",
            req.record_id,
            tier_str(&tier),
            payload_str.replace('"', "\\\""),
        );

        let mut executor = EqlExecutor::new(
            self.router.clone(),
            req.owner_id,
            password.as_str().to_string(),
        );

        executor
            .execute(&eql)
            .map(|_| Response::new(EmbedResponse {
                success: true,
                message: "ok".into(),
            }))
            .map_err(|e| Status::internal(e.to_string()))
    }

    // ── SEARCH ───────────────────────────────────────────────────────────────

    async fn search(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<SearchResponse>, Status> {
        let password = Self::extract_password(request.metadata())?;
        let req = request.into_inner();

        if req.owner_id.is_empty() {
            return Err(Status::unauthenticated("owner_id is required"));
        }
        if req.query_vec.is_empty() {
            return Err(Status::invalid_argument("query_vec must not be empty"));
        }
        if req.query_vec.len() % 4 != 0 {
            return Err(Status::invalid_argument(
                "query_vec must be a sequence of 4-byte little-endian f32 values",
            ));
        }

        let tier = map_tier(req.tier)?;
        let top_k = if req.top_k == 0 { 10 } else { req.top_k };

        // Deserialize bytes → Vec<f32>
        let floats: Vec<f32> = req
            .query_vec
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect();

        let mut executor = EqlExecutor::new(
            self.router.clone(),
            req.owner_id,
            password.as_str().to_string(),
        );

        executor
            .execute_search(tier, &floats, top_k as usize)
            .map(|hits| {
                let grpc_hits = hits
                    .into_iter()
                    .map(|(id, score)| SearchHit { record_id: id, score })
                    .collect();
                Response::new(SearchResponse { hits: grpc_hits })
            })
            .map_err(|e| Status::internal(e.to_string()))
    }
}

// ── Server entrypoint ────────────────────────────────────────────────────────

/// Start the gRPC server. Called from main.rs via tokio::join!
pub async fn serve(router: Arc<Router>, port: u16) {
    let addr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("grpc: invalid bind address");

    let svc = EdisonDbGrpc::new(router);

    println!("EdisonDB gRPC server listening on port {}", port);

    tonic::transport::Server::builder()
        .add_service(EdisonDbServer::new(svc))
        .serve(addr)
        .await
        .expect("grpc: server failed");
}
