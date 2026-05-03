# EdisonDB Tier Enforcement Engine
**Version:** 0.1-draft  
**Status:** Active Design  
**Author:** Edison / AIEONYX  
**Last Updated:** 2026-04-18  
**Depends On:** TECHNICAL_SPEC_v1.md (Spec 2)

---

## Preamble

Every privacy claim in software eventually arrives at the same question: *where exactly is the enforcement, and can it be bypassed?*

This document answers that question for EdisonDB's Tier Sovereignty system. It defines the complete enforcement pipeline — every stage, every decision point, every defense layer — so that the claim *"storage engine enforcement, not query layer policy"* is not a marketing statement but a verifiable architectural commitment.

The single most important design decision documented here:

> **Tier enforcement fires at the storage engine layer and at multiple independent stages of every pipeline. No single bypass defeats all stages simultaneously. No query, regardless of claimed privileges, can retrieve a Critical tier value for a caller that is not the authenticated data owner.**

---

## 1. The Three Tiers — Formal Definition

### 1.1 Critical Tier

**Definition:** Data whose exposure to any unauthorized party causes irreversible harm — medical, financial, identity, or physical safety consequences.

**Examples:** Biometrics, medical records, private communications, authentication credentials, precise location history, financial account data, government identity numbers.

**Encryption:** AES-256-GCM with **user-held keys**. User-held means the decryption key is derived from the user's master passphrase — it is never stored on the device in plaintext, never transmitted over any network, and never accessible to any application or platform.

**Access rule:** The authenticated data owner only. No exceptions. No escalation path. No administrator override.

**Platform Connector access:** Structurally impossible. The storage engine does not decrypt Critical tier values for any caller whose session context is `ProcessorOnly`. This is enforced at Stage 3 of the read pipeline — the decryption key is never resolved for non-owner callers.

---

### 1.2 Personal Tier

**Definition:** Data whose exposure causes privacy harm, reputational damage, or enables profiling — but whose authorized sharing serves legitimate user interests.

**Examples:** Full name, email address, purchase history, preferences, behavioral data, general location, account relationships.

**Encryption:** AES-256-GCM with **device-held keys** derived from the user's device identity and master key. Accessible to applications the user explicitly trusts.

**Access rule:** Applications with explicit user consent only. Consent is per-application, per-session, revocable at any time. Consent does not carry over between sessions unless explicitly renewed.

**Platform Connector access:** Denied by default. Requires explicit user grant for the specific session. Grant expires when the session closes. Audit log entry created for every Personal tier access.

---

### 1.3 Noise Tier

**Definition:** Data the user has consciously chosen to make public — the act of publishing constitutes consent for platform processing.

**Examples:** Public posts, public reactions, display name, public bio, published content, public engagement metrics.

**Encryption:** AES-256-GCM with **device key** at rest on the user's device. On the platform side (Noise cache), stored with platform-standard encryption.

**Access rule:** Platform Connectors with registered and user-approved access. Subject to 30-day retention limit on platform servers. Auto-purged after 30 days by the Connector scheduler.

**Platform Connector access:** Permitted within session scope. Subject to aggregation awareness monitoring (Phase 2).

---

### 1.4 Tier Assignment

Tier is assigned at **schema definition time** — declared per column by the developer building on EdisonDB:

```sql
CREATE TABLE user_data (
  id              i64  PRIMARY KEY,          -- metadata, no tier (internal only)
  biometrics      blob TIER CRITICAL,        -- never leaves device
  email           text TIER PERSONAL,        -- user-consented access
  display_name    text TIER NOISE,           -- platform-accessible
  -- Cross-tier reference key — pseudonymized at Connector boundary
  account_id      i64  TIER NOISE REFERENCES accounts(id) PSEUDONYMIZED
);
```

Tier is also assignable by the AI classification layer for unstructured content:

```sql
-- AI-assisted tier assignment for free-form content
INSERT INTO notes (content)
  VALUES ('My blood type is O negative and I take metformin daily')
  AUTO CLASSIFY content INTO tier;
  -- AI detects medical content → assigns TIER CRITICAL automatically
```

When schema tier and AI classification disagree, the **higher tier wins**. A field declared Noise but containing content the AI classifies as Critical is stored at Critical tier.

---

## 2. The Write Pipeline

Every write to EdisonDB passes through this pipeline in sequence. Stages cannot be skipped or reordered.

```
WRITE REQUEST (INSERT / UPDATE)
           │
           ▼
┌──────────────────────────────────────────────────────┐
│  STAGE W1 — PARSE AND CLASSIFY                       │
│                                                      │
│  Input:  Raw write request                           │
│  Action: Schema lookup → assign tier per field       │
│          AI classifier → verify content tier         │
│          Conflict resolution: higher tier wins       │
│  Output: Write request with tier assigned per field  │
│                                                      │
│  Failure mode: Unclassifiable content → reject write │
│  with CLASSIFICATION_REQUIRED error                  │
└──────────────────────────┬───────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────┐
│  STAGE W2 — CALLER AUTHORIZATION                     │
│                                                      │
│  Input:  Classified write request + caller identity  │
│  Action: Verify caller has write permission          │
│          for each field's assigned tier              │
│          Platform Connector → write denied entirely  │
│          Local application → check per-field consent │
│          Data owner → write permitted                │
│  Output: Authorized write request                    │
│                                                      │
│  Failure mode: Unauthorized write → reject with      │
│  WRITE_PERMISSION_DENIED error                       │
└──────────────────────────┬───────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────┐
│  STAGE W3 — ENCRYPT AT TIER LEVEL                    │
│                                                      │
│  Critical fields:                                    │
│    Key source: user master key derivation (never     │
│    stored plaintext, never transmitted)              │
│    Algorithm: AES-256-GCM                            │
│    IV: random per field per write                    │
│                                                      │
│  Personal fields:                                    │
│    Key source: device key + user key combination     │
│    Algorithm: AES-256-GCM                            │
│    IV: random per field per write                    │
│                                                      │
│  Noise fields:                                       │
│    Key source: device key                            │
│    Algorithm: AES-256-GCM                            │
│    IV: random per field per write                    │
│                                                      │
│  Tier label encoded into physical record alongside   │
│  encrypted payload — label is itself integrity-      │
│  protected and cannot be altered without detection   │
└──────────────────────────┬───────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────┐
│  STAGE W4 — WRITE TO STORAGE ENGINE                  │
│                                                      │
│  Physical record written with:                       │
│    → Encrypted payload (per field)                   │
│    → Tier label (integrity-protected)                │
│    → Write timestamp                                 │
│    → Record version (MVCC)                           │
│                                                      │
│  WAL entry written simultaneously:                   │
│    → Encrypted WAL record                            │
│    → Signed with write key (tamper-evident)          │
│    → Tier label included in WAL record               │
│                                                      │
│  Audit log entry created:                            │
│    → Caller identity                                 │
│    → Fields written                                  │
│    → Tier of each field                              │
│    → Timestamp                                       │
└──────────────────────────────────────────────────────┘
```

---

## 3. The Read Pipeline

Every read from EdisonDB passes through this pipeline. For Platform Connector reads, all six stages are active. For local application reads, stages 1, 3, 4, and 6 are active.

```
READ REQUEST (SELECT / QUERY)
           │
           ▼
┌──────────────────────────────────────────────────────┐
│  STAGE R1 — AUTHENTICATE CALLER                      │
│                                                      │
│  Who is asking?                                      │
│                                                      │
│  Data Owner (local):                                 │
│    → Full EQL access                                 │
│    → All tier access after key authentication        │
│    → Role: DataOwner                                 │
│                                                      │
│  Platform Connector (remote):                        │
│    → EDQP session token validated                    │
│    → Tier scope resolved from Connector registration │
│    → Role: ProcessorOnly                             │
│                                                      │
│  Trusted Application (local, consented):             │
│    → Consent record validated                        │
│    → Tier scope from consent grant                   │
│    → Role: TrustedApp                                │
│                                                      │
│  Unknown / Unauthenticated:                          │
│    → Rejected immediately                            │
│    → Role: None                                      │
│                                                      │
│  Failure mode: Authentication failure →              │
│  AUTH_FAILED, connection closed                      │
└──────────────────────────┬───────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────┐
│  STAGE R2 — PLAN AND TIER-CHECK QUERY                │
│                                                      │
│  Query planner resolves:                             │
│    → Which tables are referenced                     │
│    → Which fields are requested                      │
│    → Tier of each requested field (schema lookup)    │
│                                                      │
│  Tier authorization check:                           │
│    ProcessorOnly role requesting Critical field →    │
│    REJECT entire query at planning stage             │
│    (query never reaches execution)                   │
│                                                      │
│    ProcessorOnly role requesting Personal field      │
│    without session grant → REJECT at planning stage  │
│                                                      │
│    DataOwner role → all fields permitted             │
│                                                      │
│  Cross-tier join detection:                          │
│    JOIN across tier boundary → flag PSEUDONYMIZED    │
│    fields for Pseudonymization Proxy at Stage R5     │
│                                                      │
│  Failure mode: Tier violation → TIER_VIOLATION error │
│  Query rejected, audit entry created                 │
└──────────────────────────┬───────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────┐
│  STAGE R3 — EXECUTE QUERY                            │
│                                                      │
│  Query executes against storage engine               │
│  Physical records read from LSM-tree / storage       │
│  Tier labels on records verified against expected    │
│    → Label mismatch → INTEGRITY_ERROR, halt          │
│                                                      │
│  Decryption key resolution:                          │
│    Critical fields: key derived from user master key │
│      → ProcessorOnly caller: key NEVER resolved      │
│      → DataOwner caller: key resolved after auth     │
│    Personal fields: device key + user key            │
│      → ProcessorOnly without grant: key not resolved │
│    Noise fields: device key                          │
│      → Always resolved for authenticated callers     │
│                                                      │
│  Result set assembled in memory:                     │
│    Decrypted values for authorized fields only       │
│    Non-authorized fields remain encrypted in memory  │
└──────────────────────────┬───────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────┐
│  STAGE R4 — FILTER RESULT SET                        │
│                                                      │
│  Defense-in-depth audit of result set:               │
│  Every field in result set verified against          │
│  caller's permitted tier scope                       │
│                                                      │
│  Any field above permitted tier:                     │
│    → Value replaced with NULL in result set          │
│    → Tier violation logged to audit                  │
│    → If ProcessorOnly caller: additional alert       │
│                                                      │
│  This stage catches what Stage R2 should have        │
│  caught — it is an independent defense layer,        │
│  not a redundant check. A bug in R2 does not         │
│  propagate past R4.                                  │
└──────────────────────────┬───────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────┐
│  STAGE R5 — PSEUDONYMIZATION PROXY                   │
│  (Platform Connector queries only)                   │
│                                                      │
│  For every field marked PSEUDONYMIZED in schema:     │
│    Real value:   user_id = 42                        │
│    Platform salt: stored Critical tier, never tx'd  │
│    Derived value: HMAC-SHA256(42, platform_salt)     │
│                 → 0x7f3a9c... (stable within         │
│                   platform, non-reversible)          │
│                                                      │
│  Platform receives: 0x7f3a9c... (consistent)         │
│  Platform cannot: reconstruct 42 from 0x7f3a9c...   │
│  Platform cannot: correlate with other platforms     │
│                  (each platform has different salt)  │
│                                                      │
│  Not applied to DataOwner queries — owner sees       │
│  real values always                                  │
└──────────────────────────┬───────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────┐
│  STAGE R6 — SERIALIZE, AUDIT, TRANSMIT               │
│                                                      │
│  Result serialized for transmission                  │
│                                                      │
│  Pre-transmission audit:                             │
│    → Verify no Critical values in payload            │
│    → Verify no unpseudonimized cross-tier keys       │
│    → If either check fails: HALT transmission        │
│      audit entry created, error returned to caller   │
│                                                      │
│  Audit log entry written (Critical tier):            │
│    → Platform identity (Connector ID)                │
│    → Session token (hashed)                          │
│    → Tables accessed                                 │
│    → Fields accessed + tier of each                  │
│    → Row count returned                              │
│    → Timestamp                                       │
│    → Any tier violations detected                    │
│                                                      │
│  Transmission over TLS 1.3 (Connector queries)       │
│  or local IPC (local application queries)            │
└──────────────────────────────────────────────────────┘
```

---

## 4. Defense In Depth — Why Six Stages

A single enforcement point is a single point of failure. If a bug exists in the query planner (Stage R2), a malicious or malformed query might slip through. Defense in depth means multiple independent enforcement points — a bug in one stage does not compromise the system.

```
Stage R2 catches:   Tier violations at query planning
                    (before execution — most efficient)

Stage R4 catches:   What R2 missed
                    (after execution — independent check)

Stage R6 catches:   What R4 missed
                    (before transmission — final gate)
```

For a Critical tier value to reach a Platform Connector, three independent enforcement stages must all fail simultaneously. Each stage is implemented independently — different code paths, different data structures, different validation logic. The probability of all three failing simultaneously for the same field in the same query is cryptographically negligible.

---

## 5. Tier Enforcement and the Storage Engine

The most important architectural commitment in this document:

**Tier enforcement is not implemented as:**
- SQL triggers (bypassable with DISABLE TRIGGER)
- Row-level security policies (bypassable with superuser)
- Application middleware (bypassable by going around the application)
- View-based access control (bypassable by querying base tables)

**Tier enforcement is implemented as:**
- A physical property of every stored record (tier label encoded into the record)
- A key resolution policy in the storage engine (Critical key never resolved for non-owner)
- A serialization gate at transmission (pre-transmission field audit)

This means there is no SQL command, no configuration flag, no administrator password, and no privilege level that bypasses tier enforcement for a non-owner caller trying to access Critical tier data. The encryption key is never derived for that caller. The data remains encrypted. The plaintext never exists in memory for that caller.

---

## 6. Organizational Deployment — RBAC Interaction

When EdisonDB is deployed in organizational mode (school, hospital, enterprise), RBAC roles interact with the tier system as follows:

```
RBAC Role          Tier Access      Notes
─────────────────────────────────────────────────────
db_admin           All tiers        Only for DataOwner context
                                    Not applicable to Connectors

teacher            Personal + Noise Per schema policy
                                    Row-level policies restrict
                                    to authorized records only

student            Noise only       Own records via row policy
                   Own Personal     Cannot see other students

auditor            Audit log only   Tier labels visible
                                    Field values not decrypted

platform_connector Noise only       Standard Connector scope
                                    No RBAC elevation possible
```

RBAC roles can restrict access further than the tier system — a student role cannot see all Noise tier data, only rows the row-level security policy permits. RBAC roles cannot expand access beyond what the tier system permits — no RBAC grant gives a Connector access to Critical data.

The relationship is: **tier system sets the ceiling, RBAC sets the floor.**

---

## 7. Tier Migration

Data can move between tiers under specific controlled conditions:

**Upward migration (Noise → Personal → Critical):** Always permitted by the data owner. No approval required. Effective immediately on next write.

**Downward migration (Critical → Personal → Noise):** Requires explicit data owner action with confirmation. Cannot be triggered by any application or platform. The user must explicitly approve tier reduction.

**Aggregation-triggered escalation (Phase 2):** When the Aggregation Awareness Layer detects that a pattern of Noise queries is reconstructing Personal or Critical information, it escalates the effective tier of the combined result. The individual records remain at their declared tiers — only the aggregated result is elevated for the duration of the suspicious query pattern.

---

## 8. What This Document Guarantees

Reading this document, a developer, auditor, or regulator should be able to confirm:

✓ Tier enforcement fires at the storage engine layer — not only at the query layer  
✓ Critical tier values are never decrypted for non-owner callers — the key is never resolved  
✓ Three independent enforcement stages exist for read pipelines — R2, R4, R6  
✓ Pseudonymization fires at Stage R5 — before serialization, after filtering  
✓ Every read and write produces an audit entry in the user's own Critical tier storage  
✓ RBAC cannot elevate access beyond tier system limits  
✓ Tier labels are integrity-protected in physical records — tampering is detectable  
✓ There is no administrator override, no superuser bypass, no configuration flag that defeats tier enforcement for non-owner callers  

---

*This specification is a living document. All changes are made in public.*  
*Community input welcome at github.com/aieonyx/edisondb/discussions*
