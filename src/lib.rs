use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
pub enum DataTier {
    Critical,
    Personal,
    Noise,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record {   
    pub id: u64,
    pub tier: DataTier,
    pub owner_id: String,
    pub payload: Vec<u8>,
    pub created_at: u64,
}

impl Record {
    pub fn new(
        id: u64,
        tier: DataTier,
        owner_id: &str,
        payload: Vec<u8>,
    ) -> Result<Self, EdisonError> {
        if owner_id.is_empty() {
            return Err(EdisonError::NoOwner);
        }
        Ok(Record {
            id,
            tier,
            owner_id: owner_id.to_string(),
            payload,
            created_at: now(),
        })
    }

    pub fn is_readable_by(&self, requester_id: &str) -> bool {
        match self.tier {
            DataTier::Critical => requester_id == self.owner_id,
            DataTier::Personal => requester_id == self.owner_id,
            DataTier::Noise => true,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EdisonError {
    NoOwner,
    AccessDenied,
    NotFound,
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
use std::collections::HashMap;

pub struct Store {
    records: HashMap<u64, Record>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            records: HashMap::new(),
        }
    }

    pub fn write(&mut self, record: Record) {
        self.records.insert(record.id, record);
    }

    pub fn read(
        &self,
        id: u64,
        requester_id: &str,
    ) -> Result<&Record, EdisonError> {
        match self.records.get(&id) {
            None => Err(EdisonError::NotFound),
            Some(record) => {
                if record.is_readable_by(requester_id) {
                    Ok(record)
                } else {
                    Err(EdisonError::AccessDenied)
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn owner_can_read_stored_record() {
        let mut store = Store::new();
        let r = Record::new(10, DataTier::Personal,
            "owner_abc", vec![1,2,3]).unwrap();
        store.write(r);
        assert!(store.read(10, "owner_abc").is_ok());
    }

    #[test]
    fn attacker_cannot_read_stored_record() {
        let mut store = Store::new();
        let r = Record::new(11, DataTier::Critical,
            "owner_abc", vec![1,2,3]).unwrap();
        store.write(r);
        assert_eq!(
            store.read(11, "attacker"),
            Err(EdisonError::AccessDenied)
        );
    }
    #[test]
    fn owner_can_read_critical() {
        let r = Record::new(1, DataTier::Critical,
            "owner_abc", vec![1,2,3]).unwrap();
        assert!(r.is_readable_by("owner_abc"));
    }

    #[test]
    fn non_owner_cannot_read_critical() {
        let r = Record::new(2, DataTier::Critical,
            "owner_abc", vec![1,2,3]).unwrap();
        assert!(!r.is_readable_by("attacker"));
    }

    #[test]
    fn admin_cannot_read_critical() {
        let r = Record::new(3, DataTier::Critical,
            "owner_abc", vec![1,2,3]).unwrap();
        assert!(!r.is_readable_by("admin"));
        assert!(!r.is_readable_by("root"));
    }

    #[test]
    fn noise_readable_by_anyone() {
        let r = Record::new(4, DataTier::Noise,
            "owner_abc", vec![9,8,7]).unwrap();
        assert!(r.is_readable_by("anyone"));
    }

    #[test]
    fn record_without_owner_rejected() {
        let result = Record::new(5, DataTier::Personal,
            "", vec![1]);
        assert_eq!(result, Err(EdisonError::NoOwner));
    }

    #[test]
    fn record_has_timestamp() {
        let r = Record::new(6, DataTier::Personal,
            "owner_abc", vec![]).unwrap();
        assert!(r.created_at > 0);
    }
}
