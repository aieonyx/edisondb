"""EdisonDB Python SDK

The official Python client for EdisonDB — the sovereign, AI-native,
multi-model database engine.

Usage:
    from edisondb import EdisonDB

    db = EdisonDB(host="localhost", port=7777,
                  owner_id="alice", password="secret")
    db.write("note:1", "PERSONAL", "sovereign data")
    record = db.read("note:1")
"""

from .client import EdisonDB
from .models import Record, AuditEntry, DbStatus
from .exceptions import (
    EdisonDBError,
    NotFoundError,
    AccessDeniedError,
    AlreadyExistsError,
    AuditChainError,
    AuthError,
)

__version__ = "0.1.0a2"
__all__ = [
    "EdisonDB",
    "Record",
    "AuditEntry",
    "DbStatus",
    "EdisonDBError",
    "NotFoundError",
    "AccessDeniedError",
    "AlreadyExistsError",
    "AuditChainError",
    "AuthError",
]
