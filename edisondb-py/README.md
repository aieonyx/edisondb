# EdisonDB Python SDK

The official Python client for EdisonDB.

## Install

```bash
pip install edisondb
```

## Quick Start

```python
from edisondb import EdisonDB

db = EdisonDB(host="localhost", port=7777, owner_id="alice", password="secret")

# Write
db.write("user:1", "PERSONAL", "sovereign data")

# Read
record = db.read("user:1")
print(record.payload)  # sovereign data

# List
records = db.list(tier="PERSONAL")

# Delete
db.delete("user:1")

# Status
status = db.status()
print(status.chain_valid)  # True

# Verify audit chain
db.verify()
```

## Requirements

- Python 3.9+
- A running EdisonDB server (`edisondb-server --db myapp.redb --port 7777`)

## License

Apache 2.0 — © 2026 AIEONYX / Edison Lepiten
