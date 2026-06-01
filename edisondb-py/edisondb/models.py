"""EdisonDB Python SDK — Data Models"""

from dataclasses import dataclass


@dataclass
class Record:
    """A decrypted record returned from EdisonDB."""
    id: str
    tier: str
    payload: str
    created_at: int


@dataclass
class AuditEntry:
    """A single entry in the EdisonDB audit log."""
    record_id: str
    requester_id: str
    action: str
    timestamp: int


@dataclass
class DbStatus:
    """Database statistics returned by EdisonDB."""
    record_count: int
    audit_count: int
    critical_count: int
    personal_count: int
    noise_count: int
    chain_valid: bool
    backend: str
