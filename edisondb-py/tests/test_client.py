"""EdisonDB Python SDK — Integration Tests

Requires a running EdisonDB server:
  cargo run --bin edisondb-server -- --db /tmp/pytest_test.redb --port 7778
"""

import pytest
from edisondb import EdisonDB, NotFoundError, AlreadyExistsError

BASE = dict(host="localhost", port=7778, owner_id="alice", password="password")

@pytest.fixture(autouse=True)
def db():
    return EdisonDB(**BASE)

def test_ping(db):
    assert db.ping() is True

def test_write_and_read(db):
    db.write("py:1", "PERSONAL", "sovereign python data")
    rec = db.read("py:1")
    assert rec is not None
    assert rec.payload == "sovereign python data"
    assert rec.tier == "personal"

def test_read_nonexistent_returns_none(db):
    assert db.read("py:ghost") is None

def test_duplicate_write_fails(db):
    db.write("py:dup", "NOISE", "first")
    with pytest.raises(AlreadyExistsError):
        db.write("py:dup", "NOISE", "second")

def test_delete(db):
    db.write("py:del", "NOISE", "to delete")
    db.delete("py:del")
    assert db.read("py:del") is None

def test_list_all(db):
    db.write("py:la1", "CRITICAL", "a")
    db.write("py:la2", "NOISE", "b")
    records = db.list()
    ids = [r.id for r in records]
    assert "py:la1" in ids
    assert "py:la2" in ids

def test_list_tier_filter(db):
    db.write("py:tf1", "CRITICAL", "secret")
    db.write("py:tf2", "NOISE", "log")
    critical = db.list(tier="CRITICAL")
    assert all(r.tier == "critical" for r in critical)

def test_status(db):
    s = db.status()
    assert s.record_count >= 0
    assert s.chain_valid is True
    assert s.backend == "redb"

def test_verify(db):
    assert db.verify() is True

def test_audit(db):
    db.write("py:audit", "PERSONAL", "data")
    entries = db.audit(id="py:audit")
    assert len(entries) >= 1
    assert entries[0].record_id == "py:audit"
