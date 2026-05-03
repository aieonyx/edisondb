# EdisonDB — Phase 1 Technical Specification
**Version:** 0.1-draft  
**Status:** Active Design  
**Author:** Edison / AIEONYX  
**Last Updated:** 2026-04-18

---

## Preamble

This document formalizes the seven architectural commitments that must be locked before EdisonDB 0.1 Alpha ships. These are not aspirational — they are the minimum viable specification for the system to be honest about what it claims to be.

Each section addresses a gap identified through adversarial review.

---

## Spec 1 — Pseudonymization Proxy for Cross-Tier Keys

### The Problem

This is the only genuinely new architectural problem identified across all external reviews, and it must be solved at the storage engine layer before the Connector ships.

**Scenario:**

```sql
-- user_profile table — Personal tier
CREATE TABLE user_profile (
  id    i64  PRIMARY KEY TIER PERSONAL,
  email text TIER PERSONAL,
  name  text TIER PERSONAL
);

-- orders table — Noise tier (platform can access)
CREATE TABLE orders (
  id      i64 PRIMARY KEY TIER NOISE,
  user_id i64 TIER NOISE,  -- foreign key to user_profile.id
  total   f64 TIER NOISE,
  item    text TIER NOISE
);
```

**The leak:** The platform Connector queries:

```sql
SELECT user_id, total, item FROM orders WHERE total > 50
```

The Connector never sees `email` or `name`. But it receives `user_id = 42` — consistently — across every session, every query, every day. Over time, the platform builds a complete behavioral profile of user 42 using Noise data alone. The `user_id` becomes a **stable cross-session tracking identifier** even though it was never classified as Critical or Personal.

This is precisely how "anonymized" datasets are re-identified in practice. EdisonDB cannot claim data sovereignty while permitting this.

---

### The Solution — Per-Connector Pseudonymization

When a Connector is installed by a platform, the user's EdisonDB generates a **platform-unique cryptographic salt** — a 256-bit random value stored in the user's Critical tier, never transmitted.

Any field that functions as a **cross-tier reference key** — a field in a Noise or Personal record that points to a Personal or Critical record — is **HMAC-derived** before crossing the Connector boundary.

```
Real ID:        42
Platform A salt: [random 256-bit, stored Critical tier]
Platform B salt: [different random 256-bit, stored Critical tier]

Platform A sees: HMAC-SHA256(42, salt_A) = 0x7f3a...  (stable within A)
Platform B sees: HMAC-SHA256(42, salt_B) = 0x2c91...  (stable within B)
Cross-platform:  0x7f3a ≠ 0x2c91  (cannot correlate)
Reverse:         impossible without salt, stored only on user device
```

**Properties:**

- **Intra-platform consistency:** The same real `user_id` always produces the same pseudonym for a given platform. JOINs and WHERE clauses within platform scope work correctly.
- **Cross-platform isolation:** Platform A's pseudonym for user 42 is cryptographically unrelated to Platform B's pseudonym for user 42. Cross-platform user tracking is impossible.
- **Non-reversibility:** Without the user's salt (stored Critical tier, never transmitted), the real ID cannot be recovered.
- **Transparent to queries:** The Connector's query layer performs the derivation automatically. Platform SQL is unchanged.

---

### Implementation Design

```
┌─────────────────────────────────────────────────────────┐
│                 PSEUDONYMIZATION PROXY                  │
│                 (Connector Boundary Layer)              │
│                                                         │
│  Incoming query from platform:                          │
│    SELECT user_id, total FROM orders WHERE total > 50   │
│                                                         │
│  EdisonDB executes query internally with real IDs       │
│                                                         │
│  Result row before crossing boundary:                   │
│    { user_id: 42, total: 89.00 }                        │
│                                                         │
│  Pseudonymization Proxy intercepts:                     │
│    detect: user_id is a cross-tier reference key        │
│    derive: HMAC-SHA256(42, platform_salt) → 0x7f3a...   │
│                                                         │
│  Result row after crossing boundary:                    │
│    { user_id: "0x7f3a...", total: 89.00 }               │
│                                                         │
│  Platform receives pseudonym, never real ID             │
└─────────────────────────────────────────────────────────┘
```

**Cross-tier reference key detection** — EdisonDB identifies a field as a cross-tier reference key when:
1. The field contains a foreign key reference to a record of equal or higher tier classification, AND
2. The field itself is in a lower tier than the record it references

At schema definition time, this relationship is declared explicitly:

```sql
CREATE TABLE orders (
  id      i64 PRIMARY KEY TIER NOISE,
  user_id i64 TIER NOISE REFERENCES user_profile(id) PSEUDONYMIZED,
  total   f64 TIER NOISE,
  item    text TIER NOISE
);
```

The `PSEUDONYMIZED` keyword is the schema-level commitment that this field will be HMAC-derived at the Connector boundary.

**Salt lifecycle:**
- Generated once per Connector installation, stored in user's Critical tier
- Rotatable by user at any time — rotation invalidates platform's historical pseudonyms, breaking behavioral continuity
- Non-exportable — never transmitted in any Connector communication
- Backed up as part of Critical tier encrypted backup, recoverable only with user's master key

---

### What This Enables

A platform using EdisonDB Connector correctly:
- Can analyze aggregate Noise data (purchase totals, post engagement, public activity)
- Can track a user's behavior within their own platform using a consistent pseudonym
- Cannot correlate that user's identity with any other platform
- Cannot reverse the pseudonym to real identity
- Cannot build a cross-platform profile even if two platforms collude — their salts differ

This is the concrete technical answer to: *"How does EdisonDB prevent the platform from building a user profile using just the user_id over time?"*

---

## Spec 2 — Full Tier Enforcement Pipeline

### The Commitment

Tier enforcement is **not** a query filter. It is **not** application middleware. It fires at every stage of the read and write pipeline. No stage can be bypassed independently.

### Write Pipeline

```
WRITE REQUEST
     │
     ▼
┌─────────────────────────────────────────────┐
│  1. PARSE & CLASSIFY                        │
│     Schema lookup → assign tier per field   │
│     AI classifier → verify/override if      │
│     unstructured content detected           │
└────────────────────┬────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────┐
│  2. ENCRYPT AT TIER LEVEL                   │
│     Critical  → AES-256-GCM, user-held key  │
│     Personal  → AES-256-GCM, user-held key  │
│     Noise     → AES-256-GCM, device key     │
│     Tier label encoded into physical record │
└────────────────────┬────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────┐
│  3. STORE                                   │
│     Physical record written with tier tag   │
│     WAL entry signed and encrypted          │
│     Audit log entry created                 │
└─────────────────────────────────────────────┘
```

### Read Pipeline

```
READ REQUEST (from Connector or local)
     │
     ▼
┌─────────────────────────────────────────────┐
│  1. AUTHENTICATE CALLER                     │
│     Who is asking? Local user or Connector? │
│     Connector → role = ProcessorOnly        │
│     Local     → role = DataOwner            │
└────────────────────┬────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────┐
│  2. PLAN & TIER-CHECK QUERY                 │
│     Query planner resolves field tiers      │
│     Connector requesting Critical/Personal  │
│     field → query REJECTED at plan stage    │
│     (never reaches execution)               │
└────────────────────┬────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────┐
│  3. EXECUTE                                 │
│     Query runs against storage engine       │
│     Tier tags on physical records verified  │
│     Decryption keys resolved per tier       │
└────────────────────┬────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────┐
│  4. FILTER RESULT SET                       │
│     Post-execution tier audit               │
│     Any field above caller's permitted tier │
│     is NULL-replaced in result set          │
│     (defense in depth — catches edge cases) │
└────────────────────┬────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────┐
│  5. PSEUDONYMIZATION PROXY                  │
│     Cross-tier reference keys HMAC-derived  │
│     (Spec 1 — fires here, not earlier)      │
└────────────────────┬────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────┐
│  6. SERIALIZE & TRANSMIT                    │
│     Response serialized                     │
│     Audit log: query, caller, timestamp,    │
│     fields accessed, tier of each field     │
│     Transmitted to Connector over TLS 1.3   │
└─────────────────────────────────────────────┘
```

### Defense In Depth

Enforcement fires at **stages 2, 4, and 5** — not once. This means:

- A bug in the query planner (stage 2) that allows a forbidden field through will be caught by the result filter (stage 4) before it leaves the execution engine
- A bug in the result filter (stage 4) will be caught by the serializer audit (stage 6) which can halt transmission
- No single point of failure exposes protected data

The honest claim is: *"Direct access is structurally prevented at multiple independent enforcement points. Indirect inference is actively mitigated by the Aggregation Awareness Layer."*

---

## Spec 3 — SQL Dialect Commitment

### The Commitment

EdisonDB Query Language (EQL) is a **PostgreSQL-compatible dialect** extended with native sovereignty, vector, graph, and AI operations.

This means:
- All valid PostgreSQL SQL is valid EQL
- PostgreSQL window functions, CTEs, recursive CTEs are supported
- PostgreSQL data types are recognized and mapped
- ORM compatibility is through the PostgreSQL wire protocol

### What EQL Adds

```sql
-- Sovereignty extensions
TIER CRITICAL | PERSONAL | NOISE          -- column declaration
PSEUDONYMIZED                             -- cross-tier key declaration

-- Vector extensions  
VECTOR SEARCH 'natural language query'    -- semantic search
SIMILARITY > 0.85                         -- similarity threshold
AUTO EMBED field INTO embedding           -- local embedding trigger

-- Graph extensions (Labeled Property Graph — native storage)
TRAVERSE FROM node('id') DEPTH n          -- graph traversal
WHERE relationship = 'type'               -- edge filter

-- AI extensions
AUTO CLASSIFY content INTO tier           -- AI tier classification
```

### What EQL Does NOT Claim

EQL does not claim 100% ANSI SQL compatibility. It claims PostgreSQL dialect compatibility as the baseline. Deviations from PostgreSQL behavior will be documented in the EQL Language Reference as they are discovered. Unknown behavior defaults to PostgreSQL semantics.

### Wire Protocol

EdisonDB speaks the **PostgreSQL wire protocol** in Server and Cluster modes. This means existing PostgreSQL clients, drivers, and ORMs connect to EdisonDB without modification. This is the fastest path to developer adoption.

---

## Spec 4 — BUSL Connector License Conversion Date

### The Commitment

The EdisonDB Connector is licensed under the **Business Source License 1.1 (BUSL-1.1)**.

The **Change Date is January 1, 2030.**

On that date, the Connector license automatically converts to **Apache License 2.0** — the same license as EdisonDB Core. This conversion is irrevocable and requires no action from AIEONYX or any contributor.

### What This Means In Practice

| Period | Connector License | Commercial Use |
|---|---|---|
| Now → Dec 31, 2029 | BUSL 1.1 | Requires commercial license for revenue-generating platforms |
| Jan 1, 2030 → forever | Apache 2.0 | Free for all use |

**Free use during BUSL period:**
- Personal projects
- Open source projects
- Research and academic use
- Non-revenue-generating applications
- Internal tooling (not customer-facing)

**Commercial license required during BUSL period:**
- SaaS platforms using EdisonDB Connector for paying customers
- Enterprise applications
- Any platform generating revenue through EdisonDB-connected user data

This is not a bait-and-switch. The date is public, fixed, and irrevocable. Community Promise II (Apache 2.0 Core forever) is unaffected.

---

## Spec 5 — Embedding Model Storage Architecture

### The Commitment

The EdisonDB binary is the **engine only**. AI models are **separate artifacts**, managed by the `edctl` model manager.

### Model Storage

```
~/.edisondb/
├── data/           ← database files
├── models/         ← AI model artifacts
│   ├── bge-small-en-v1.5.gguf     ← default embedding model (~67MB quantized)
│   ├── phi-3-mini-4k.gguf         ← default classification model (~2.3GB quantized)
│   └── manifest.json              ← model registry, checksums, versions
├── keys/           ← encrypted key store (Critical tier)
└── config.toml     ← local configuration
```

### Model Lifecycle

```bash
# First use — model downloaded on demand, not bundled
edctl model pull bge-small              # 67MB, default embedding
edctl model pull phi3-mini              # 2.3GB, classification + assistant
edctl model pull llama3.2-1b            # 1.3GB, lightweight alternative

# Bring your own fine-tuned model
edctl model import ./my-fine-tuned.gguf --name my-model
edctl model set-default embedding my-model

# List installed models
edctl model list

# BASTION delegates heavy models, mobile uses quantized
edctl model set-profile mobile          # auto-selects quantized variants
edctl model set-profile bastion         # enables full-size models
```

### Model Profiles

| Profile | Embedding Model | Classification | Memory |
|---|---|---|---|
| `mobile` | bge-small Q4 (~67MB) | Disabled locally, delegates to BASTION | < 128MB |
| `standard` | bge-small Q4 (~67MB) | phi3-mini Q4 (~2.3GB) | < 3GB |
| `bastion` | bge-large Q8 (~670MB) | phi3-mini full | < 6GB |

The binary remains **< 15 MB**. Models are separate, user-managed, checksummed, and portable.

---

## Spec 6 — Property Graph Storage Commitment

### The Commitment

EdisonDB's graph model is a **native Labeled Property Graph** stored at the storage engine layer.

It is **not** a recursive SQL CTE with syntax sugar. It is **not** a view over relational tables. It is a first-class storage primitive alongside the relational, document, and vector models.

### Storage Model

```
Nodes: { id, labels[], properties{} }
Edges: { id, from_node_id, to_node_id, relationship, weight, properties{} }
```

Stored natively in the LSM-tree engine as a separate column family — not as SQL tables. The graph traversal engine reads directly from this column family without going through the SQL query planner.

### Syntax

```sql
-- Create graph entities
INSERT INTO trust_graph VERTEX (id: 'user_001', labels: ['User'], name: 'Edison');
INSERT INTO trust_graph EDGE   (from: 'user_001', to: 'user_002', 
                                 relationship: 'TRUSTS', weight: 0.9);

-- Traverse
SELECT * FROM trust_graph
  TRAVERSE FROM node('user_001') DEPTH 3
  WHERE relationship = 'TRUSTS' AND weight > 0.7;

-- Shortest path
SELECT PATH FROM trust_graph
  BETWEEN node('user_001') AND node('user_099')
  WHERE relationship IN ('TRUSTS', 'KNOWS');
```

### Why This Matters

A recursive CTE implementation would mean graph performance degrades with depth — every hop is a new SQL query. A native storage engine implementation means traversal is O(depth × average_edges) regardless of graph size. At Depth 3 across a trust graph of millions of nodes, this difference is seconds versus milliseconds.

---

## Spec 7 — Crash Reporting and Zero Telemetry Compatibility

### The Commitment

Zero telemetry means **the binary never phones home under any circumstances**. This promise is unconditional. No exception for crashes. No exception for "opt-in anonymous statistics."

### The Tension

Rust panics are rare but possible. Without any reporting mechanism, debugging Alpha-phase crashes requires users to manually capture and share stack traces. This creates friction that kills early contributor momentum.

### The Solution — edctl crash-report

```bash
# After a crash, EdisonDB writes a local crash report
~/.edisondb/crashes/
└── crash_2026-04-18T06-23-11Z.report

# User reads the report first
edctl crash-report read crash_2026-04-18T06-23-11Z.report

# User decides to share it — manually, consciously
edctl crash-report export crash_2026-04-18T06-23-11Z.report
# Produces: crash_redacted.txt — user reviews before sending
# User then manually pastes into GitHub Issue or emails

# Nothing is ever sent automatically
# No background process. No scheduler. No silent upload.
```

### Crash Report Contents

```
EdisonDB Crash Report
=====================
Version:     0.1-alpha
OS:          Linux ARM64
Mode:        Embedded
Timestamp:   2026-04-18T06:23:11Z

Panic message:  [included]
Stack trace:    [included]
Last operation: [included — operation type only, no data values]
Memory usage:   [included]

NOT INCLUDED:
- Database contents
- Any user data
- Any query values
- Any file paths
- Any network addresses
- Any identifiers
```

The report contains **execution context only** — never data content. The user reads it before it goes anywhere. The binary never sends it. Zero telemetry promise is intact.

---

## Spec 8 — Digital Legacy System

### The Human Problem

When a person dies, their data does not die with them under the current internet architecture. Public posts resurface in feeds. Algorithms serve memorial content to followers without consent. Families are forced to submit death certificates to corporations and wait months for profiles to be removed — if they are removed at all. Platforms have financial incentives to keep deceased profiles active because engagement, even grief-driven engagement, generates revenue.

This is not a policy failure. It is an architectural failure. The platform owns the data, so the platform decides what happens to it. The family has no structural standing.

Under EdisonDB's sovereignty model, this problem dissolves by design — not through platform policy, but through the mechanics of how data is stored, classified, and expired.

> *"When a person dies, their data should not outlive their family's consent. Under EdisonDB's model, public posts expire automatically. Private data goes dark when the device does. A family should never have to beg a corporation to let their loved one rest."*

---

### How The Existing Architecture Already Handles This

**Noise tier auto-expiry** addresses public posts immediately. Every public post, reaction, and comment is Noise tier — hosted on the platform server for a maximum of 30 days, then auto-purged by the Connector. When a user's device goes permanently offline after death, no new Noise data flows to the platform. Existing Noise data expires on schedule. The family makes no request. The platform exercises no discretion. The architecture handles it.

**Critical and Personal tiers go dark at device loss.** Private messages, personal photos, location history, and all data above Noise tier lives on the user's device. When the device goes offline permanently, that data becomes inaccessible to any platform. It does not linger. It does not get discovered by an algorithm and resurface three years later.

**What the existing architecture does not yet handle** is the user's ability to pre-designate what happens to their data, by whom, and on what timeline. This is the Digital Legacy system.

---

### Digital Legacy — Phase 2 Specification

#### 8.1 Legacy Key Designation

At any time, a user may designate one or more **Legacy Holders** — trusted persons (family member, legal executor, designated friend) who receive a partial cryptographic recovery key stored in the user's Critical tier.

A Legacy Holder can, upon the user's death:
- Request immediate purge of all Noise tier data from all connected platform Connectors
- Access Personal tier data if the user's Data Testament explicitly permits it
- Transfer the EdisonDB vault to a designated heir
- Trigger permanent deletion of the entire vault

Legacy Holders hold a partial key only — they cannot access the vault unilaterally. A threshold scheme (e.g. 2-of-3 designated holders) prevents single-point abuse.

#### 8.2 Data Testament

A user may pre-define, per tier, what happens to their data after a defined inactivity period or upon a Legacy Holder's death declaration:

```sql
-- Data Testament declaration
CREATE TESTAMENT FOR current_user (
  inactivity_trigger: INTERVAL '180 days',

  ON TRIGGER DO:
    CRITICAL tier  → DELETE immediately,
    PERSONAL tier  → TRANSFER TO legacy_holder('name')
                     OR DELETE IF no_holder_designated,
    NOISE tier     → PURGE from all connected Connectors
                     immediately, regardless of 30-day schedule
);
```

The Testament is stored encrypted in the user's Critical tier. The Connector receives a purge instruction when the trigger fires — it has no ability to refuse or delay.

#### 8.3 Inactivity Trigger

If a user's EdisonDB instance has zero activity for a user-defined period (default: 180 days), the system:

```
Day 0    → Inactivity period begins
Day 90   → Warning notification to designated Legacy Holder
Day 180  → Inactivity trigger fires
           → Platform Connectors receive Noise purge instruction
           → Personal tier locked pending Legacy Holder action
           → Critical tier remains encrypted, awaiting Testament
Day 365  → If no Legacy Holder action: full vault deletion
           (configurable — user may set permanent preservation)
```

The user may disable or extend this at any time while alive. A single login resets the counter.

#### 8.4 Platform Obligation Under GDPR

Under GDPR Article 17 (Right to Erasure), data processors — which is what platforms become under EdisonDB's Connector model — are obligated to honor deletion requests without undue delay. A Legacy Holder acting on behalf of a deceased user's estate is a valid rights holder under GDPR interpretations in multiple EU member states.

When the EdisonDB Connector receives a legacy purge instruction, the platform has no architectural ability to refuse — the Connector auto-purges on schedule regardless of platform action. The legal obligation and the architectural reality align for the first time.

#### 8.5 The Storage and Energy Dividend

This is not only a human rights feature. It is a practical infrastructure benefit for every platform that adopts EdisonDB.

Platforms currently store and serve content from hundreds of millions of inactive or deceased accounts indefinitely. Every stored post, every cached photo, every indexed profile represents ongoing storage cost, energy consumption, and infrastructure overhead. Under EdisonDB's 30-day Noise expiry and Digital Legacy system:

- Inactive account content expires automatically
- Deceased user platform data purges without manual review
- Storage consumed by permanently inactive accounts trends to zero over time
- Content moderation burden for deceased account interactions is eliminated

The energy and storage savings across a platform with hundreds of millions of users are measurable in millions of dollars annually — a concrete cost reduction argument for enterprise platform adoption alongside the legal compliance argument.

---

### Summary — Digital Legacy Status

| Feature | Phase | Status |
|---|---|---|
| Noise tier 30-day auto-expiry | 1 | Already designed — handles public posts automatically |
| Critical/Personal dark-on-offline | 1 | Already designed — device loss = data inaccessibility |
| Legacy Key Designation | 2 | Designed here |
| Data Testament declaration | 2 | Designed here |
| Inactivity Trigger | 2 | Designed here |
| GDPR Article 17 alignment | 2 | Legal basis documented |
| Platform energy/storage dividend | 2 | Benefit documented |

---

## Spec 9 — Regional Compliance Profiles

### The Cultural Reality

Privacy is not a universal value expressed in a single form. It is shaped by law, culture, history, and community. The EU model is built on individual sovereignty and silence. The Filipino model is built on community, visibility, and shared experience. The American model is built on opt-out rather than opt-in. Each reflects genuine values that deserve respect — not correction.

EdisonDB was designed EU-first, which means its defaults reflect the strictest privacy standard on earth. Applied universally without adjustment, those defaults would make EdisonDB genuinely unusable for cultural contexts where open sharing is not a privacy failure — it is a way of life.

A database that imposes its privacy philosophy on users who did not ask for it is not sovereign. It is paternalistic.

Regional Compliance Profiles solve this. They allow EdisonDB to adapt its default behavior to local law and cultural context — while preserving the non-negotiable protections that belong to every human being regardless of jurisdiction.

---

### 9.1 The Non-Negotiables — No Regional Override

Three properties of EdisonDB cannot be changed by any regional profile, any user setting, or any platform configuration. These are universal and absolute:

```
1. CRITICAL TIER IS ALWAYS CRITICAL
   Medical records, biometrics, private communications,
   authentication credentials — these are structurally
   protected everywhere. A Filipino's blood type is as
   sensitive as a German's. A Chinese citizen's private
   messages are as deserving of protection as anyone else's.
   No regional profile lowers Critical tier protection.

2. THE USER IS ALWAYS THE ADMINISTRATOR
   No regional profile gives a platform, government,
   or any third party administrative rights over a
   user's Critical or Personal tier data.
   The Inverted Administration Model is universal.

3. THE AUDIT LOG IS ALWAYS ON
   Regardless of region, the user always has full
   visibility into who accessed what data and when.
   Transparency over one's own data is not a
   European value — it is a human one.
```

---

### 9.2 Regional Profile Structure

A Regional Compliance Profile defines default behavior for Personal and Noise tier handling, consent models, and retention rules — within the bounds of local law:

```rust
RegionalProfile {
  id:                   String,         // "EU_GDPR", "PH_DPA", etc.
  jurisdiction:         Jurisdiction,
  noise_retention:      Duration | Indefinite,
  personal_consent:     ConsentModel,   // PerSession | AtRegistration | OptOut
  erasure_trigger:      ErasureTrigger, // OnInactivity | OnRequest | Manual
  ai_inference:         InferenceScope, // LocalOnly | LocalPlusOptionalCloud
  user_override:        bool,           // always true — user can always go stricter
  stricter_than:        Option<ProfileId>, // profile hierarchy
}
```

**Stricter always wins between profiles.** If a user in the Philippines manually sets their Noise retention to 30 days, the 30-day setting overrides the PH_DPA default of indefinite. Users can always choose stricter settings than their regional default. They cannot choose weaker settings than the law of their jurisdiction requires.

---

### 9.3 Built-In Regional Profiles

| Profile ID | Jurisdiction | Noise Retention | Consent Model | Erasure Trigger |
|---|---|---|---|---|
| `EU_GDPR` | European Union | 30 days | Per session | On inactivity (180 days) |
| `PH_DPA` | Philippines | Indefinite (user choice) | At registration | On explicit request |
| `US_CCPA` | California, USA | Indefinite (user choice) | Opt-out model | On request (45 days) |
| `BR_LGPD` | Brazil | 30 days | Per session | On request |
| `IN_PDPB` | India | 90 days | At registration | On request |
| `SG_PDPA` | Singapore | 90 days | At registration | On request |
| `JP_APPI` | Japan | 90 days | At registration | On request |
| `KR_PIPA` | South Korea | 30 days | Per session | On request |
| `UK_GDPR` | United Kingdom | 30 days | Per session | On inactivity |
| `AU_PRIVACY` | Australia | Indefinite (user choice) | Opt-out model | On request |
| `CUSTOM` | User-defined | User-defined | User-defined | User-defined |

All profiles enforce Critical tier universal protection. All profiles enforce the audit log. All profiles enforce the IAM model.

---

### 9.4 Profile Selection

Profile selection is transparent and user-controlled:

```
On first EdisonDB setup:

  Your device is in: Philippines
  Suggested profile: PH_DPA

  Under this profile:
  → Your medical records and private messages are always
    protected — this never changes regardless of settings.
  → Your public posts stay on the platform as long as
    you want — no forced deletion.
  → You decide what personal information is shared
    and with which platforms.
  → You can switch to stricter settings at any time.

  [ Use Philippines settings ]
  [ Use EU settings (stricter) ]
  [ Customize my own settings ]
```

Profile selection is stored in the user's Personal tier. It can be changed at any time. Platforms do not control profile selection — they declare which profiles their Connector supports in the Connector Manifest, and the user's EdisonDB uses whichever profile applies to the user.

---

### 9.5 Jurisdictional Incompatibilities — An Honest Statement

EdisonDB is designed for jurisdictions that recognize individual data sovereignty as a legal right. Some jurisdictions are fundamentally incompatible with EdisonDB's architecture at the commercial platform level. This is documented honestly here and in public-facing documentation.

**China:**
Three laws create structural incompatibility with the EdisonDB Connector model:

- **National Intelligence Law (2017), Article 7** — requires organizations and individuals to support government intelligence access upon request. EdisonDB's Critical tier has no backdoor, no government override, and no administrative access path. These are cryptographically enforced — not policy-enforced. A Chinese government access request for Critical tier data cannot be honored without the user's master key.

- **OSCCA Encryption Regulations** — require commercially deployed encryption to use state-approved SM-series algorithms. EdisonDB uses AES-256-GCM and standard Western cryptographic primitives. Operating as a commercial platform gateway in China without OSCCA certification and potential algorithm changes is not compliant.

- **Data Security Law (2021)** — subjects "important data" and "core data" to government security reviews. The definition is broad and enforced with significant government discretion.

**Russia:**
- **SORM Laws** — require telecommunications and internet service providers to install FSB-accessible interception equipment. A platform using EdisonDB Connector in Russia as a sovereign data gateway would face regulatory pressure to provide government access that the architecture structurally prevents.

**Other jurisdictions with mandatory government backdoor requirements** face the same incompatibility for the same architectural reason.

---

### 9.6 Personal Use — The Important Distinction

The incompatibilities above apply specifically to **commercial platform deployment** — when EdisonDB operates as a sovereign data gateway between users and platforms at scale.

**For personal, embedded, or local use — with no Connector and no platform integration — EdisonDB functions as a local database with no regulatory surface.** A developer in China using EdisonDB as a local SQLite replacement for a personal project, a researcher storing data locally, a student building a personal application — these use cases do not trigger the commercial deployment conflicts described above.

The distinction is:

```
PERSONAL / EMBEDDED USE (no Connector, no platform gateway):
  → No cross-border data transmission
  → No government access demands at platform level
  → Functions as any local database would
  → Compatible with personal use in all jurisdictions
  → The user is the only party involved

COMMERCIAL PLATFORM DEPLOYMENT (Connector, sovereignty gateway):
  → Creates legal obligations for the platform
  → Structurally prevents government backdoor access
  → Incompatible with jurisdictions requiring
    mandatory government data access
```

EdisonDB welcomes developers from every country for personal and embedded use. The architectural incompatibilities are specific to the commercial sovereign gateway model — and they are incompatibilities because the architecture will not compromise on what it was built to protect.

---

### 9.7 The Values Statement

EdisonDB does not assume that privacy means the same thing everywhere. In some cultures, data sovereignty means aggressive protection and silence. In others, it means the freedom to share widely and the right to be remembered. EdisonDB respects both — because sovereignty means the user decides, not the platform and not the database.

What EdisonDB does not do is build a backdoor for any government, in any jurisdiction, under any legal framework. A database with a government backdoor is not a sovereign database. It is a surveillance tool with a privacy label.

This is not a political position. It is an architectural one. The encryption is mathematical. The protection is structural. It cannot be negotiated away without destroying what the project is.

---

## Summary — All Architectural Commitments

| Spec | Commitment | Phase | Status |
|---|---|---|---|
| 1. Pseudonymization Proxy | HMAC-derived cross-tier keys per Connector | 1 | Designed |
| 2. Tier Enforcement Pipeline | 6-stage, defense-in-depth enforcement | 1 | Designed |
| 3. SQL Dialect | PostgreSQL-compatible, wire protocol | 1 | Committed |
| 4. BUSL Conversion | Apache 2.0 on January 1, 2030 | 1 | Committed |
| 5. Model Storage | Separate artifacts, edctl model manager | 1 | Designed |
| 6. Property Graph | Native LPG at storage engine layer | 1 | Committed |
| 7. Crash Reporting | edctl crash-report, never auto-sends | 1 | Designed |
| 8. Digital Legacy System | Legacy keys, Data Testament, inactivity trigger | 2 | Designed |
| 9. Regional Compliance Profiles | Per-jurisdiction defaults, universal Critical protections | 2 | Designed |

---

## Open Questions — Phase 2

The following questions are valid but intentionally deferred. They do not block Alpha.

**Conflict Resolution Model (Multi-Device Sync)**
When a user edits a Critical document offline on both phone and laptop simultaneously, how does EdisonDB resolve the conflict when BASTION syncs them? Options: Last-Write-Wins (simple, lossy), CRDT (complex, lossless), Version Vectors (middle ground). Decision deferred to Phase 2 when multi-device sync is implemented.

**Aggregation Awareness Formal Policy**
The Aggregation Awareness Layer (Phase 2) needs formal policy definition: k-anonymity threshold, query rate limits, pattern detection rules, and escalation behavior (block, degrade precision, require consent). Deferred pending Phase 1 foundation.

**Key Recovery Architecture**
Lost device recovery, multi-device key rotation, and social recovery (trusted contacts hold partial key shards). Resolved partially by Digital Legacy key designation — full spec in Phase 2.

---

*This specification is a living document. All changes are made in public. Community input welcome at github.com/aieonyx/edisondb/discussions*
