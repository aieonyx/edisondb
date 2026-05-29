use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;
use argon2::Argon2;
use redb::{Database, TableDefinition, ReadableTable};

const RECORDS_TABLE: TableDefinition<u64, &str> = TableDefinition::new("records");
const AUDIT_TABLE: TableDefinition<u64, &str> = TableDefinition::new("audit");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataTier {
    Critical,
    Personal,
    Noise,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub id: u64,
    pub tier: DataTier,
    pub owner_id: String,
    pub payload: Vec<u8>,
    pub salt: [u8; 32],
    pub created_at: u64,
}

impl Record {
    pub fn new(
        id: u64,
        tier: DataTier,
        owner_id: &str,
        payload: Vec<u8>,
        salt: [u8; 32],
    ) -> Result<Self, EdisonError> {
        if owner_id.is_empty() {
            return Err(EdisonError::NoOwner);
        }
        Ok(Record {
            id,
            tier,
            owner_id: owner_id.to_string(),
            payload,
            salt,
            created_at: now(),
        })
    }

    fn is_readable_by(&self, requester_id: &str) -> bool {
        match self.tier {
            DataTier::Critical => requester_id == self.owner_id,
            DataTier::Personal => requester_id == self.owner_id,
            DataTier::Noise => true,
        }
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum EdisonError {
    #[error("Record must have an owner")]
    NoOwner,
    #[error("Access denied — owner only")]
    AccessDenied,
    #[error("Record not found")]
    NotFound,
    #[error("Failed to save database")]
    SaveFailed,
    #[error("Failed to load database")]
    LoadFailed,
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed — wrong key or corrupted data")]
    DecryptionFailed,
    #[error("Key derivation failed")]
    KeyDerivationFailed,
    #[error("Record already exists")]
    AlreadyExists,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditAction {
    Write,
    ReadGranted,
    ReadDenied,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub record_id: u64,
    pub requester_id: String,
    pub action: AuditAction,
    pub timestamp: u64,
}

pub struct Store {
    pub records: HashMap<u64, Record>,
    pub audit_log: Vec<AuditEntry>,
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Store {
    pub fn new() -> Self {
        Store {
            records: HashMap::new(),
            audit_log: Vec::new(),
        }
    }

    pub fn write(&mut self, record: Record) -> Result<(), EdisonError> {
        if self.records.contains_key(&record.id) {
            return Err(EdisonError::AlreadyExists);
        }
        self.audit_log.push(AuditEntry {
            record_id: record.id,
            requester_id: record.owner_id.clone(),
            action: AuditAction::Write,
            timestamp: now(),
        });
        self.records.insert(record.id, record);
        Ok(())
    }
  

    pub fn read(
        &mut self,
        id: u64,
        requester_id: &str,
    ) -> Result<&Record, EdisonError> {
        match self.records.get(&id) {
            None => Err(EdisonError::NotFound),
            Some(record) => {
                if record.is_readable_by(requester_id) {
                    self.audit_log.push(AuditEntry {
                        record_id: id,
                        requester_id: requester_id.to_string(),
                        action: AuditAction::ReadGranted,
                        timestamp: now(),
                    });
                    Ok(record)
                } else {
                    self.audit_log.push(AuditEntry {
                        record_id: id,
                        requester_id: requester_id.to_string(),
                        action: AuditAction::ReadDenied,
                        timestamp: now(),
                    });
                    Err(EdisonError::AccessDenied)
                }
            }
        }
    }

    pub fn audit_count(&self) -> usize {
        self.audit_log.len()
    }

    pub fn list_by_owner(&self, owner_id: &str) -> Vec<&Record> {
        self.records
            .values()
            .filter(|r| r.owner_id == owner_id)
            .collect()
    }

    pub fn audit_entries(&self) -> &Vec<AuditEntry> {
        &self.audit_log
    }

    pub fn delete(
        &mut self,
        id: u64,
        requester_id: &str,
    ) -> Result<(), EdisonError> {
        match self.records.get(&id) {
            None => Err(EdisonError::NotFound),
            Some(record) => {
                if record.owner_id != requester_id {
                    return Err(EdisonError::AccessDenied);
                }
                self.audit_log.push(AuditEntry {
                    record_id: id,
                    requester_id: requester_id.to_string(),
                    action: AuditAction::Delete,
                    timestamp: now(),
                });
                self.records.remove(&id);
                Ok(())
            }
        }
    }

    pub fn save(&self, path: &str) -> Result<(), EdisonError> {
        let db = Database::create(path)
            .map_err(|_| EdisonError::SaveFailed)?;
        let write_txn = db.begin_write()
            .map_err(|_| EdisonError::SaveFailed)?;
        {
            let mut table = write_txn.open_table(RECORDS_TABLE)
                .map_err(|_| EdisonError::SaveFailed)?;
            for (id, record) in &self.records {
                let json = serde_json::to_string(record)
                    .map_err(|_| EdisonError::SaveFailed)?;
                table.insert(*id, json.as_str())
                    .map_err(|_| EdisonError::SaveFailed)?;
            }
        }
        {
            let mut table = write_txn.open_table(AUDIT_TABLE)
                .map_err(|_| EdisonError::SaveFailed)?;
            for (i, entry) in self.audit_log.iter().enumerate() {
                let json = serde_json::to_string(entry)
                    .map_err(|_| EdisonError::SaveFailed)?;
                table.insert(i as u64, json.as_str())
                    .map_err(|_| EdisonError::SaveFailed)?;
            }
        }
        write_txn.commit()
            .map_err(|_| EdisonError::SaveFailed)?;
        Ok(())
    }

    pub fn load(path: &str) -> Result<Self, EdisonError> {
        let db = Database::open(path)
            .map_err(|_| EdisonError::LoadFailed)?;
        let read_txn = db.begin_read()
            .map_err(|_| EdisonError::LoadFailed)?;
        let mut records = HashMap::new();
        let table = read_txn.open_table(RECORDS_TABLE)
            .map_err(|_| EdisonError::LoadFailed)?;
        for entry in table.iter()
            .map_err(|_| EdisonError::LoadFailed)? {
            let (key, value) = entry
                .map_err(|_| EdisonError::LoadFailed)?;
            let record: Record = serde_json::from_str(value.value())
                .map_err(|_| EdisonError::LoadFailed)?;
            records.insert(key.value(), record);
        }
        let mut audit_log = Vec::new();
        if let Ok(table) = read_txn.open_table(AUDIT_TABLE)
            && let Ok(iter) = table.iter() {
                for (_, value) in iter.flatten() {
                    if let Ok(a) = serde_json::from_str(value.value()) {
                        audit_log.push(a);
                    }
                }
            }
        Ok(Store { records, audit_log })
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn encrypt_payload(
    data: &[u8],
    key: &[u8; 32],
) -> Result<Vec<u8>, EdisonError> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let mut encrypted = cipher
        .encrypt(nonce, data)
        .map_err(|_| EdisonError::EncryptionFailed)?;
    let mut result = nonce_bytes.to_vec();
    result.append(&mut encrypted);
    Ok(result)
}

pub fn decrypt_payload(
    data: &[u8],
    key: &[u8; 32],
) -> Result<Vec<u8>, EdisonError> {
    if data.len() < 12 {
        return Err(EdisonError::DecryptionFailed);
    }
    let (nonce_bytes, encrypted) = data.split_at(12);
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher
        .decrypt(nonce, encrypted)
        .map_err(|_| EdisonError::DecryptionFailed)
}

pub fn derive_key(password: &str, salt: &[u8; 32]) -> Result<[u8; 32], EdisonError> {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(
            password.as_bytes(),
            salt,
            &mut key,
        )
        .map_err(|_| EdisonError::KeyDerivationFailed)?;
    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_can_read_critical() {
        let r = Record::new(1, DataTier::Critical,
            "owner_abc", vec![1,2,3], [0u8; 32]).unwrap();
        assert!(r.is_readable_by("owner_abc"));
    }

    #[test]
    fn non_owner_cannot_read_critical() {
        let r = Record::new(2, DataTier::Critical,
            "owner_abc", vec![1,2,3], [0u8; 32]).unwrap();
        assert!(!r.is_readable_by("attacker"));
    }

    #[test]
    fn admin_cannot_read_critical() {
        let r = Record::new(3, DataTier::Critical,
            "owner_abc", vec![1,2,3], [0u8; 32]).unwrap();
        assert!(!r.is_readable_by("admin"));
        assert!(!r.is_readable_by("root"));
    }

    #[test]
    fn noise_readable_by_anyone() {
        let r = Record::new(4, DataTier::Noise,
            "owner_abc", vec![9,8,7], [0u8; 32]).unwrap();
        assert!(r.is_readable_by("anyone"));
    }

    #[test]
    fn record_without_owner_rejected() {
        let result = Record::new(5, DataTier::Personal,
            "", vec![1], [0u8; 32]);
        assert_eq!(result, Err(EdisonError::NoOwner));
    }

    #[test]
    fn record_has_timestamp() {
        let r = Record::new(6, DataTier::Personal,
            "owner_abc", vec![], [0u8; 32]).unwrap();
        assert!(r.created_at > 0);
    }

    #[test]
    fn owner_can_read_stored_record() {
        let mut store = Store::new();
        let r = Record::new(10, DataTier::Personal,
            "owner_abc", vec![1,2,3], [0u8; 32]).unwrap();
        store.write(r).unwrap();
        assert!(store.read(10, "owner_abc").is_ok());
    }

    #[test]
    fn attacker_cannot_read_stored_record() {
        let mut store = Store::new();
        let r = Record::new(11, DataTier::Critical,
            "owner_abc", vec![1,2,3], [0u8; 32]).unwrap();
        store.write(r).unwrap();
        assert_eq!(
            store.read(11, "attacker"),
            Err(EdisonError::AccessDenied)
        );
    }

    #[test]
    fn write_creates_audit_entry() {
        let mut store = Store::new();
        let r = Record::new(20, DataTier::Personal,
            "owner_abc", vec![1], [0u8; 32]).unwrap();
        store.write(r).unwrap();
        assert_eq!(store.audit_count(), 1);
    }

    #[test]
    fn multiple_writes_all_audited() {
        let mut store = Store::new();
        for i in 0..5 {
            let r = Record::new(i, DataTier::Noise,
                "owner_abc", vec![], [0u8; 32]).unwrap();
            store.write(r).unwrap();
        }
        assert_eq!(store.audit_count(), 5);
    }

    #[test]
    fn granted_read_is_audited() {
        let mut store = Store::new();
        let r = Record::new(30, DataTier::Personal,
            "owner_abc", vec![1], [0u8; 32]).unwrap();
        store.write(r).unwrap();
        let _ = store.read(30, "owner_abc");
        assert_eq!(store.audit_count(), 2);
    }

    #[test]
    fn denied_read_is_audited() {
        let mut store = Store::new();
        let r = Record::new(31, DataTier::Critical,
            "owner_abc", vec![1], [0u8; 32]).unwrap();
        store.write(r).unwrap();
        let _ = store.read(31, "attacker");
        assert_eq!(store.audit_count(), 2);
    }

    #[test]
    fn list_returns_owner_records_only() {
        let mut store = Store::new();
        let r1 = Record::new(60, DataTier::Personal,
            "alice", vec![1], [0u8; 32]).unwrap();
        let r2 = Record::new(61, DataTier::Noise,
            "bob", vec![2], [0u8; 32]).unwrap();
        store.write(r1).unwrap();
        store.write(r2).unwrap();
        let alice_records = store.list_by_owner("alice");
        assert_eq!(alice_records.len(), 1);
        assert_eq!(alice_records[0].id, 60);
    }

    #[test]
    fn owner_can_delete_own_record() {
        let mut store = Store::new();
        let r = Record::new(70, DataTier::Personal,
            "alice", vec![1], [0u8; 32]).unwrap();
        store.write(r).unwrap();
        assert!(store.delete(70, "alice").is_ok());
        assert_eq!(store.list_by_owner("alice").len(), 0);
    }

    #[test]
    fn non_owner_cannot_delete_record() {
        let mut store = Store::new();
        let r = Record::new(71, DataTier::Critical,
            "alice", vec![1], [0u8; 32]).unwrap();
        store.write(r).unwrap();
        assert_eq!(
            store.delete(71, "attacker"),
            Err(EdisonError::AccessDenied)
        );
    }

    #[test]
    fn payload_encrypts_and_decrypts() {
        let key = [0u8; 32];
        let original = b"sovereign data";
        let encrypted = encrypt_payload(original, &key).unwrap();
        assert_ne!(encrypted, original.to_vec());
        let decrypted = decrypt_payload(&encrypted, &key).unwrap();
        assert_eq!(decrypted, original);
    }

    #[test]
    fn store_saves_and_loads() {
        let path = "/tmp/test_edison.redb";
        let _ = std::fs::remove_file(path);
        let mut store = Store::new();
        let r = Record::new(40, DataTier::Personal,
            "owner_abc", vec![1,2,3], [0u8; 32]).unwrap();
        store.write(r).unwrap();
        store.save(path).unwrap();
        let loaded = Store::load(path).unwrap();
        let record = loaded.records.get(&40).unwrap();
        assert_eq!(record.owner_id, "owner_abc");
    }

    #[test]
    fn audit_log_persists() {
        let path = "/tmp/test_audit.redb";
        let _ = std::fs::remove_file(path);
        let mut store = Store::new();
        let r = Record::new(50, DataTier::Personal,
            "owner_abc", vec![1], [0u8; 32]).unwrap();
        store.write(r).unwrap();
        let _ = store.read(50, "owner_abc");
        store.save(path).unwrap();
        let loaded = Store::load(path).unwrap();
        assert_eq!(loaded.audit_count(), 2);
    }

    #[test]
    fn same_password_same_key() {
        let salt = [1u8; 32];
        let key1 = derive_key("owner_password", &salt).unwrap();
        let key2 = derive_key("owner_password", &salt).unwrap();
        assert_eq!(key1, key2);
    }

    #[test]
    fn different_password_different_key() {
        let salt = [1u8; 32];
        let key1 = derive_key("owner_password", &salt).unwrap();
        let key2 = derive_key("wrong_password", &salt).unwrap();
        assert_ne!(key1, key2);
    }

    #[test]
    fn duplicate_id_rejected() {
        let mut store = Store::new();
        let r1 = Record::new(100, DataTier::Personal,
            "alice", vec![1], [0u8; 32]).unwrap();
        let r2 = Record::new(100, DataTier::Personal,
            "alice", vec![2], [0u8; 32]).unwrap();
        store.write(r1).unwrap();
        assert_eq!(store.write(r2), Err(EdisonError::AlreadyExists));
    }
}