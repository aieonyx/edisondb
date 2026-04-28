# EdisonDB Community Promise & Open Source Charter

*Effective: April 2026*  
*Author: Edison, Founder, AIEONYX*  
*Status: Irrevocable*

---

## Preamble

EdisonDB was built on a simple belief — that the people who use a database should be able to trust it completely. Not because of a terms of service they clicked through, but because the promises made here are structural, public, and cannot be taken back.

This document is not legal boilerplate. It is a personal commitment from the founder to every developer, contributor, and user who places their trust in EdisonDB. These promises were written before the first line of production code. They will outlast any business decision, any funding round, and any change in circumstance.

If EdisonDB ever violates these promises, the community has the full legal and moral right to fork the project and carry the mission forward without us. That right is not a loophole — it is the point.

---

## The Seven Promises

---

### Promise I — EdisonDB Core Will Always Be Free

EdisonDB Core — the database engine, the query language, the storage engine, the Tier Enforcement Layer, and all local features — will always be available at zero cost to any person, for any purpose.

Free means free. Not free with a usage cap. Not free for individuals but paid for teams. Not free until we decide otherwise.

This promise has no expiration date.

---

### Promise II — The License Will Never Be Downgraded

EdisonDB Core is licensed under the **Apache License 2.0**.

This license will never be changed to a more restrictive license. It will never become a Business Source License, a Commons Clause restriction, a Server Side Public License, or any other license that reduces the rights of users and contributors.

Apache 2.0 grants you the right to use, modify, distribute, and build commercial products with EdisonDB Core without restriction. That right is yours permanently.

> *This promise was made before the project launched. It applies to every version of EdisonDB Core, past, present, and future. No board decision, no investor pressure, and no change in ownership can override it.*

**Note on the EdisonDB Connector:** The Connector — the platform integration SDK — is a separate component with its own Business Source License. It did not exist when this promise was written and is not covered by Promise II. The Connector license converts to Apache 2.0 on **January 1, 2030**, at which point it also becomes fully free and open.

---

### Promise III — Features Will Never Move From Free To Paid

No feature that exists today in EdisonDB Core will ever move behind a paywall, a subscription, or a commercial license requirement.

Future commercial offerings, if any, will be built as new, separate components — never by removing or restricting what already exists in the open source core.

The history of open source is marked by projects that gave generously and then quietly took back. EdisonDB will not be one of them.

---

### Promise IV — Zero Telemetry, Zero Exceptions

EdisonDB will never collect usage data, crash statistics, query patterns, performance metrics, or any other information about how you use the software — without your explicit, informed, opt-in action.

The binary does not phone home. There is no background process. There is no scheduled reporter. There is no "anonymous statistics" toggle that defaults to on.

Zero telemetry means the software is blind to your usage by design, not by setting.

The only exception is the `edctl crash-report export` command — an opt-in tool that you invoke manually, read yourself before sending, and choose to share or discard. Even this tool never sends anything automatically.

---

### Promise V — Every Release Will Be Reproducible

Every release of EdisonDB Core will be fully reproducible from public source code.

This means:
- All source code is public at the time of release
- The build process is documented and deterministic
- Anyone can produce a byte-for-byte identical binary from the published source
- No proprietary build steps, no hidden dependencies, no closed toolchains

You will never have to trust a binary you cannot verify.

---

### Promise VI — Governance Will Always Be Conducted In Public

All significant decisions about EdisonDB's direction — roadmap changes, architectural decisions, license interpretations, community standards — will be discussed and decided in public.

This means:
- Major proposals are posted as GitHub Discussions before decisions are made
- The reasoning behind decisions is always documented and publicly accessible
- No significant change to the project will be made in private and announced after the fact
- The community will always have a meaningful opportunity to respond before decisions are finalized

EdisonDB is not a democracy where every voice carries equal weight in every decision. It is a project with a founder and maintainers who carry final responsibility. But it is a project that operates in the open, explains itself honestly, and treats the community as partners rather than users.

---

### Promise VII — Forking Is Always Welcome

EdisonDB is built on the belief that good software should be free to grow in directions its creators did not anticipate.

Forking this project is not a hostile act. It is the healthiest thing open source can produce. If the community disagrees with the direction EdisonDB is taking, forking is not just permitted — it is encouraged.

The Apache 2.0 license gives you the legal right to fork without asking for permission. This promise gives you the moral right as well. No contributor agreement, no community guideline, and no statement from the project will ever be used to discourage, shame, or obstruct a fork.

If a fork serves the mission of sovereign data better than the original, that is a success — not a failure.

---

## What Is Not Promised

Honesty requires stating what is not covered here.

**EdisonDB does not promise to be production-ready today.** The project is in active early development. Bugs exist. APIs will change. Stability will come with time and contributions.

**EdisonDB does not promise indefinite maintenance by the original team.** If the project is ever abandoned, the community has everything it needs — source code, documentation, and the full legal right — to continue without us.

**EdisonDB does not promise that the EdisonDB Connector will always be free.** The Connector is a separate component with a Business Source License until January 1, 2030. Commercial use of the Connector before that date requires a license.

**EdisonDB does not promise GDPR compliance as a complete solution.** The architecture enforces data minimization and access boundaries structurally — reducing compliance surface more than any previous database. But GDPR compliance also requires purpose limitation, lawful basis, and consent management that exist at the application and organizational level. EdisonDB is a powerful tool for compliance. It is not a substitute for legal counsel.

---

## Amendments

This document may be amended to add clarity, correct errors, or address situations not anticipated at the time of writing. Amendments will never reduce the rights granted here. They may only extend them.

All amendments will be made publicly, with a clear record of what changed and why, committed to the public repository with a full history.

---

## A Final Word From Edison

I wrote these promises before I wrote production code because I believe the philosophy of a project matters more than any feature it ships.

I have seen too many projects start with genuine good intentions and end with license changes, paywalls, and broken trust. I wanted EdisonDB to begin with its promises already in writing — so that no future version of me, no investor, and no circumstance can quietly undo what was committed to here.

These seven promises are the contract I make with everyone who gives EdisonDB their time, their trust, or their contributions. I intend to keep them.

*— Edison*  
*Founder, AIEONYX*  
*Czech Republic, April 2026*

---

<div align="center">

*EdisonDB Community Promise & Open Source Charter*  
*Apache License 2.0 Core · © 2026 AIEONYX / EdisonDB*  
*This document is itself released under CC0 — no rights reserved.*

[github.com/aieonyx/edisondb](https://github.com/aieonyx/edisondb)

</div>
