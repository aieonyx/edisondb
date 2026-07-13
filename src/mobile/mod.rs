// Copyright (c) 2026 Edison Lepiten / AIEONYX
// License: Apache-2.0
//
// EdisonDB Mobile — embedded core module
// Feature-gated: `mobile`. Excludes tonic/gRPC server stack.
// Storage: fjall LSM backend. Signing: BLAKE3. Provenance: ARPi header.

pub mod ffi;
pub mod jni_bridge;

use std::path::Path;
use blake3::Hasher;
use fjall::{Database, Keyspace, KeyspaceCreateOptions};

/// Errors surfaced across the FFI boundary.
#[derive(Debug)]
pub enum DbError {
    Io(std::io::Error),
    Fjall(fjall::Error),
    KeyExists,
    NotFound,
    InvalidArpi,
    Other(String),
}

impl From<fjall::Error> for DbError {
    fn from(e: fjall::Error) -> Self { DbError::Fjall(e) }
}
impl From<std::io::Error> for DbError {
    fn from(e: std::io::Error) -> Self { DbError::Io(e) }
}

/// ARPi header — 78 bytes fixed.
/// Public name: AXON Receptor Protocol Interface.
///
/// Offset  Size  Field
///  0       4    magic: b"ARPi"
///  4       8    write_counter (u64 LE, monotonic)
/// 12       8    timestamp_us (u64 LE, Unix microseconds)
/// 20       1    tier (0=Critical 1=Personal 2=Noise)
/// 21       3    reserved (zero)
/// 24      32    blake3_content_hash
/// 56      22    node_id (UTF-8, zero-padded)
#[derive(Debug, Clone)]
pub struct ArpiHeader {
    pub magic: [u8; 4],
    pub write_counter: u64,
    pub timestamp_us: u64,
    pub tier: u8,
    pub reserved: [u8; 3],
    pub blake3_hash: [u8; 32],
    pub node_id: [u8; 22],
}

impl ArpiHeader {
    pub const SIZE: usize = 78;

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < Self::SIZE { return None; }
        if &bytes[0..4] != b"ARPi" { return None; }
        let write_counter = u64::from_le_bytes(bytes[4..12].try_into().ok()?);
        let timestamp_us  = u64::from_le_bytes(bytes[12..20].try_into().ok()?);
        let tier = bytes[20];
        let mut reserved   = [0u8; 3];  reserved.copy_from_slice(&bytes[21..24]);
        let mut blake3_hash = [0u8; 32]; blake3_hash.copy_from_slice(&bytes[24..56]);
        let mut node_id    = [0u8; 22]; node_id.copy_from_slice(&bytes[56..78]);
        Some(Self { magic: *b"ARPi", write_counter, timestamp_us,
                    tier, reserved, blake3_hash, node_id })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut out = [0u8; Self::SIZE];
        out[0..4].copy_from_slice(&self.magic);
        out[4..12].copy_from_slice(&self.write_counter.to_le_bytes());
        out[12..20].copy_from_slice(&self.timestamp_us.to_le_bytes());
        out[20] = self.tier;
        out[21..24].copy_from_slice(&self.reserved);
        out[24..56].copy_from_slice(&self.blake3_hash);
        out[56..78].copy_from_slice(&self.node_id);
        out
    }
}

fn pack_record(arpi: &[u8; 78], value: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(78 + value.len());
    buf.extend_from_slice(arpi);
    buf.extend_from_slice(value);
    buf
}

fn unpack_value(record: &[u8]) -> Option<&[u8]> {
    if record.len() < 78 { return None; }
    Some(&record[78..])
}

/// Embedded EdisonDB instance — no gRPC, no network stack.
pub struct MobileDb {
    _db: Database,
    partition: Keyspace,
    write_counter: u64,
}

impl MobileDb {
    pub fn open(path: &str) -> Result<Self, DbError> {
        let db = Database::builder(Path::new(path))
            .open()
            .map_err(DbError::Fjall)?;
        let partition = db
            .keyspace("main", KeyspaceCreateOptions::default)
            .map_err(DbError::Fjall)?;

        let counter = match partition.get(b"__write_counter__") {
            Ok(Some(v)) => {
                let arr: [u8; 8] = v.as_ref().try_into().unwrap_or([0u8; 8]);
                u64::from_le_bytes(arr)
            }
            _ => 0u64,
        };

        Ok(Self { _db: db, partition, write_counter: counter })
    }

    fn next_counter(&mut self) -> u64 {
        self.write_counter += 1;
        self.write_counter
    }

    fn persist_counter(&self) -> Result<(), DbError> {
        self.partition
            .insert(b"__write_counter__", &self.write_counter.to_le_bytes())
            .map_err(DbError::Fjall)
    }

    pub fn insert(&mut self, key: &str, value: &str, arpi_raw: &[u8]) -> Result<(), DbError> {
        if arpi_raw.len() < 78 { return Err(DbError::InvalidArpi); }
        let header = ArpiHeader::from_bytes(arpi_raw).ok_or(DbError::InvalidArpi)?;

        // Verify BLAKE3 hash matches value
        let mut h = Hasher::new();
        h.update(value.as_bytes());
        let computed: [u8; 32] = h.finalize().into();
        if computed != header.blake3_hash {
            return Err(DbError::InvalidArpi);
        }

        let counter = self.next_counter();
        let mut final_header = header.clone();
        final_header.write_counter = counter;
        let header_bytes = final_header.to_bytes();

        let record = pack_record(&header_bytes, value.as_bytes());
        self.partition
            .insert(key.as_bytes(), &record)
            .map_err(DbError::Fjall)?;
        self.persist_counter()?;
        Ok(())
    }

    pub fn query(&self, key: &str) -> Result<Option<String>, DbError> {
        match self.partition.get(key.as_bytes()) {
            Ok(Some(record)) => {
                let value_bytes = unpack_value(record.as_ref()).unwrap_or(&[]);
                Ok(Some(String::from_utf8_lossy(value_bytes).into_owned()))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(DbError::Fjall(e)),
        }
    }

    pub fn delete(&mut self, key: &str) -> Result<bool, DbError> {
        let existed = self.partition.get(key.as_bytes())
            .map(|v| v.is_some())
            .unwrap_or(false);
        self.partition
            .remove(key.as_bytes())
            .map_err(DbError::Fjall)?;
        Ok(existed)
    }
}
