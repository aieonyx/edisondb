# EdisonDB Query Protocol (EDQP)
**Version:** 0.1-draft  
**Status:** Active Design  
**Author:** Edison / AIEONYX  
**Last Updated:** 2026-04-18  
**Depends On:** TECHNICAL_SPEC_v1.md (Spec 1, 2, 3)

---

## Preamble

Every database claim about platform integration eventually arrives at the same hard question: *how, exactly, does the platform talk to the user's database?*

This document answers that question formally. The EdisonDB Query Protocol (EDQP) is the specification for how external platforms communicate with a user's EdisonDB instance through the Connector. It defines what requests are permitted, how they are authenticated, where query execution happens, how results flow, and what is structurally prevented from ever crossing the boundary.

EDQP is not a general-purpose database protocol. It is a sovereignty-enforcing gateway protocol. Every design decision in it prioritizes user data protection over platform convenience. Where those two values conflict, user protection wins.

---

## 1. Overview and Scope

### 1.1 What EDQP Governs

EDQP governs all communication between:

- A **Platform Connector** (installed on a platform's backend server)
- A **User EdisonDB Instance** (running on the user's device or BASTION node)

EDQP does not govern:
- Communication between the user's own devices (handled by the personal mesh sync protocol)
- Communication between EdisonDB instances in a cluster deployment (handled by the Raft consensus protocol)
- Direct application queries to a locally running EdisonDB in Standard or Embedded mode (handled by the PostgreSQL wire protocol)

### 1.2 The Fundamental Execution Model

This is the question ChatGPT's review correctly identified as critical. EDQP answers it with a clear commitment:

**Queries execute on the user's EdisonDB instance. Results — filtered by tier enforcement — are returned to the platform. Raw data above Noise tier never travels.**

```
┌──────────────────────────────────────────────────────────┐
│                  EXECUTION MODEL                         │
│                                                          │
│  Platform sends:    Query (EQL subset — Noise tier only) │
│                          │                              │
│                          ▼                              │
│  User EdisonDB:     Receives query                       │
│                     Executes against local data          │
│                     Applies tier enforcement             │
│                     Applies pseudonymization proxy       │
│                     Returns filtered result set          │
│                          │                              │
│                          ▼                              │
│  Platform receives: Noise tier results only              │
│                     Pseudonymized cross-tier keys        │
│                     No raw Personal or Critical values   │
│                                                          │
│  Raw data location: Never leaves user's device           │
└──────────────────────────────────────────────────────────┘
```

This means computation happens at the source — the user's device. The platform receives query results, not raw data. This is the architectural inversion that makes sovereignty real rather than promised.

---

## 2. Protocol Architecture

### 2.1 Connection Lifecycle

```
PLATFORM CONNECTOR                    USER EDISONDB
       │                                    │
       │──── 1. HELLO (connector_id) ──────▶│
       │                                    │ Verify connector_id
       │                                    │ Check Connector registry
       │◀─── 2. CHALLENGE (nonce) ──────────│
       │                                    │
       │ Sign nonce with Connector key      │
       │──── 3. AUTH (signed_nonce) ────────▶│
       │                                    │ Verify signature
       │                                    │ Resolve platform tier permissions
       │                                    │ Generate rotating session token
       │◀─── 4. SESSION (token, expiry) ────│
       │                                    │
       │ [SESSION ACTIVE]                   │
       │                                    │
       │──── 5. QUERY (token, EQL) ─────────▶│
       │◀─── 6. RESULT (filtered data) ─────│
       │                                    │
       │ [Repeat 5-6 within session]        │
       │                                    │
       │──── 7. CLOSE ──────────────────────▶│
       │◀─── 8. ACK + AUDIT_ENTRY ──────────│
       │                                    │
       [SESSION CLOSED — token invalidated]
```

**Step-by-step:**

**Step 1 — HELLO:** The Connector identifies itself with its registered `connector_id` — a stable identifier assigned when the Connector was installed on the platform. This is not a user identifier. It identifies the platform, not the person.

**Step 2 — CHALLENGE:** The user's EdisonDB responds with a cryptographic nonce (random one-time value). This prevents replay attacks.

**Step 3 — AUTH:** The Connector signs the nonce with its private key. The platform cannot forge this — it requires the private key stored securely in the Connector installation.

**Step 4 — SESSION:** The user's EdisonDB verifies the signature, looks up the platform's registered tier permissions, and issues a **rotating session token** — valid for this session only, never reused, not tied to a persistent device or user identifier.

**Steps 5–6 — QUERY / RESULT:** The active session permits EQL queries within the platform's permitted tier scope. Each query is executed locally, filtered, pseudonymized, and returned.

**Steps 7–8 — CLOSE / ACK:** The session closes cleanly. The user's EdisonDB writes an audit log entry covering the session: platform identity, session duration, fields accessed, tier of each field accessed, timestamp. The session token is invalidated and cannot be reused.

---

### 2.2 Session Token Properties

Session tokens in EDQP have strict properties that enforce the zero-fingerprint commitment:

| Property | Value | Reason |
|---|---|---|
| **Lifetime** | Single session only | Prevents cross-session behavioral tracking |
| **Binding** | Token ≠ user identity | Platform cannot correlate token to real user |
| **Rotation** | New token every session | Same user, different token each time |
| **Entropy** | 256-bit random | Cryptographically unguessable |
| **Revocability** | Immediate on CLOSE | No lingering access after session ends |
| **Cross-platform isolation** | Each platform gets different derivation | Platform A's token reveals nothing about Platform B's sessions |

---

## 3. Query Scope and Permissions

### 3.1 What Platforms Can Query

A platform Connector is registered with a fixed tier permission scope at installation time. This scope cannot be elevated at runtime — not by the platform, not by a query, not by any parameter.

**Default Connector scope:**

```
tier_access:    [Noise]          — only Noise tier fields
query_type:     [SELECT]         — read only, no mutations
retention:      30 days          — Noise cache on platform side
aggregation:    permitted        — aggregate Noise data freely
cross_join:     restricted       — cross-tier joins return pseudonymized keys only
```

**Extended scope (requires explicit user grant, per-session):**

```
tier_access:    [Noise, Personal] — user grants Personal access for specific session
query_type:     [SELECT]          — still read only
duration:       session only      — grant expires when session closes
audit:          elevated logging  — Personal tier access logged with higher detail
```

Critical tier access cannot be granted to any Connector under any circumstances. This is not a permission level. It is a structural impossibility — Critical tier values are decrypted only with user-held keys that are never transmitted.

### 3.2 EQL Subset Available to Connectors

Connectors may use a defined subset of EQL. The full EQL language is available to local applications and the authenticated data owner. The Connector EQL subset is:

**Permitted:**
```sql
SELECT [noise_tier_fields] FROM [table]
  WHERE [conditions on noise_tier_fields]
  ORDER BY [noise_tier_fields]
  LIMIT n;

-- Aggregation over Noise tier
SELECT COUNT(*), AVG(engagement_score)
  FROM user_activity
  WHERE tier = NOISE;

-- Vector search within Noise tier
SELECT * FROM public_posts
  VECTOR SEARCH 'topic query'
  LIMIT 10 SIMILARITY > 0.7;
```

**Not permitted (rejected at query planning stage):**
```sql
-- Accessing Personal tier fields — REJECTED
SELECT email, full_name FROM user_profile;

-- Accessing Critical tier fields — REJECTED
SELECT biometrics, medical_id FROM user_profile;

-- INSERT, UPDATE, DELETE — REJECTED (Connectors are read-only)
INSERT INTO user_data VALUES (...);
UPDATE user_profile SET email = '...';
DELETE FROM user_records WHERE id = 42;

-- Schema operations — REJECTED
CREATE TABLE ...; DROP TABLE ...; ALTER TABLE ...;

-- Raw ID without pseudonymization — handled automatically
-- (user_id is PSEUDONYMIZED in schema — Connector receives derived value)
```

### 3.3 Query Rejection Response

When a query is rejected for tier violation, the Connector receives a structured error — not a generic database error — that explains the rejection without revealing anything about the data:

```json
{
  "status": "REJECTED",
  "code": "TIER_VIOLATION",
  "message": "Query requests fields above permitted tier scope.",
  "permitted_tier": "NOISE",
  "requested_tier": "PERSONAL",
  "suggestion": "Request user grant for Personal tier access if required."
}
```

The error does not reveal the field values, the schema structure beyond what the Connector already knows, or any information that could assist inference attacks.

---

## 4. Result Serialization

### 4.1 What Leaves the User's Device

Before any result set is serialized and transmitted to the platform, it passes through two final enforcement stages:

**Stage 1 — Tier Audit:** Every field in the result set is verified against the session's permitted tier scope. Any field above the permitted tier is replaced with `NULL` in the result. This is defense-in-depth — the query planner should have caught this already, but the serializer provides a final independent check.

**Stage 2 — Pseudonymization Proxy:** Any field marked `PSEUDONYMIZED` in the schema (cross-tier reference keys) is HMAC-derived with the platform's salt before serialization. The platform receives the derived value, never the real value. See TECHNICAL_SPEC_v1.md Spec 1 for the full pseudonymization design.

**Stage 3 — Serialization:** The filtered, pseudonymized result is serialized and transmitted over TLS 1.3. The transmission includes:
- Result rows
- Field names
- Execution metadata (query time, rows returned)
- Session token (for multi-query sessions)

The transmission never includes:
- Any field value above the permitted tier
- The real value of any pseudonymized key
- Any user identity information
- Any device identifier
- Any persistent session correlation identifier

### 4.2 Audit Log Entry

Every completed query generates an audit log entry stored in the user's EdisonDB (Critical tier — the user owns their own audit log):

```
AUDIT ENTRY
===========
Timestamp:        2026-04-18T06:23:11Z
Platform:         [connector_id — platform identity, not user identity]
Session token:    [hashed — token itself not stored after session close]
Query type:       SELECT
Tables accessed:  [table names]
Fields accessed:  [field names + tier of each]
Rows returned:    [count only — not the actual rows]
Tier violations:  [any rejected field requests]
Session duration: [seconds]
```

The user can query their own audit log at any time:

```sql
-- User reviewing their own audit history
SELECT * FROM edisondb_audit_log
  WHERE platform_id = 'facebook_connector'
  ORDER BY timestamp DESC
  LIMIT 100;
```

This gives users complete visibility into who accessed what, when, and what tier — something no existing database architecture provides to end users.

---

## 5. Connector Registration

### 5.1 How a Platform Registers Its Connector

Before a platform can establish any EDQP session with any user, the platform must publish a **Connector Manifest** — a signed, public document that declares:

```json
{
  "connector_id": "platform-name-v1",
  "platform_name": "Example Platform",
  "platform_url": "https://example.com",
  "requested_tier_scope": ["NOISE"],
  "retention_days": 30,
  "audit_contact": "privacy@example.com",
  "connector_public_key": "-----BEGIN PUBLIC KEY-----...",
  "manifest_version": "1.0",
  "signed_by": "connector_private_key"
}
```

This manifest is:
- Published publicly (users and auditors can inspect it)
- Signed by the platform's private key (tamper-evident)
- Versioned (changes to scope require a new manifest version and re-approval)

### 5.2 User Connector Approval

A user's EdisonDB does not accept connections from unregistered Connectors. The first time a platform's Connector attempts a HELLO handshake with a user's EdisonDB, the user receives an approval prompt:

```
Platform "Example Platform" is requesting access to your EdisonDB.

Requested access:
  → Noise tier only (public posts, display names)
  → Read-only
  → 30-day retention on platform servers
  → Auto-purge after 30 days

Approve / Deny / Approve for this session only
```

Once approved, the approval is stored in the user's Personal tier. The user can revoke it at any time from their EdisonDB settings. Revocation takes effect on the next HELLO attempt.

---

## 6. The Digital Legacy Protocol Extension

When a user's inactivity trigger fires (see TECHNICAL_SPEC_v1.md Spec 8), EDQP includes a **Legacy Purge Instruction** — a special signed message sent to all registered platform Connectors:

```
LEGACY_PURGE_INSTRUCTION
=========================
Issued by:    EdisonDB Digital Legacy System
User vault:   [vault_id — opaque, not user-identifiable]
Instruction:  PURGE ALL NOISE TIER DATA
Scope:        All records associated with this vault's session history
Effective:    Immediately upon receipt
Signed:       [legacy_holder_key + vault_key threshold signature]
```

Upon receiving a valid Legacy Purge Instruction, the Connector:
1. Deletes all Noise tier data associated with this vault's pseudonymized identifiers
2. Invalidates all session tokens associated with this vault
3. Removes the Connector registration for this vault
4. Returns a signed deletion confirmation to the vault's audit log

The platform cannot reject a valid Legacy Purge Instruction. The Connector is architecturally designed to honor it without requiring human review or platform approval.

---

## 7. Error Codes

| Code | Meaning | Platform Action |
|---|---|---|
| `AUTH_FAILED` | Signature verification failed | Recheck Connector key configuration |
| `CONNECTOR_NOT_REGISTERED` | Platform not approved by user | Request user approval first |
| `TIER_VIOLATION` | Query requests data above permitted tier | Reduce query scope to Noise tier |
| `SESSION_EXPIRED` | Session token no longer valid | Re-authenticate with new HELLO |
| `RATE_LIMITED` | Query rate exceeds permitted threshold | Implement backoff, reduce query frequency |
| `VAULT_OFFLINE` | User device is not reachable | Retry when user comes online |
| `CONSENT_REVOKED` | User revoked Connector approval | Cease all connection attempts |
| `LEGACY_PURGE_ACTIVE` | User vault is in legacy purge state | Honor purge, delete all local Noise cache |

---

## 8. Security Properties

EDQP provides the following security guarantees:

| Property | Guarantee |
|---|---|
| **No persistent identity** | Rotating session tokens prevent cross-session fingerprinting |
| **No raw data transmission** | Only Noise tier filtered results leave the user's device |
| **No backdoor access** | Critical tier is structurally inaccessible — not policy-protected |
| **Tamper-evident** | All sessions produce signed audit entries |
| **Forward secrecy** | Session keys are ephemeral — past sessions cannot be decrypted |
| **User visibility** | Users can query their complete access audit log at any time |
| **Consent revocability** | User can revoke any Connector approval at any time, effective immediately |
| **Legacy protection** | Digital Legacy purge instructions cannot be refused by a Connector |

---

## 9. Relationship to Other Specs

| Spec | Relationship |
|---|---|
| TECHNICAL_SPEC Spec 1 (Pseudonymization) | EDQP calls the Pseudonymization Proxy at Stage 2 of result serialization |
| TECHNICAL_SPEC Spec 2 (Tier Enforcement) | EDQP queries are subject to the full 6-stage tier enforcement pipeline |
| TECHNICAL_SPEC Spec 3 (SQL Dialect) | EDQP queries use the Connector EQL subset — a restricted form of the full EQL |
| TECHNICAL_SPEC Spec 8 (Digital Legacy) | EDQP includes the Legacy Purge Instruction as a protocol-level message type |
| TIER_ENFORCEMENT.md | The enforcement pipeline that every EDQP query passes through |

---

*This specification is a living document. All changes are made in public.*  
*Community input welcome at github.com/aieonyx/edisondb/discussions*
