<div align="center">

<img src="https://img.shields.io/badge/EdisonDB-v0.1--alpha-gold?style=for-the-badge" alt="version"/>
<img src="https://img.shields.io/badge/License-Apache%202.0-blue?style=for-the-badge" alt="license"/>
<img src="https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust" alt="rust"/>
<img src="https://img.shields.io/badge/Status-Active%20Development-green?style=for-the-badge" alt="status"/>
<img src="https://img.shields.io/badge/Telemetry-Zero-red?style=for-the-badge" alt="zero telemetry"/>

<br/><br/>

# EdisonDB

### *"Light for your data."*

**The sovereign, AI-native, multi-model database engine.**  
Built in Rust. Encrypted by default. Yours forever.

<br/>

**S4+i** &nbsp;·&nbsp; Security &nbsp;·&nbsp; Speed &nbsp;·&nbsp; Sovereignty &nbsp;·&nbsp; Simplicity &nbsp;·&nbsp; Intelligence

<br/>

[Documentation](#documentation) · [Quick Start](#quick-start) · [Community Promise](#community-promise) · [Roadmap](#roadmap) · [Contributing](#contributing)

</div>

---

## What is EdisonDB?

EdisonDB is a **sovereign, AI-native, multi-model database engine** — built from the ground up in Rust, designed for a world where your data belongs to you and intelligence belongs inside the engine, not in some external API.

Unlike every major database that came before it:

| Other databases | EdisonDB |
|---|---|
| Bolt AI on as an afterthought | AI is woven into the storage engine itself |
| Encryption is a feature you configure | Encryption is a property of existence — always on |
| Require cloud connectivity for intelligence | All AI inference runs locally, on your machine |
| Change licenses when profit demands it | Apache 2.0 forever — legally irrevocable |
| Call home to measure your usage | Zero telemetry. Zero exceptions. |

EdisonDB is **not** trying to replace PostgreSQL for existing workloads. It is building what a database looks like when you start from scratch in 2026, with AI, security, and sovereignty as first principles — not afterthoughts.

---

## The S4+i Philosophy

Every decision in EdisonDB — from encryption algorithm to error message wording — is evaluated against the **S4+i framework**. This is not a marketing tagline. It is an architectural contract.

```
🔒 S1 · SECURITY      Data is born encrypted. Access is born restricted. Trust is born zero.
⚡ S2 · SPEED         Rust-native. Sub-millisecond local. Fast at every scale.
🏛️ S3 · SOVEREIGNTY   Apache 2.0 forever. Zero telemetry. Offline-first. No vendor. No lock-in.
🌿 S4 · SIMPLICITY    Zero-config start. SQL-first. 15-minute promise. Errors that teach.
🧠 +i · INTELLIGENCE  AI woven into the engine. Local inference only. Self-optimizing.
```

**Why S4+i and not S5?**

Intelligence is not a fifth pillar standing beside the others. It is the **multiplier** that runs through all four:

```
Security  × Intelligence  =  threats detected before they materialize
Speed     × Intelligence  =  queries that get faster the longer you use them
Sovereignty × Intelligence =  AI that knows everything about you and tells no one
Simplicity  × Intelligence =  a database that teaches you how to use it, as you use it
```

*S4 is the foundation. +i is what makes it breathe.*

---

## Quick Start

> ⚠️ EdisonDB is in active early development. Installation packages will be available with the 0.1 Alpha release. **Star this repo to be notified when it ships.**

Once released, getting started will look like this:

```bash
# Install (single binary, no dependencies)
edctl init myapp
edctl shell myapp
```

```sql
-- Standard SQL works on day one
CREATE TABLE users (id i64 PRIMARY KEY, name text, age i32);
INSERT INTO users VALUES (1, 'Edison', 32);
SELECT * FROM users WHERE age > 25;

-- Vector semantic search — native, no extension needed
SELECT * FROM memories
  VECTOR SEARCH 'recent conversations about security'
  LIMIT 10 SIMILARITY > 0.85;

-- Hybrid: relational + vector in a single query
SELECT u.name, m.content
  FROM users u JOIN memories m ON u.id = m.user_id
  VECTOR SEARCH m.embedding ~ 'data breach attempt'
  WHERE u.trust_score > 0.8
  LIMIT 5;

-- Auto-embed on insert — local AI, no external API
INSERT INTO documents (title, content)
  VALUES ('Sovereignty Report', 'Full text here...')
  AUTO EMBED content INTO embedding;
```

> **The 15-Minute Promise:** Any developer from any background will be productive in EdisonDB within 15 minutes. If that is not true, the documentation failed — not you.

---

## Architecture

EdisonDB is structured as a clean layered architecture. Each layer has a single, well-defined responsibility.

```
┌─────────────────────────────────────────────────────────────┐
│                     APPLICATION LAYER                       │
│     EQL Shell  │  REST API  │  gRPC API  │  SDK (Rust/Py/TS)│
├─────────────────────────────────────────────────────────────┤
│                     QUERY ENGINE LAYER                      │
│       EQL Parser → Planner → Optimizer → Executor           │
├──────────────┬──────────────┬────────────┬──────────────────┤
│  RELATIONAL  │   DOCUMENT   │   VECTOR   │     GRAPH        │
│  SQL Tables  │  JSON/BSON   │   HNSW     │   Trust Web      │
├──────────────┴──────────────┴────────────┴──────────────────┤
│               INTELLIGENCE LAYER (Mnemos)                   │
│   Auto-Embed  │  Query Learn  │  Anomaly Detect  │  AI RAG  │
├─────────────────────────────────────────────────────────────┤
│                   TRANSACTION LAYER                         │
│         ACID Transactions  │  MVCC  │  WAL (Encrypted)      │
├─────────────────────────────────────────────────────────────┤
│                   SECURITY LAYER (Aegis)                    │
│    AES-256-GCM  │  Row ACL  │  Audit Trail  │  Key Vault    │
├─────────────────────────────────────────────────────────────┤
│                ADAPTIVE STORAGE ENGINE                      │
│    LSM-Tree  │  Columnar  │  In-Memory  │  B-Tree Hybrid    │
├──────────────┬──────────────┬────────────────────────────── ┤
│   EMBEDDED   │    SERVER    │      DISTRIBUTED CLUSTER      │
│  (OS/Mobile) │  (Web/API)   │      (Raft Consensus)         │
└──────────────┴──────────────┴───────────────────────────────┘
```

### Deployment Modes

A **single EdisonDB binary** detects its environment at runtime and activates the appropriate mode. No configuration required.

| Mode | Target | Memory | Key Characteristics |
|---|---|---|---|
| **Embedded** | IoT, Mobile, Raspberry Pi, AIEONYX OS | < 8 MB | No server. Single file. Zero network surface. |
| **Standard** | Desktop, Developer machines | < 64 MB | Full feature set. Local server. Studio UI. |
| **Server** | Web backends, APIs, Containers | < 256 MB | Multi-user. REST + gRPC. TLS required. |
| **Cluster** | Production distributed systems | < 1 GB/node | Raft consensus. Geo-partitioning. HA. |

---

## EQL — EdisonDB Query Language

EQL is a **strict superset of SQL**. All valid SQL is valid EQL. EQL extends SQL with native vector search, graph traversal, and AI operations.

```sql
-- Graph traversal
SELECT * FROM trust_graph
  TRAVERSE FROM node('user_001') DEPTH 3
  WHERE relationship = 'trusted' AND weight > 0.7;

-- Time-series windowing
SELECT WINDOW(timestamp, '1h') as hour, AVG(cpu_usage)
  FROM system_metrics
  WHERE timestamp > NOW() - INTERVAL '24h'
  GROUP BY hour;

-- AI-native: generate and store embeddings locally
INSERT INTO docs (title, content)
  VALUES ('Report', 'Full text...')
  AUTO EMBED content INTO embedding;
```

---

## Performance Targets

| Operation | Embedded | Server | Cluster |
|---|---|---|---|
| Single row read (PK lookup) | < 0.1 ms | < 0.5 ms | < 2 ms |
| Vector ANN search (1M vectors) | < 5 ms | < 10 ms | < 25 ms |
| Hybrid relational + vector query | < 15 ms | < 30 ms | < 75 ms |
| Full-text search (1M documents) | < 10 ms | < 20 ms | < 50 ms |
| Auto-embedding (1K char document) | < 100 ms | < 100 ms | < 150 ms |
| Concurrent reads | — | 10,000+ | 100,000+ |

Single binary size: **< 15 MB**. No runtime dependencies. No installers.

---

## Security — Always On

EdisonDB treats security not as a feature you configure, but as **a property of existence**.

- **AES-256-GCM encryption at rest** — always on, cannot be disabled, zero opt-out
- **TLS 1.3** — mandatory for all network communication in Server and Cluster modes
- **Cryptographic Write-Ahead Log** — every write is signed and tamper-evident
- **Zero-trust access model** — every query authenticated, every result authorized
- **Row-level and column-level access control** — fine-grained permissions
- **Air-gap compatible** — operates with zero network connectivity, including all security functions

> *"A database that asks you to configure security has already failed you. EdisonDB is secure before you write your first line of code."*

---

## AI-Native Features

EdisonDB treats intelligence as a **native property of data**, not an external tool.

| Feature | Phase | Description |
|---|---|---|
| Native vector columns | 1 | `vector(n)` as a first-class column type |
| HNSW vector indexing | 1 | Sub-millisecond approximate nearest-neighbor search |
| Auto-embedding pipeline | 1 | INSERT triggers local AI — no external API |
| Semantic search (EQL) | 1 | `VECTOR SEARCH` syntax natively in EQL |
| Hybrid relational + vector | 1 | Combine SQL filters with semantic search in one query |
| RAG storage model | 1 | Unified doc + embedding + metadata for LLM pipelines |
| Intelligent query planner | 2 | Learns usage patterns, auto-optimizes over time |
| Anomaly detection | 2 | ML-based detection of unusual access patterns |

All AI inference uses **local models only** (Phi-3 Mini, Llama 3.2). No query, document, or embedding ever leaves your machine for AI processing.

---

## Community Promise

The following seven promises are made to every person who uses, builds on, or contributes to EdisonDB. **These promises are irrevocable.**

| # | Promise |
|---|---|
| **I** | EdisonDB Core will always be free |
| **II** | The Apache 2.0 license will never be downgraded |
| **III** | No features will move from free to paid |
| **IV** | Zero telemetry. Zero exceptions. |
| **V** | Every release will be fully reproducible from public source |
| **VI** | Governance will always be conducted in public |
| **VII** | Forking is always welcome — legally and morally |

→ Read the full [Community Promise & Open Source Charter](./COMMUNITY_PROMISE.md)

---

## Roadmap

| Phase | Timeline | Milestone | Key Deliverables |
|---|---|---|---|
| **Phase 1 — Alpha** | Month 1–9 | EdisonDB 0.1 Alpha | EQL parser, routing layer, AES-256 encryption, edctl CLI, Rust SDK, Apache 2.0 release |
| **Phase 2 — Beta** | Month 10–18 | EdisonDB 0.5 Beta | Native LSM-tree engine, HNSW index, auto-embedding, REST + gRPC, Python + TS SDKs, EdisonDB Studio |
| **Phase 3 — Stable** | Month 19–30 | EdisonDB 1.0 | Full production engine, all SDKs, distributed cluster (Raft), migration toolkit, compliance tooling |
| **Phase 4 — Scale** | Year 3+ | EdisonDB 2.0 | Horizontal auto-scaling, geo-partitioning, AI query planner v2, enterprise security suite |

---

## Platform Support

| Platform | Architecture | Mode | Status |
|---|---|---|---|
| Linux (Ubuntu 20.04+, Debian, Fedora, Arch) | x86_64, ARM64 | All modes | ✅ Tier 1 |
| macOS (12+) | x86_64, Apple Silicon | All modes | ✅ Tier 1 |
| Raspberry Pi (3, 4, 5) | ARM64, ARMv7 | Embedded + Standard | ✅ Tier 1 |
| AIEONYX OS | x86_64, ARM64, RISC-V | All modes | ✅ Primary target |
| Windows 10/11 | x86_64 | Standard + Server | 🟡 Tier 2 |
| Android (10+) | ARM64 | Embedded | 🟡 Tier 2 |
| iOS (15+) | ARM64 | Embedded | 🟡 Tier 2 |
| WebAssembly | wasm32 | Embedded | 🟡 Tier 2 |

---

## Technology Stack

| Component | Technology | Rationale |
|---|---|---|
| Primary language | Rust (2021 edition) | Memory safety, zero-cost abstractions, WASM target |
| Async runtime | Tokio | Industry-standard async Rust |
| Encryption | `ring` crate (BoringSSL-based) | FIPS-validated. Rust-native. Audited. |
| TLS | `rustls` | Pure Rust TLS. No OpenSSL. No C in the crypto path. |
| AI inference | `candle` (HuggingFace Rust) | Run Phi-3 and Llama locally in pure Rust. No Python runtime. |
| gRPC | `tonic` | Pure Rust gRPC. Tokio-native async. |
| Query parsing | `pest` | Fast, safe grammar definitions for EQL. |
| Testing | `cargo test` + `proptest` + `criterion` | Unit, property-based, and benchmark testing. |

---

## AIEONYX Platform Integration

EdisonDB is the universal data engine of the **AIEONYX Platform** — a security-first, AI-native operating system built on seL4 and Rust, currently in early development. Within AIEONYX, EdisonDB powers every core module:

| AIEONYX Module | EdisonDB Role |
|---|---|
| **Aegis** (Security) | Cryptographic primitives, key management, audit trail |
| **Mnemos** (AI Memory) | Vector + relational hybrid memory store |
| **Focus** (Productivity) | Tasks, schedules, user context |
| **TrustGraph** | Native graph tables for trust path resolution |
| **Commons** (Community repo) | Package metadata, ratings, download stats |
| **apm** (Package Manager) | Package registry and dependency graph |

EdisonDB also operates as a **fully independent project** — usable on any OS, by any developer, for any application.

→ Follow AIEONYX development: [github.com/aieonyx](https://github.com/aieonyx)

---

## Contributing

EdisonDB is built in the open. All contributions — code, documentation, bug reports, translations, and ideas — are welcome.

```bash
# Clone the repo (source code arriving with Phase 1)
git clone https://github.com/aieonyx/edisondb
cd edisondb
cargo build
cargo test --all-features
```

**Quality gates for all contributions:**
- All tests pass (`cargo test --all-features`)
- No unsafe code without documented justification
- No performance regressions > 5% in benchmark suite
- Clippy passes with zero warnings (pedantic mode)
- All public API documented

→ Join the discussion: [github.com/aieonyx/edisondb/discussions](https://github.com/aieonyx/edisondb/discussions)

---

## Documentation

> Documentation is being built alongside the engine. All docs will be published with EdisonDB 0.1 Alpha.

- [Technical Design Specification](./docs/TECHNICAL_SPEC.md)
- [Community Promise & Charter](./COMMUNITY_PROMISE.md)
- EQL Language Reference *(coming Phase 1)*
- API Reference *(coming Phase 1)*
- Getting Started Guide *(coming Phase 1)*

---

## License

EdisonDB Core is licensed under the **Apache License 2.0**.

This license will **never** be changed to a more restrictive license. This is Promise II of the Community Promise, and it is irrevocable.

See [LICENSE](./LICENSE) for the full license text.

---

<div align="center">

**Built by Edison. For the world. Powered by AIEONYX.**

*Apache License 2.0 · © 2026 AIEONYX / EdisonDB*

[github.com/aieonyx](https://github.com/aieonyx)

<br/>

*"Light for your data."*

</div>
