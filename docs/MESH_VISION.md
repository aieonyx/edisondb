# AIEONYX Mesh Network — Vision Document
**Document Type:** Long-Horizon Vision (Phase 3–4)  
**Status:** Published Vision — Not Yet In Active Development  
**Author:** Edison / AIEONYX  
**Date:** April 2026  
**Purpose:** Establish public record of design intent, attract long-horizon contributors, support future grant applications

---

## A Note On This Document

This is a vision document, not a technical specification. It describes where EdisonDB and AIEONYX BASTION are going — the problem being solved, the design philosophy, and the architecture being aimed at. It is published now to establish a dated public record of these ideas and to invite contributors who think in long horizons.

The features described here are **Phase 3–4**. Phase 1 (core EdisonDB engine) must ship first. What is written here will not be built tomorrow. It will be built on the foundation that is being laid today.

---

## The Problem This Solves

EdisonDB gives users sovereignty over their data. AIEONYX BASTION gives users a sovereign home node. Together they solve the data ownership problem. But one scenario remains unresolved — and it is the most human one.

**The flash flood scenario.**

Your house is flooded. You escape with nothing. Your phone is gone. Your laptop is gone. Your BASTION node is underwater. Every copy of your data — your medical records, your contacts, your documents, your credentials — is physically destroyed.

Under the current internet model, the answer is: your data is on our servers, here is your password reset link. The price of recovery is surrendering sovereignty.

Under EdisonDB's sovereign model, without the Mesh Network, the answer is: your data is gone.

That is not good enough. Sovereignty cannot mean fragility. The AIEONYX Mesh Network is the answer — a resilience layer that survives any single-point physical catastrophe without ever surrendering data to a server you do not control.

---

## The Design Philosophy — S4+i Throughout

Every decision in the Mesh Network is evaluated against the S4+i framework:

```
Security:     Your data fragments are mathematically useless
              to anyone who holds them — ghost data.
              No central attack surface. No server to breach.

Speed:        Local mesh first. Nearest node serves fastest.
              AI optimizes node selection dynamically.

Sovereignty:  Pure peer-to-peer. No AIEONYX server.
              No central directory. No authority controls
              your recovery. Your biometric is your key.

Simplicity:   You give storage, you earn storage.
              You press your thumb, your data comes back.
              No seed phrases. No passwords. No helpdesk.

Intelligence: AI balances fragment distribution, monitors
              node health, optimizes geo-redundancy,
              and manages recovery orchestration
              automatically.
```

---

## Part I — Ghost Fragment Hosting

### The Core Concept

Your data is never stored whole on any node you do not own. It is split into encrypted fragments — mathematically incomplete pieces that are individually and collectively useless without the reconstruction key that only you hold.

Nodes that host your fragments are **ghost hosts** — they hold something that, from their perspective, does not exist. They cannot read it. They cannot identify whose it is. They cannot combine it with other fragments they hold. The data is a ghost passing through their storage.

### How Fragmentation Works

**Step 1 — Encryption**
Before fragmentation, the data is encrypted with the user's master key. What gets fragmented is already ciphertext — random bytes to anyone without the key.

**Step 2 — Erasure Coding**
The encrypted data is split into fragments using Reed-Solomon erasure coding. The fragment count scales with file size to distribute compute and bandwidth fairly:

```
File size        Fragments    Needed to reconstruct
─────────────────────────────────────────────────
< 1 MB           3 fragments  2-of-3
1 MB – 10 MB     6 fragments  4-of-6
10 MB – 100 MB   12 fragments 8-of-12
> 100 MB         24 fragments 16-of-24
```

The ratio is consistent — roughly two-thirds of fragments needed to reconstruct, one-third redundant. Larger data produces more fragments, distributing the load across more nodes and more bandwidth providers.

**Step 3 — Triple Geo-Redundant Scatter**
Each fragment is stored three times — on three different nodes, in three different geographic regions. Fragment 4 of 12 exists on a node in the Philippines, a node in Germany, and a node in Brazil. Losing any one of those nodes does not lose the fragment.

```
Fragment 4 of 12:
  Copy A → Node: Philippines region
  Copy B → Node: Germany region
  Copy C → Node: Brazil region

For Fragment 4 to be permanently lost:
  All three nodes must fail simultaneously.
  Probability: astronomically low.

For the file to be unrecoverable:
  Enough fragments must be lost that
  fewer than 8-of-12 remain available.
  This requires a genuinely catastrophic
  multi-continent simultaneous failure.
```

**Step 4 — Fragment Encryption**
Each fragment is additionally encrypted with a per-fragment key before being sent to the host node. The host receives doubly-encrypted partial data. Even if a node operator is technically sophisticated and motivated, they hold encrypted fragments of encrypted data — two cryptographic barriers between them and any meaningful content.

**Step 5 — Auto-Purge**
Fragments have a defined lifetime. When the lifetime expires, the hosting node's Connector purges the fragment automatically — the same auto-purge mechanism used for Noise tier data. Even if a host node saves a copy externally, the fragment becomes part of an expired reconstruction window. The other fragments rotate on their own schedules. The reconstruction window closes permanently.

### What A Ghost Host Knows

A node hosting your fragment knows:

```
✓ The fragment's size (a number)
✓ That it is hosting a fragment for someone (no identity)
✗ Whose data it is
✗ What the data contains
✗ What any other fragment contains
✗ How many total fragments exist
✗ Where the other copies of this fragment are
✗ Anything that is useful without your master key
```

Ghost hosting is genuinely zero-knowledge. The host is a storage medium, not a data processor. They hold something that does not exist from their perspective.

### Mesh Nodes Also Protect Each Other

The same mesh infrastructure that distributes ghost fragments for resilience also carries **Aegis Collective** threat signatures — anonymized attack patterns shared instantly across all BASTION nodes worldwide. When any BASTION detects a novel attack, the behavioral signature propagates across the mesh within seconds, updating every BASTION's defensive posture automatically. The mesh is not only a resilience network. It is a living, self-updating immune system. See `docs/BASTION_SECURITY_VISION.md` for the full security architecture.

---

### The Model

The Mesh Network runs on a simple, honest economy: **you give storage to others, you earn storage credits for yourself.**

No tokens. No speculation. No mining. No AIEONYX-controlled currency. Pure reciprocal exchange — the same principle as a community grain store, a neighborhood tool library, or a mutual aid network.

```
You configure your BASTION:
  "I am willing to share 50GB for the mesh."

Your BASTION hosts ghost fragments for others.

You earn 50GB of mesh storage credit.

When you need mesh storage for your own backups,
your credits cover it.

When you contribute more, you earn more.
When you need less, your excess credits
benefit others at no cost to you.
```

### Credit Records

Credits are tracked locally — no central ledger, no blockchain, no AIEONYX server. Each BASTION node maintains a signed local credit record:

```
Credit record structure:
  Transaction:     [timestamp]
  Node A gave:     [X GB] to Node B
  Node B gave:     [X GB] to Node A
  Signed by:       Node A key + Node B key
  Verified by:     Both nodes locally

No third party verifies this.
No central authority tracks balances.
Disputes resolve by comparing signed records
between the two parties directly.
```

This is the simplest possible economy that works — the same double-entry bookkeeping that has run human commerce for 700 years, applied to storage exchange between sovereign nodes.

### Storage Budget Control

Every BASTION user controls their participation fully:

```
Storage settings:
  Total storage I share:    [user sets — e.g. 50GB]
  Maximum per single user:  [user sets — e.g. 5GB]
  Accepted regions:         [user sets — any / specific]
  Fragment types accepted:  [Critical only / Critical + Personal]
  Auto-purge after:         [user sets — e.g. 90 days]
```

Participation is always optional. Reducing shared storage reduces earned credits. A user who shares nothing earns nothing — but their own local 3-device resilience (phone, laptop, BASTION) still works without any mesh participation.

---

## Part III — Node Discovery

### No Central Directory

The Mesh Network has no central AIEONYX-controlled node registry. There is no server that knows where all the BASTION nodes are. Discovering nodes is peer-to-peer — the same way BitTorrent finds peers without a central server.

### Discovery Priority

The AI layer selects nodes in this order, optimizing for latency, reliability, and geographic diversity:

```
Priority 1 — Local network (LAN)
  Nodes on the same home or office network.
  Fastest. Most private. Zero internet exposure.
  Used for sync between personal devices.

Priority 2 — Nearby geographic nodes
  Nodes in the same city or region.
  Low latency. High reliability.
  Preferred for fragment hosting.

Priority 3 — User-preferred region
  User has selected specific countries or regions.
  Example: "store my fragments in EU only"
  for GDPR-conscious users.

Priority 4 — Any available global node
  Fallback when preferred nodes are unavailable.
  Used for maximum redundancy.

Never — A central AIEONYX directory
  AIEONYX does not operate infrastructure
  that knows where your data lives.
```

### Node Health Monitoring

The AI layer continuously monitors the health and reliability of nodes hosting your fragments:

```
Monitoring:
  → Fragment retrieval success rate
  → Node uptime history
  → Response latency
  → Geographic stability

Automatic response:
  → If a node's reliability drops below threshold,
    AI automatically migrates your fragment
    to a healthier node before the original fails.
  → If a region shows instability (natural disaster,
    political disruption), AI redistributes
    affected fragments proactively.
  → User is notified of significant redistributions.
```

---

## Part IV — Biometric Sovereign Recovery

### The Hardest UX Problem In Sovereign Data

Every sovereign data system eventually faces the same question: what happens when the user loses everything, including their password?

Traditional answers:
- Seed phrase (24 words to memorize — users lose them)
- Recovery email (controlled by a third party)
- Security questions (guessable, forgotten)
- Helpdesk (requires identity verification by a corporation)

Every traditional answer either fails the user in a crisis or surrenders sovereignty to a third party.

AIEONYX BASTION's answer: **your body is your key.**

### How Biometric Recovery Works

The AIEONYX BASTION OS requires certified hardware with a hardware security chip (TPM 2.0 minimum) and biometric sensors. The biometric is never stored as an image — it is processed into a cryptographic template that lives only inside the hardware security chip, never transmitted, never accessible to any software layer.

```
Biometric → Hardware security chip
          → Derives consistent cryptographic key
          → Key unlocks recovery credentials
          → Recovery credentials authenticate to mesh
          → Mesh returns fragments
          → Fragments reconstruct vault
          → Vault transfers to new device

The biometric image never exists outside the chip.
The cryptographic key never leaves the chip.
The recovery credentials are the only thing
that crosses the network — encrypted.
```

### The Flash Flood Scenario — Full Recovery Flow

```
Day 0:    Flash flood. All devices destroyed.
          User survives.

Day 1:    User reaches a shelter.
          Finds a new AIEONYX BASTION certified device.

          User presses: "Recover Account"
          User places thumb on fingerprint scanner.

          Hardware chip derives recovery key.
          Recovery key authenticates to mesh network.
          Mesh locates fragments across geo-redundant nodes.
          Fragments retrieved and reconstructed.
          Vault transferred to new device.

          User has their data back.
          No password. No helpdesk. No corporation involved.
```

### Backup Biometrics

Users register multiple biometrics at setup to protect against injury, disability, or amputation:

```
Primary:    Right thumb fingerprint
Backup 1:   Left thumb fingerprint
Backup 2:   Any finger fingerprint
Backup 3:   Face scan
Backup 4:   Iris scan
Emergency:  Legacy Holder key (from Digital Legacy spec)
```

Any registered biometric unlocks recovery. The user cannot be locked out of their own data by a physical accident as long as any registered biometric remains viable.

### Emergency Profile Recovery — Borrowed Device Mode

In the immediate hours after a crisis, a user may not have access to a BASTION-certified device. They may only have a borrowed phone.

**Emergency Profile Recovery** provides degraded but meaningful access:

```
EMERGENCY PROFILE RECOVERY

Access method:    Face scan on any camera-equipped device
                  (front camera sufficient)

Data accessible:  Emergency tier only — pre-defined at setup:
                  → Emergency contacts
                  → Email recovery credentials (masked)
                  → Basic identity documents
                  → Medical emergency information
                  → Financial emergency access (last 4 digits)

Time limit:       User-defined at setup (default: 24 hours)
                  Session closes automatically at expiry.

Device retention: Zero. Borrowed device retains nothing
                  after session closes.

Audit:            Full log — device fingerprint, location,
                  timestamp, what was accessed.

Critical tier:    NOT accessible in emergency mode.
                  Requires BASTION-certified hardware.

Full recovery:    Completes only on certified BASTION hardware.
                  Emergency mode is a bridge, not a destination.
```

Emergency Profile Recovery gives a crisis survivor what they need immediately — contacts, basic documents, email access — without exposing Critical tier data on a device they do not control.

---

## Part V — The Sovereign 3-2-1-Mesh Model

The complete resilience architecture combines personal device redundancy with the mesh network:

```
SOVEREIGN 3-2-1-MESH RESILIENCE MODEL

Personal layer (always):
  Copy 1 → Phone        (on your person)
  Copy 2 → Laptop       (at home or work)
  Copy 3 → BASTION      (home node, 24/7)

Mesh layer (optional, for Critical/Personal):
  Copy 4+ → Ghost fragments across geo-redundant
             nodes worldwide (if opted in)

Failure scenarios:

  Phone lost           → BASTION + Laptop
  Laptop broken        → BASTION + Phone
  BASTION fails        → Phone + Laptop
  House fire           → Phone was with you
                         Mesh fragments survive
  Flash flood          → Mesh fragments survive
                         Biometric recovery on new device
  All devices lost,
  no mesh opted in     → Emergency Legacy Holder recovery
  All devices lost,
  mesh opted in        → Full biometric mesh recovery
  Truly everything     → Impossible if geo-redundant
                         fragments exist across 3+ regions
```

The last line is the mission statement of the Mesh Network. With geo-redundant fragments across multiple continents and biometric recovery on any certified hardware, losing your data permanently becomes a scenario that requires simultaneous multi-continental catastrophic failure. This is not theoretical robustness — it is designed, measurable, cryptographic robustness.

---

## Part VI — What This Means For AIEONYX BASTION OS

The Mesh Network transforms AIEONYX BASTION OS from a personal home server into a **platform**.

Hardware manufacturers who choose to install AIEONYX BASTION OS on their devices are not just shipping a privacy-respecting home server. They are shipping a node in a global sovereign resilience network. Their hardware becomes part of an infrastructure that protects people's most important data through any physical catastrophe.

**AIEONYX BASTION OS Certified Hardware Requirements:**

```
Security:
  → TPM 2.0 chip (hardware key storage)
  → Fingerprint sensor (500 DPI minimum)
  → Optional: Face recognition camera
  → Optional: Iris scanner
  → Secure enclave for biometric template storage
    (biometric derivative never leaves the chip)

Compute:
  → ARM64 or x86_64 processor
  → Minimum 4GB RAM (8GB recommended for local LLM)
  → Minimum 64GB storage (user-expandable)

Network:
  → Ethernet port
  → WiFi capability
  → Stable uptime capability (designed for 24/7 operation)

Certification:
  → Hardware vendor applies for AIEONYX BASTION certification
  → Security audit of biometric handling and TPM implementation
  → Confirmation of secure enclave architecture
  → Listed in AIEONYX public certified hardware registry
```

Any hardware manufacturer meeting these requirements can ship AIEONYX BASTION OS. The certification process is open and public. AIEONYX does not charge for certification — alignment with the standard is the only requirement.

---

## Relationship To The Broader AIEONYX Ecosystem

```
EdisonDB Core         → The data engine and tier enforcement
EdisonDB Connector    → The platform sovereignty gateway
AIEONYX BASTION OS    → The sovereign home node operating system
AIEONYX Mesh Network  → The resilience layer above all three

Each layer is independent.
Each layer is stronger with the others.
Together they form a complete sovereign
personal data infrastructure — from the
storage engine to the global resilience network.
```

---

## Phases and Timing

```
Phase 1 (Now):
  EdisonDB core engine, tier enforcement,
  Connector, basic multi-device sync.
  The 3-device personal resilience model works.

Phase 2:
  Digital Legacy system, Regional Compliance Profiles,
  Emergency Profile Recovery groundwork,
  BASTION hardware certification program begins.

Phase 3:
  AIEONYX Mesh Network — local mesh first.
  Ghost Fragment Hosting between personal devices.
  Reciprocal storage economy between trusted nodes.
  Biometric recovery on certified hardware.

Phase 4:
  Full global mesh with DHT node discovery.
  Geo-redundant triple scatter across regions.
  Emergency Profile Recovery on borrowed devices.
  AI-driven node health monitoring and
  automatic fragment migration.
  Open hardware certification registry.
```

---

## A Final Word

The internet was built without thinking about what happens when people lose everything. Cloud services solved the resilience problem by owning the data. That trade — resilience for sovereignty — was accepted because no alternative existed.

EdisonDB and the AIEONYX Mesh Network are the alternative. Resilience without surrender. Recovery without a corporation in the middle. Your thumb on a scanner is enough to bring your life back after a flood.

That is what this is being built for.

---

*This is a vision document. Implementation begins when Phase 1 is complete.*  
*Community input welcome at github.com/aieonyx/edisondb/discussions*  
*Published April 2026 — establishing public record of design intent.*
