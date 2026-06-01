"""EdisonDB Python SDK — Client"""

from __future__ import annotations
from typing import List, Optional
import urllib.request
import urllib.error
import json

from .models import Record, AuditEntry, DbStatus
from .exceptions import (
    EdisonDBError, NotFoundError, AccessDeniedError,
    AlreadyExistsError, AuditChainError, AuthError,
)


class EdisonDB:
    """
    EdisonDB Python client.

    Connects to a running EdisonDB REST server and provides a clean
    Pythonic API for sovereign encrypted data operations.

    Example:
        >>> db = EdisonDB(host="localhost", port=7777,
        ...               owner_id="alice", password="secret")
        >>> db.write("note:1", "PERSONAL", "my sovereign note")
        >>> record = db.read("note:1")
        >>> print(record.payload)
        my sovereign note
    """

    def __init__(
        self,
        host: str = "localhost",
        port: int = 7777,
        owner_id: str = "",
        password: str = "",
        timeout: int = 10,
    ) -> None:
        self._base = f"http://{host}:{port}"
        self._owner_id = owner_id
        self._password = password
        self._timeout = timeout

    # -- Internal helpers ----------------------------------------------------

    def _headers(self) -> dict:
        return {
            "Content-Type":  "application/json",
            "X-Owner-ID":    self._owner_id,
            "X-Password":    self._password,
        }

    def _request(self, method: str, path: str, body: Optional[dict] = None) -> dict:
        url = self._base + path
        data = json.dumps(body).encode() if body else None
        headers = self._headers()
        req = urllib.request.Request(url, data=data, headers=headers, method=method)
        try:
            with urllib.request.urlopen(req, timeout=self._timeout) as resp:
                return json.loads(resp.read())
        except urllib.error.HTTPError as e:
            raw = json.loads(e.read())
            msg = raw.get("error", str(e))
            if e.code == 401:
                raise AuthError(msg)
            if e.code == 404 or "not found" in msg.lower():
                raise NotFoundError(msg)
            if "access denied" in msg.lower():
                raise AccessDeniedError(msg)
            if "already exists" in msg.lower():
                raise AlreadyExistsError(msg)
            raise EdisonDBError(msg)
        except Exception as e:
            raise EdisonDBError(str(e))

    # -- Public API ----------------------------------------------------------

    def ping(self) -> bool:
        """Check if the server is reachable. Returns True if healthy."""
        try:
            req = urllib.request.Request(self._base + "/health")
            with urllib.request.urlopen(req, timeout=self._timeout) as resp:
                data = json.loads(resp.read())
                return data.get("status") == "ok"
        except Exception:
            return False

    def write(self, id: str, tier: str, payload: str) -> None:
        """
        Write a new encrypted record.

        Args:
            id:      Unique record identifier (e.g. "user:1").
            tier:    Data tier — "CRITICAL", "PERSONAL", or "NOISE".
            payload: Plaintext data to encrypt and store.

        Raises:
            AlreadyExistsError: If the ID is already taken.
            AuthError:          If credentials are invalid.
        """
        self._request("POST", "/api/write", {
            "id": id, "tier": tier, "payload": payload
        })

    def read(self, id: str) -> Optional[Record]:
        """
        Read and decrypt a record by ID.

        Returns None if the record does not exist.

        Raises:
            AccessDeniedError: If the caller is not the owner.
            AuthError:         If credentials are invalid.
        """
        try:
            data = self._request("GET", f"/api/read/{id}")
            return Record(
                id=data["id"],
                tier=data["tier"],
                payload=data["payload"],
                created_at=data.get("created_at", 0),
            )
        except NotFoundError:
            return None

    def list(self, tier: Optional[str] = None) -> List[Record]:
        """
        List all records owned by this client.

        Args:
            tier: Optional filter — "CRITICAL", "PERSONAL", or "NOISE".

        Returns:
            List of Record objects (payload is empty for list results).
        """
        path = "/api/list"
        if tier:
            path += f"?tier={tier.upper()}"
        data = self._request("GET", path)
        return [
            Record(
                id=r["id"],
                tier=r["tier"],
                payload="",
                created_at=r.get("created_at", 0),
            )
            for r in data.get("records", [])
        ]

    def delete(self, id: str) -> None:
        """
        Delete a record by ID.

        Raises:
            NotFoundError:     If the record does not exist.
            AccessDeniedError: If the caller is not the owner.
        """
        self._request("DELETE", f"/api/delete/{id}")

    def audit(self, id: Optional[str] = None) -> List[AuditEntry]:
        """
        Retrieve the audit log.

        Args:
            id: Optional record ID filter.

        Returns:
            List of AuditEntry objects.
        """
        path = "/api/audit"
        if id:
            path += f"?id={id}"
        data = self._request("GET", path)
        return [
            AuditEntry(
                record_id=e["record_id"],
                requester_id=e["requester_id"],
                action=e["action"],
                timestamp=e["timestamp"],
            )
            for e in data.get("entries", [])
        ]

    def status(self) -> DbStatus:
        """Return database statistics."""
        data = self._request("GET", "/api/status")
        return DbStatus(
            record_count=data["record_count"],
            audit_count=data["audit_count"],
            critical_count=data["critical_count"],
            personal_count=data["personal_count"],
            noise_count=data["noise_count"],
            chain_valid=data["chain_valid"],
            backend=data["backend"],
        )

    def verify(self) -> bool:
        """
        Verify audit chain integrity.

        Returns True if the chain is intact, False if tampered.
        """
        data = self._request("GET", "/api/verify")
        return data.get("chain_valid", False)
