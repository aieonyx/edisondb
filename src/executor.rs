use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::RngCore;

use crate::{DataTier, EdisonError, Record, Store, decrypt_payload, derive_key, encrypt_payload};
use crate::eql::{Statement, Tier};

// ── ID hashing ────────────────────────────────────────────────────────────────
// M1: string IDs hashed to u64 to match existing Record.id: u64.
// M2 migration: change Record.id to String, remove this.
fn id_hash(s: &str) -> u64 {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}

// ── Payload envelope ──────────────────────────────────────────────────────────
// The string ID is embedded inside the encrypted envelope so LIST can recover it.
// Format: "<string_id>\n<payload>"
fn encode(id: &str, payload: &str) -> String {
    format!("{}\n{}", id, payload)
}

fn decode(raw: &str) -> (&str, &str) {
    raw.split_once('\n').unwrap_or(("", raw))
}

// ── Tier conversion ───────────────────────────────────────────────────────────
fn to_data_tier(t: &Tier) -> DataTier {
    match t {
        Tier::Critical => DataTier::Critical,
        Tier::Personal => DataTier::Personal,
        Tier::Noise    => DataTier::Noise,
    }
}

// ── Result types ──────────────────────────────────────────────────────────────
#[derive(Debug)]
pub enum EqlResult {
    Written { id: String, tier: Tier },
    Read    { id: String, tier: DataTier, payload: String },
    Listed  (Vec<RecordInfo>),
    Deleted { id: String },
    Audited (Vec<String>),
}

#[derive(Debug)]
pub struct RecordInfo {
    pub string_id:  String,
    pub tier:       DataTier,
    pub created_at: u64,
}

impl std::fmt::Display for EqlResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EqlResult::Written { id, tier } =>
                write!(f, "OK  WROTE   [{tier}] {id}"),
            EqlResult::Read { id, tier, payload } =>
                write!(f, "OK  READ    [{tier:?}] {id}\n    → {payload}"),
            EqlResult::Deleted { id } =>
                write!(f, "OK  DELETED {id}"),
            EqlResult::Listed(records) => {
                writeln!(f, "OK  {} record(s)", records.len())?;
                for r in records {
                    writeln!(f, "    [{:?}]  {}", r.tier, r.string_id)?;
                }
                Ok(())
            }
            EqlResult::Audited(lines) => {
                writeln!(f, "OK  {} audit entry/entries", lines.len())?;
                for l in lines { writeln!(f, "    {l}")?; }
                Ok(())
            }
        }
    }
}

// ── Executor ──────────────────────────────────────────────────────────────────
pub struct EqlExecutor {
    store:    Store,
    owner_id: String,
    password: String,
    db_path:  String,
}

impl EqlExecutor {
    pub fn open(path: &str, owner_id: &str, password: &str) -> Result<Self, EdisonError> {
        let store = if std::path::Path::new(path).exists() {
            Store::load(path)?
        } else {
            Store::new()
        };
        Ok(Self {
            store,
            owner_id: owner_id.to_string(),
            password: password.to_string(),
            db_path:  path.to_string(),
        })
    }

    pub fn execute(&mut self, stmt: Statement) -> Result<EqlResult, EdisonError> {
        match stmt {
            Statement::Write  { id, tier, payload } => self.exec_write(id, tier, payload),
            Statement::Read   { id }                => self.exec_read(id),
            Statement::List   { tier }              => self.exec_list(tier),
            Statement::Delete { id }                => self.exec_delete(id),
            Statement::Audit  { id }                => self.exec_audit(id),
        }
    }

    fn exec_write(
        &mut self,
        id: String,
        tier: Tier,
        payload: String,
    ) -> Result<EqlResult, EdisonError> {
        let mut salt = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut salt);
        let key       = derive_key(&self.password, &salt)?;
        let envelope  = encode(&id, &payload);
        let encrypted = encrypt_payload(envelope.as_bytes(), &key)?;
        let record    = Record::new(id_hash(&id), to_data_tier(&tier), &self.owner_id, encrypted, salt)?;
        self.store.write(record)?;
        self.store.save(&self.db_path)?;
        Ok(EqlResult::Written { id, tier })
    }

    fn exec_read(&mut self, id: String) -> Result<EqlResult, EdisonError> {
        let id_num = id_hash(&id);
        // Clone needed fields before mutable borrow of store ends
        let (salt, payload, tier) = {
            let record = self.store.read(id_num, &self.owner_id)?;
            (record.salt, record.payload.clone(), record.tier.clone())
        };
        let key       = derive_key(&self.password, &salt)?;
        let decrypted = decrypt_payload(&payload, &key)?;
        let raw       = String::from_utf8(decrypted).map_err(|_| EdisonError::DecryptionFailed)?;
        let (_, data) = decode(&raw);
        Ok(EqlResult::Read { id, tier, payload: data.to_string() })
    }

    fn exec_list(&mut self, tier_filter: Option<Tier>) -> Result<EqlResult, EdisonError> {
        // Collect owned snapshots first — releases the borrow on self.store
        // so we can use self.password for key derivation in the loop.
        let snapshots: Vec<([u8; 32], Vec<u8>, DataTier, u64)> = self.store
            .list_by_owner(&self.owner_id)
            .into_iter()
            .map(|r| (r.salt, r.payload.clone(), r.tier.clone(), r.created_at))
            .collect();

        let mut infos = Vec::new();
        for (salt, payload, tier, created_at) in snapshots {
            if let Some(ref tf) = tier_filter {
                if to_data_tier(tf) != tier { continue; }
            }
            let key       = derive_key(&self.password, &salt)?;
            let decrypted = decrypt_payload(&payload, &key)?;
            let raw       = String::from_utf8(decrypted).map_err(|_| EdisonError::DecryptionFailed)?;
            let (string_id, _) = decode(&raw);
            infos.push(RecordInfo { string_id: string_id.to_string(), tier, created_at });
        }
        Ok(EqlResult::Listed(infos))
    }

    fn exec_delete(&mut self, id: String) -> Result<EqlResult, EdisonError> {
        self.store.delete(id_hash(&id), &self.owner_id)?;
        self.store.save(&self.db_path)?;
        Ok(EqlResult::Deleted { id })
    }

    fn exec_audit(&self, id: Option<String>) -> Result<EqlResult, EdisonError> {
        let lines = self.store
            .audit_entries()
            .iter()
            .filter(|e| match &id {
                Some(filter) => e.record_id == id_hash(filter),
                None         => true,
            })
            .map(|e| format!(
                "t={:10}  {:12?}  record={}  by={}",
                e.timestamp, e.action, e.record_id, e.requester_id
            ))
            .collect();
        Ok(EqlResult::Audited(lines))
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::eql::parse;

    fn fresh(path: &str) -> EqlExecutor {
        let _ = std::fs::remove_file(path);
        EqlExecutor::open(path, "owner", "password").unwrap()
    }

    #[test]
    fn eql_write_and_read_critical() {
        let mut ex = fresh("/tmp/eql_ex_1.redb");
        ex.execute(parse("WRITE k1 TIER CRITICAL top secret").unwrap()).unwrap();
        match ex.execute(parse("READ k1").unwrap()).unwrap() {
            EqlResult::Read { payload, tier, .. } => {
                assert_eq!(payload, "top secret");
                assert_eq!(tier, DataTier::Critical);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn eql_write_and_read_personal() {
        let mut ex = fresh("/tmp/eql_ex_2.redb");
        ex.execute(parse("WRITE note TIER PERSONAL birthday note").unwrap()).unwrap();
        match ex.execute(parse("READ note").unwrap()).unwrap() {
            EqlResult::Read { tier, .. } => assert_eq!(tier, DataTier::Personal),
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn eql_write_and_read_noise() {
        let mut ex = fresh("/tmp/eql_ex_3.redb");
        ex.execute(parse("WRITE log1 TIER NOISE server started").unwrap()).unwrap();
        match ex.execute(parse("READ log1").unwrap()).unwrap() {
            EqlResult::Read { payload, tier, .. } => {
                assert_eq!(payload, "server started");
                assert_eq!(tier, DataTier::Noise);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn eql_wrong_password_fails() {
        let path = "/tmp/eql_ex_4.redb";
        let _ = std::fs::remove_file(path);
        let mut ex1 = EqlExecutor::open(path, "owner", "correct").unwrap();
        ex1.execute(parse("WRITE k1 TIER CRITICAL secret").unwrap()).unwrap();
        let mut ex2 = EqlExecutor::open(path, "owner", "wrong").unwrap();
        assert!(ex2.execute(parse("READ k1").unwrap()).is_err());
    }

    #[test]
    fn eql_duplicate_id_rejected() {
        let mut ex = fresh("/tmp/eql_ex_5.redb");
        ex.execute(parse("WRITE k1 TIER NOISE foo").unwrap()).unwrap();
        assert!(ex.execute(parse("WRITE k1 TIER NOISE bar").unwrap()).is_err());
    }

    #[test]
    fn eql_delete_removes_record() {
        let mut ex = fresh("/tmp/eql_ex_6.redb");
        ex.execute(parse("WRITE k1 TIER PERSONAL data").unwrap()).unwrap();
        ex.execute(parse("DELETE k1").unwrap()).unwrap();
        assert!(ex.execute(parse("READ k1").unwrap()).is_err());
    }

    #[test]
    fn eql_list_all() {
        let mut ex = fresh("/tmp/eql_ex_7.redb");
        ex.execute(parse("WRITE a TIER CRITICAL x").unwrap()).unwrap();
        ex.execute(parse("WRITE b TIER NOISE y").unwrap()).unwrap();
        match ex.execute(parse("LIST").unwrap()).unwrap() {
            EqlResult::Listed(v) => assert_eq!(v.len(), 2),
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn eql_list_tier_filter() {
        let mut ex = fresh("/tmp/eql_ex_8.redb");
        ex.execute(parse("WRITE a TIER CRITICAL x").unwrap()).unwrap();
        ex.execute(parse("WRITE b TIER NOISE y").unwrap()).unwrap();
        match ex.execute(parse("LIST TIER NOISE").unwrap()).unwrap() {
            EqlResult::Listed(v) => {
                assert_eq!(v.len(), 1);
                assert_eq!(v[0].string_id, "b");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn eql_audit_captures_operations() {
        let mut ex = fresh("/tmp/eql_ex_9.redb");
        ex.execute(parse("WRITE k1 TIER PERSONAL data").unwrap()).unwrap();
        ex.execute(parse("READ k1").unwrap()).unwrap();
        match ex.execute(parse("AUDIT").unwrap()).unwrap() {
            EqlResult::Audited(lines) => assert!(lines.len() >= 2),
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn eql_read_nonexistent_fails() {
        let mut ex = fresh("/tmp/eql_ex_10.redb");
        assert!(ex.execute(parse("READ ghost").unwrap()).is_err());
    }
}