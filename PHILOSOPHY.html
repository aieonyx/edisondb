<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>EdisonDB Manifesto — Sovereign Data Infrastructure</title>
<link href="https://fonts.googleapis.com/css2?family=Cormorant+Garamond:ital,wght@0,300;0,400;0,600;1,300&family=JetBrains+Mono:wght@300;400;500&family=Bebas+Neue&display=swap" rel="stylesheet">
<style>
  :root {
    --gold: #C9A84C;
    --gold-light: #E8C96A;
    --cyan: #00D4FF;
    --cyan-dim: #0099BB;
    --dark: #080A0E;
    --dark-2: #0D1117;
    --dark-3: #141920;
    --dark-4: #1C2333;
    --white: #F0EDE8;
    --white-dim: #9A9590;
    --red-alert: #FF4444;
  }

  * { margin: 0; padding: 0; box-sizing: border-box; }

  html { scroll-behavior: smooth; }

  body {
    background: var(--dark);
    color: var(--white);
    font-family: 'Cormorant Garamond', serif;
    font-size: 19px;
    line-height: 1.8;
    overflow-x: hidden;
  }

  /* NOISE TEXTURE OVERLAY */
  body::before {
    content: '';
    position: fixed;
    inset: 0;
    background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 256 256' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noise'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noise)' opacity='0.04'/%3E%3C/svg%3E");
    pointer-events: none;
    z-index: 0;
    opacity: 0.4;
  }

  /* HERO */
  .hero {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    padding: 4rem 2rem;
    position: relative;
    background: radial-gradient(ellipse 80% 60% at 50% 40%, #0D1A2E 0%, var(--dark) 70%);
  }

  .hero::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 1px;
    background: linear-gradient(90deg, transparent, var(--gold), transparent);
  }

  .shield-container {
    margin-bottom: 3rem;
    animation: float 6s ease-in-out infinite;
  }

  @keyframes float {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-12px); }
  }

  .shield-svg {
    width: 100px;
    height: 120px;
    filter: drop-shadow(0 0 30px rgba(201, 168, 76, 0.4)) drop-shadow(0 0 60px rgba(0, 212, 255, 0.15));
  }

  .eyebrow {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.7rem;
    letter-spacing: 0.4em;
    color: var(--cyan);
    text-transform: uppercase;
    margin-bottom: 1.5rem;
    opacity: 0.8;
  }

  h1 {
    font-family: 'Bebas Neue', sans-serif;
    font-size: clamp(4rem, 12vw, 9rem);
    letter-spacing: 0.05em;
    line-height: 0.9;
    background: linear-gradient(135deg, var(--gold-light) 0%, var(--gold) 50%, #8B6914 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin-bottom: 1rem;
  }

  .subtitle {
    font-family: 'Cormorant Garamond', serif;
    font-size: clamp(1rem, 2.5vw, 1.4rem);
    font-style: italic;
    font-weight: 300;
    color: var(--white-dim);
    max-width: 600px;
    margin-bottom: 3rem;
  }

  .tagline {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.75rem;
    letter-spacing: 0.2em;
    color: var(--gold);
    border: 1px solid rgba(201, 168, 76, 0.3);
    padding: 0.6rem 1.5rem;
    text-transform: uppercase;
  }

  /* ECOSYSTEM BAR */
  .ecosystem {
    background: var(--dark-2);
    border-top: 1px solid rgba(201, 168, 76, 0.15);
    border-bottom: 1px solid rgba(201, 168, 76, 0.15);
    padding: 1.5rem 2rem;
    display: flex;
    justify-content: center;
    gap: 3rem;
    flex-wrap: wrap;
    position: relative;
    z-index: 1;
  }

  .eco-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.3rem;
  }

  .eco-label {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.6rem;
    letter-spacing: 0.3em;
    color: var(--cyan);
    text-transform: uppercase;
  }

  .eco-name {
    font-family: 'Bebas Neue', sans-serif;
    font-size: 1.3rem;
    letter-spacing: 0.1em;
    color: var(--gold);
  }

  /* MAIN CONTENT */
  .content {
    max-width: 820px;
    margin: 0 auto;
    padding: 6rem 2rem;
    position: relative;
    z-index: 1;
  }

  /* SECTIONS */
  .section {
    margin-bottom: 6rem;
    opacity: 0;
    transform: translateY(30px);
    animation: fadeUp 0.8s forwards;
  }

  .section:nth-child(1) { animation-delay: 0.1s; }
  .section:nth-child(2) { animation-delay: 0.2s; }
  .section:nth-child(3) { animation-delay: 0.3s; }
  .section:nth-child(4) { animation-delay: 0.4s; }
  .section:nth-child(5) { animation-delay: 0.5s; }
  .section:nth-child(6) { animation-delay: 0.6s; }
  .section:nth-child(7) { animation-delay: 0.7s; }

  @keyframes fadeUp {
    to { opacity: 1; transform: translateY(0); }
  }

  .section-label {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.65rem;
    letter-spacing: 0.4em;
    color: var(--cyan);
    text-transform: uppercase;
    margin-bottom: 1.5rem;
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .section-label::after {
    content: '';
    flex: 1;
    height: 1px;
    background: linear-gradient(90deg, var(--cyan-dim), transparent);
    opacity: 0.4;
  }

  h2 {
    font-family: 'Bebas Neue', sans-serif;
    font-size: clamp(2.2rem, 5vw, 3.5rem);
    letter-spacing: 0.05em;
    color: var(--gold);
    line-height: 1;
    margin-bottom: 2rem;
  }

  p {
    color: var(--white);
    font-weight: 300;
    margin-bottom: 1.5rem;
    opacity: 0.9;
  }

  /* PULL QUOTE */
  .pull-quote {
    border-left: 3px solid var(--gold);
    padding: 1.5rem 2rem;
    margin: 3rem 0;
    background: linear-gradient(90deg, rgba(201,168,76,0.05), transparent);
    font-size: 1.3rem;
    font-style: italic;
    font-weight: 300;
    color: var(--gold-light);
    line-height: 1.6;
  }

  /* CODE BLOCK */
  .code-block {
    background: var(--dark-3);
    border: 1px solid rgba(0, 212, 255, 0.15);
    border-left: 3px solid var(--cyan);
    padding: 1.5rem 2rem;
    margin: 2rem 0;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.8rem;
    line-height: 2;
    color: var(--cyan);
    overflow-x: auto;
  }

  .code-block .comment { color: var(--white-dim); }
  .code-block .highlight { color: var(--gold); }

  /* TIER CARDS */
  .tier-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 1px;
    margin: 2.5rem 0;
    background: rgba(201, 168, 76, 0.1);
    border: 1px solid rgba(201, 168, 76, 0.15);
  }

  .tier-card {
    padding: 2rem;
    background: var(--dark-2);
    position: relative;
    overflow: hidden;
  }

  .tier-card::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    width: 4px;
  }

  .tier-critical::before { background: var(--red-alert); }
  .tier-personal::before { background: var(--gold); }
  .tier-noise::before { background: var(--cyan); }

  .tier-name {
    font-family: 'Bebas Neue', sans-serif;
    font-size: 1.4rem;
    letter-spacing: 0.1em;
    margin-bottom: 0.5rem;
  }

  .tier-critical .tier-name { color: var(--red-alert); }
  .tier-personal .tier-name { color: var(--gold); }
  .tier-noise .tier-name { color: var(--cyan); }

  .tier-desc {
    font-size: 0.9rem;
    color: var(--white-dim);
    margin: 0;
    font-family: 'JetBrains Mono', monospace;
  }

  .tier-rule {
    margin-top: 1rem;
    margin-bottom: 0;
    font-size: 0.95rem;
    opacity: 0.85;
  }

  /* FEATURE GRID */
  .feature-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 1px;
    margin: 2.5rem 0;
    background: rgba(0, 212, 255, 0.08);
    border: 1px solid rgba(0, 212, 255, 0.12);
  }

  .feature-card {
    background: var(--dark-2);
    padding: 1.5rem;
    transition: background 0.2s;
  }

  .feature-card:hover {
    background: var(--dark-3);
  }

  .feature-icon {
    font-size: 1.4rem;
    margin-bottom: 0.5rem;
  }

  .feature-title {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.75rem;
    letter-spacing: 0.15em;
    color: var(--cyan);
    text-transform: uppercase;
    margin-bottom: 0.5rem;
  }

  .feature-desc {
    font-size: 0.9rem;
    color: var(--white-dim);
    margin: 0;
    line-height: 1.5;
  }

  /* ECOSYSTEM DIAGRAM */
  .diagram {
    background: var(--dark-3);
    border: 1px solid rgba(201, 168, 76, 0.15);
    padding: 2.5rem;
    margin: 2.5rem 0;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.78rem;
    line-height: 2.2;
    color: var(--white-dim);
    overflow-x: auto;
  }

  .diagram .box { color: var(--gold); }
  .diagram .arrow { color: var(--cyan); opacity: 0.6; }
  .diagram .label { color: var(--white); }

  /* REPLACE TABLE */
  .replace-table {
    width: 100%;
    border-collapse: collapse;
    margin: 2rem 0;
    font-size: 0.95rem;
  }

  .replace-table th {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.65rem;
    letter-spacing: 0.3em;
    text-transform: uppercase;
    color: var(--cyan);
    padding: 0.8rem 1rem;
    text-align: left;
    border-bottom: 1px solid rgba(0, 212, 255, 0.2);
  }

  .replace-table td {
    padding: 0.8rem 1rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    color: var(--white-dim);
  }

  .replace-table td:first-child {
    color: var(--white);
    opacity: 0.7;
    font-style: italic;
  }

  .replace-table td:last-child {
    color: var(--gold);
    font-weight: 400;
  }

  .replace-table tr:hover td {
    background: rgba(201, 168, 76, 0.04);
  }

  /* LICENSE */
  .license-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1px;
    margin: 2rem 0;
    background: rgba(201, 168, 76, 0.1);
    border: 1px solid rgba(201, 168, 76, 0.15);
  }

  .license-card {
    background: var(--dark-2);
    padding: 2rem;
  }

  .license-name {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.7rem;
    letter-spacing: 0.3em;
    color: var(--cyan);
    text-transform: uppercase;
    margin-bottom: 0.5rem;
  }

  .license-title {
    font-family: 'Bebas Neue', sans-serif;
    font-size: 1.6rem;
    color: var(--gold);
    letter-spacing: 0.05em;
    margin-bottom: 0.8rem;
  }

  .license-desc {
    font-size: 0.9rem;
    color: var(--white-dim);
    margin: 0;
    line-height: 1.6;
  }

  /* CTA */
  .cta {
    text-align: center;
    padding: 6rem 2rem;
    background: radial-gradient(ellipse 60% 50% at 50% 50%, #0D1A2E, var(--dark));
    border-top: 1px solid rgba(201, 168, 76, 0.15);
    position: relative;
  }

  .cta-title {
    font-family: 'Bebas Neue', sans-serif;
    font-size: clamp(2.5rem, 7vw, 5rem);
    color: var(--gold);
    letter-spacing: 0.05em;
    margin-bottom: 1rem;
  }

  .cta-sub {
    font-style: italic;
    font-weight: 300;
    color: var(--white-dim);
    max-width: 500px;
    margin: 0 auto 3rem;
    font-size: 1.1rem;
  }

  .cta-links {
    display: flex;
    gap: 1rem;
    justify-content: center;
    flex-wrap: wrap;
  }

  .btn-primary {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.7rem;
    letter-spacing: 0.3em;
    text-transform: uppercase;
    color: var(--dark);
    background: var(--gold);
    padding: 1rem 2.5rem;
    text-decoration: none;
    display: inline-block;
    transition: all 0.2s;
  }

  .btn-primary:hover {
    background: var(--gold-light);
    transform: translateY(-2px);
  }

  .btn-secondary {
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.7rem;
    letter-spacing: 0.3em;
    text-transform: uppercase;
    color: var(--cyan);
    border: 1px solid rgba(0, 212, 255, 0.4);
    padding: 1rem 2.5rem;
    text-decoration: none;
    display: inline-block;
    transition: all 0.2s;
  }

  .btn-secondary:hover {
    border-color: var(--cyan);
    background: rgba(0, 212, 255, 0.05);
    transform: translateY(-2px);
  }

  /* FOOTER */
  footer {
    background: var(--dark-2);
    border-top: 1px solid rgba(201, 168, 76, 0.1);
    padding: 2rem;
    text-align: center;
    font-family: 'JetBrains Mono', monospace;
    font-size: 0.65rem;
    letter-spacing: 0.2em;
    color: var(--white-dim);
    text-transform: uppercase;
  }

  footer span { color: var(--gold); }

  /* DIVIDER */
  .divider {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin: 4rem 0;
    opacity: 0.3;
  }

  .divider::before, .divider::after {
    content: '';
    flex: 1;
    height: 1px;
    background: var(--gold);
  }

  .divider-symbol {
    color: var(--gold);
    font-size: 1.2rem;
  }

  @media (max-width: 600px) {
    .license-grid { grid-template-columns: 1fr; }
    .ecosystem { gap: 1.5rem; }
  }
</style>
</head>
<body>

<!-- HERO -->
<section class="hero">
  <div class="shield-container">
    <svg class="shield-svg" viewBox="0 0 100 120" xmlns="http://www.w3.org/2000/svg">
      <defs>
        <linearGradient id="shieldGold" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" style="stop-color:#E8C96A"/>
          <stop offset="100%" style="stop-color:#8B6914"/>
        </linearGradient>
        <linearGradient id="shieldInner" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" style="stop-color:#0D1A2E"/>
          <stop offset="100%" style="stop-color:#080A0E"/>
        </linearGradient>
      </defs>
      <path d="M50 5 L90 20 L90 55 Q90 90 50 115 Q10 90 10 55 L10 20 Z" fill="url(#shieldGold)"/>
      <path d="M50 12 L83 25 L83 55 Q83 84 50 107 Q17 84 17 55 L17 25 Z" fill="url(#shieldInner)"/>
      <path d="M50 35 L58 48 L72 50 L62 60 L64 74 L50 67 L36 74 L38 60 L28 50 L42 48 Z" fill="url(#shieldGold)" opacity="0.9"/>
      <path d="M35 38 Q50 28 65 38" stroke="#00D4FF" stroke-width="1.5" fill="none" opacity="0.6"/>
      <path d="M32 50 L68 50" stroke="#00D4FF" stroke-width="0.8" fill="none" opacity="0.3"/>
      <circle cx="50" cy="50" r="2" fill="#00D4FF" opacity="0.8"/>
    </svg>
  </div>
  <div class="eyebrow">AIEONYX Ecosystem — Sovereign Infrastructure Manifesto</div>
  <h1>EdisonDB</h1>
  <p class="subtitle">The database that treats your data the way the law always intended — as yours.</p>
  <div class="tagline">Your Data. Your Device. Your Law.</div>
</section>

<!-- ECOSYSTEM BAR -->
<div class="ecosystem">
  <div class="eco-item">
    <span class="eco-label">Operating System</span>
    <span class="eco-name">AIEONYX OS</span>
  </div>
  <div class="eco-item">
    <span class="eco-label">Sovereign Database</span>
    <span class="eco-name">EdisonDB</span>
  </div>
  <div class="eco-item">
    <span class="eco-label">Home Node Hardware</span>
    <span class="eco-name">AIEONYX BASTION</span>
  </div>
  <div class="eco-item">
    <span class="eco-label">Security</span>
    <span class="eco-name">Aegis</span>
  </div>
  <div class="eco-item">
    <span class="eco-label">AI Memory</span>
    <span class="eco-name">Mnemos</span>
  </div>
</div>

<!-- MAIN CONTENT -->
<main class="content">

  <!-- SECTION 1: THE PROBLEM -->
  <div class="section">
    <div class="section-label">I — The Structural Failure</div>
    <h2>The Internet Was Built Without An Owner</h2>
    <p>Every platform tells you their servers exist to serve you. They do not. Their servers exist to own you. Your thoughts, your movements, your relationships, your habits — extracted, stored, profiled, and monetized without your meaningful consent.</p>
    <p>The EU GDPR tried to fix this at the legal layer. The EU AI Act is trying again. But laws applied to a structurally broken architecture produce only compliance theater. Companies hire lawyers. They anonymize data that can be re-identified. They present consent forms nobody reads. They pay fines as a cost of doing business.</p>
    <div class="pull-quote">The law cannot fix a database problem. Only a database can fix a database problem.</div>
  </div>

  <!-- SECTION 2: THE LAW -->
  <div class="section">
    <div class="section-label">II — The Forcing Function</div>
    <h2>The Law Is Now Making Compliance Impossible</h2>
    <p>The EU AI Act and evolving global privacy legislation have created a structural crisis for every platform that holds user data. The right to erasure, the right to portability, the prohibition of unlawful AI inference from personal data — these are not requests. They are legal obligations with existential financial consequences.</p>
    <p>Platforms are discovering that compliance is architecturally impossible when your entire infrastructure assumes ownership of data that was never legally yours.</p>
    <p>EdisonDB does not help platforms comply with the law. <strong style="color: var(--gold)">EdisonDB makes non-compliance architecturally impossible.</strong></p>
  </div>

  <!-- SECTION 3: THE PRINCIPLE -->
  <div class="section">
    <div class="section-label">III — The Core Principle</div>
    <h2>The Store Analogy</h2>
    <div class="pull-quote">Data belongs to the person it describes. Always. Without exception. Not as a policy. Not as a setting. As a database primitive.</div>
    <p>When you walk into a store, the salesperson may remember your visit, the time, and what you purchased. They cannot keep your photograph. They cannot follow you home. They cannot sell a record of your visit to a third party profiling you across every store you have ever entered.</p>
    <p>EdisonDB brings this common sense into the digital world. You are the store. The platform is the salesperson. The transaction ends when you leave. What remains is only what the law permits — a timestamped receipt, nothing more.</p>
    <p>When you are online, you are present. When you go offline, your presence ends. There is no complication. Your data rights go offline when the processing ends — exactly as you would expect from a real-world transaction.</p>
  </div>

  <!-- SECTION 4: TIERED SOVEREIGNTY -->
  <div class="section">
    <div class="section-label">IV — The Architecture</div>
    <h2>Tiered Data Sovereignty</h2>
    <p>Not all data is created equal. EdisonDB recognizes this as a first-class database principle — enforced structurally, not by application policy or platform promise.</p>

    <div class="tier-grid">
      <div class="tier-card tier-critical">
        <div class="tier-name">⬤ Critical Tier</div>
        <p class="tier-desc">Medical records · Biometrics · Private messages · Financial data · Precise location history</p>
        <p class="tier-rule">Never leaves your device. Ever. No platform, no government, no third party accesses Critical data without explicit, revocable, audited consent. Encrypted at rest. Structurally isolated.</p>
      </div>
      <div class="tier-card tier-personal">
        <div class="tier-name">⬤ Personal Tier</div>
        <p class="tier-desc">Name · Preferences · Purchase history · General behavior · Account identity</p>
        <p class="tier-rule">Accessible only to applications you explicitly trust, under consent you control and can withdraw at any moment. Treated like a paying transaction — when the session ends, access ends.</p>
      </div>
      <div class="tier-card tier-noise">
        <div class="tier-name">⬤ Noise Tier</div>
        <p class="tier-desc">Public posts · Reactions · Comments you chose to publish · Public activity</p>
        <p class="tier-rule">Data you consciously depersonalized by publishing. Syncs freely. Hosted on platform servers for a maximum of 30 days per EU CCTV data retention precedent — then auto-purged. Your act of publishing was the consent.</p>
      </div>
    </div>

    <p>Classification is enforced by schema, role, and AI layer simultaneously. No manual tagging. The database is intelligent enough to know what it holds.</p>

    <!-- DIGITAL LEGACY -->
    <div style="border-left:3px solid var(--gold);padding:1.5rem 2rem;margin:3rem 0;background:linear-gradient(90deg,rgba(201,168,76,0.05),transparent);font-size:1.1rem;font-style:italic;font-weight:300;color:var(--gold-light);line-height:1.6;">
      "When a person dies, their data should not outlive their family's consent. Under EdisonDB's model, public posts expire automatically. Private data goes dark when the device does. A family should never have to beg a corporation to let their loved one rest."
    </div>

    <p>Under the current internet architecture, a deceased person's profile remains permanently active — served by algorithms, interacted with publicly, controlled entirely by a platform that has no obligation to honor a grieving family's request. EdisonDB addresses this not through policy, but through the mechanics of how data is classified and expires. Noise tier posts auto-purge from platform servers after 30 days. Critical and Personal data goes dark when the device does. A Phase 2 <strong style="color:var(--gold)">Digital Legacy</strong> system allows users to pre-designate a trusted person who can trigger immediate data purge, vault transfer, or full deletion — on the family's terms, not the platform's.</p>
  </div>

  <!-- SECTION 5: CONNECTOR -->
  <div class="section">
    <div class="section-label">V — Platform Integration</div>
    <h2>The EdisonDB Connector</h2>
    <p>Platforms do not rebuild their infrastructure to integrate EdisonDB. They install the <strong style="color:var(--gold)">EdisonDB Connector</strong> — a lightweight SDK on their backend server — the same way they install a payment gateway.</p>

    <div class="code-block">
      <span class="comment"># Platform integration — as simple as a payment SDK</span><br>
      <span class="highlight">EdisonDB</span>::Connector::install(<br>
      &nbsp;&nbsp;role: <span class="highlight">ProcessorOnly</span>,<br>
      &nbsp;&nbsp;tier_access: [<span class="highlight">Noise</span>],&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="comment">// Critical and Personal: unreachable</span><br>
      &nbsp;&nbsp;retention: <span class="highlight">30.days</span>,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="comment">// Auto-purge enforced</span><br>
      &nbsp;&nbsp;audit: <span class="highlight">GDPR::AutoLog</span>,&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="comment">// Regulator-ready trail</span><br>
      &nbsp;&nbsp;identity: <span class="highlight">RotatingToken</span>,&nbsp;&nbsp;&nbsp;&nbsp;<span class="comment">// No persistent fingerprint</span><br>
      )
    </div>

    <p>The Connector is architecturally incapable of passing Critical or Personal data to the platform — even if the platform requests it. Platforms become true <strong style="color:var(--gold)">Scale-to-Zero processors.</strong> They shed data storage infrastructure, legal liability, breach exposure, and compliance cost simultaneously.</p>
    <p>EdisonDB is not a burden to platforms. It is a relief.</p>
  </div>

  <div class="divider"><span class="divider-symbol">◆</span></div>

  <!-- SECTION 6: BASTION -->
  <div class="section">
    <div class="section-label">VI — The Sovereign Node</div>
    <h2>AIEONYX BASTION</h2>
    <p>A personal sovereign infrastructure device. Your data brain. Your security shield. Your private AI. Your home cloud. One box you own completely — running AIEONYX OS on a seL4 formally verified microkernel, built in Rust.</p>
    <p>It replaces your ISP router. It becomes the sync anchor for all your devices. When your phone saves data and goes offline, BASTION holds it. When your laptop connects, it syncs from BASTION. No platform ever touches this exchange.</p>

    <div class="diagram">
      <span class="box">┌─────────────────────────────────────┐</span><br>
      <span class="box">│       AIEONYX BASTION               │</span>&nbsp;&nbsp;<span class="comment">← Runs 24/7 in your home</span><br>
      <span class="box">│                                     │</span><br>
      <span class="box">│  ┌──────────────┐ ┌──────────────┐  │</span><br>
      <span class="box">│  │  AIEONYX OS  │ │   EdisonDB   │  │</span><br>
      <span class="box">│  │  seL4 + Rust │ │ Sovereign DB │  │</span><br>
      <span class="box">│  └──────────────┘ └──────────────┘  │</span><br>
      <span class="box">│  ┌──────────────┐ ┌──────────────┐  │</span><br>
      <span class="box">│  │ Network Core │ │  Local LLM   │  │</span><br>
      <span class="box">│  │ Router+Aegis │ │  Inference   │  │</span><br>
      <span class="box">│  └──────────────┘ └──────────────┘  │</span><br>
      <span class="box">└─────────────────────────────────────┘</span><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="arrow">↕ encrypted mesh</span><br>
      <span class="box">┌──────────┐</span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="box">┌──────────┐</span><br>
      <span class="box">│  Phone   │</span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="box">│  Laptop  │</span><br>
      <span class="box">│EdisonDB  │</span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="box">│EdisonDB  │</span><br>
      <span class="box">└──────────┘</span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="box">└──────────┘</span>
    </div>

    <div class="feature-grid">
      <div class="feature-card">
        <div class="feature-icon">🛡️</div>
        <div class="feature-title">Network Router + Firewall</div>
        <p class="feature-desc">Replaces your ISP router entirely. DNS-level ad and tracker blocking. Anti-malware traffic inspection. Intrusion detection.</p>
      </div>
      <div class="feature-card">
        <div class="feature-icon">🔐</div>
        <div class="feature-title">Password Vault + 2FA</div>
        <p class="feature-desc">Fully local password manager. TOTP authenticator. Hardware security key support. Biometric unlock — data never leaves BASTION.</p>
      </div>
      <div class="feature-card">
        <div class="feature-icon">🧠</div>
        <div class="feature-title">Local LLM Intelligence</div>
        <p class="feature-desc">Private AI inference on your own hardware. Answers questions from your data only. Zero telemetry. Works fully offline.</p>
      </div>
      <div class="feature-card">
        <div class="feature-icon">☁️</div>
        <div class="feature-title">Sovereign File Storage</div>
        <p class="feature-desc">Documents, photos, videos stored locally. Version history. Encrypted sync to your devices. Time-limited encrypted sharing links.</p>
      </div>
      <div class="feature-card">
        <div class="feature-icon">🌐</div>
        <div class="feature-title">Personal VPN Server</div>
        <p class="feature-desc">WireGuard-based. Connect to your BASTION from anywhere. Browse through your own network. No third-party VPN needed.</p>
      </div>
      <div class="feature-card">
        <div class="feature-icon">🔑</div>
        <div class="feature-title">Sovereign Identity (DID)</div>
        <p class="feature-desc">Cryptographic self-sovereign identity. Rotating session tokens — no persistent fingerprint. Device trust registry.</p>
      </div>
    </div>

    <h3 style="font-family: 'Bebas Neue'; color: var(--gold); font-size: 1.8rem; letter-spacing: 0.05em; margin: 2.5rem 0 1.5rem;">What BASTION Replaces</h3>
    <table class="replace-table">
      <tr><th>Replaced Product</th><th>Replaced By</th></tr>
      <tr><td>ISP Router</td><td>BASTION Network Core</td></tr>
      <tr><td>Google Drive / iCloud</td><td>Sovereign File Storage</td></tr>
      <tr><td>1Password / Bitwarden</td><td>Local Password Vault</td></tr>
      <tr><td>Google Authenticator</td><td>Local 2FA Engine</td></tr>
      <tr><td>Pi-hole</td><td>Built-in DNS Blocker</td></tr>
      <tr><td>NordVPN / ExpressVPN</td><td>Personal VPN Server</td></tr>
      <tr><td>ChatGPT / Copilot</td><td>Local LLM on EdisonDB</td></tr>
      <tr><td>GDPR Compliance Lawyer</td><td>EdisonDB Connector Audit Log</td></tr>
    </table>

    <div class="pull-quote">One box. One purchase. No subscriptions. No cloud dependency. No data leaving your home.</div>
  </div>

  <!-- SECTION 7: LICENSE -->
  <div class="section">
    <div class="section-label">VII — License and Philosophy</div>
    <h2>Open By Principle</h2>
    <div class="license-grid">
      <div class="license-card">
        <div class="license-name">EdisonDB Core</div>
        <div class="license-title">AGPLv3</div>
        <p class="license-desc">Free forever. Open forever. Modifications must remain open. Corporations cannot take this work, close it, and compete against the community that built it.</p>
      </div>
      <div class="license-card">
        <div class="license-name">EdisonDB Connector</div>
        <div class="license-title">BUSL</div>
        <p class="license-desc">Free for personal and open source use. Commercially licensed for platforms generating revenue from EdisonDB-connected users. The only revenue mechanism. Minimal by design.</p>
      </div>
    </div>
  </div>

  <!-- SECTION 8: WHY NOW -->
  <div class="section">
    <div class="section-label">VIII — The Window</div>
    <h2>Why This Moment</h2>
    <p>The EU AI Act enforcement is accelerating. Public trust in platforms is at a historic low. The WordPress ecosystem — powering 43% of the internet — is fracturing under governance failures and security vulnerabilities. Developers are actively searching for a new foundation.</p>
    <p>EdisonDB is built in Rust for memory safety and performance. Designed for AI-native classification at the database layer. Aligned with EU, GDPR, and emerging global sovereignty legislation from the first line of code.</p>
    <p>EdisonDB does not ask you to leave your platforms. <strong style="color:var(--gold)">It legally transforms your relationship with them.</strong></p>
    <p>The law is ready. The market is ready. The technology is ready.</p>
  </div>

</main>

  <!-- REGIONAL & JURISDICTIONAL -->
  <div class="section">
    <div class="section-label">VIII — Sovereignty Without Borders</div>
    <h2>Privacy Means Different Things. Sovereignty Does Not.</h2>
    <p>EdisonDB does not assume that privacy means the same thing everywhere. In some cultures, data sovereignty means aggressive protection and silence. In others, it means the freedom to share widely and the right to be remembered. A Filipino user who posts family albums publicly and genuinely enjoys community visibility should not be forced into defaults they never asked for. A German user who wants every trace deleted after 30 days deserves exactly that. EdisonDB respects both — because sovereignty means the user decides, not the platform and not the database.</p>

    <div class="pull-quote">Regional Compliance Profiles adapt EdisonDB's behavior to local law and culture — while Critical tier protection, the Inverted Administration Model, and the audit log remain absolute and universal.</div>

    <p>Some jurisdictions, however, are architecturally incompatible with EdisonDB's commercial platform model. China's National Intelligence Law and Russia's SORM laws require technology providers to give government access to user data on demand. EdisonDB's Critical tier has no backdoor, no government override, and no administrative access path. These are cryptographic commitments — not policy promises. They cannot be honored without destroying what the project is.</p>

    <p>For <strong style="color:var(--gold)">personal and embedded use</strong> — a developer anywhere in the world using EdisonDB locally with no Connector — there is no regulatory surface and no conflict. EdisonDB welcomes every developer from every country for personal use. The incompatibility is specific to commercial sovereign gateway deployment, and it exists because the architecture will not compromise on what it was built to protect.</p>

    <div class="pull-quote">A database with a government backdoor is not a sovereign database. It is a surveillance tool with a privacy label. EdisonDB will not build one.</div>
  </div>

  <!-- SMART DIGITAL SOVEREIGN COMMUNITY -->
  <div class="section">
    <div class="section-label">IX — The Community</div>
    <h2>Smart Digital Sovereign Community</h2>
    <p>AIEONYX is not a product. It is a community — a Smart Digital Sovereign Community of devices and people who protect each other, remember each other's data through any catastrophe, and refuse to surrender their digital lives to corporations or governments.</p>

    <div class="pull-quote">Every BASTION that connects to the mesh makes every other BASTION stronger. Every threat detected anywhere is defeated everywhere within seconds. Every person who joins brings their sovereignty — and adds to everyone else's. This is what the internet was supposed to be.</div>

    <p>When a USB drive infected with malware is plugged into a device in Manila, BASTION intercepts the threat before it reaches the internet. The anonymous signature propagates across the Aegis Collective to every BASTION worldwide — Germany, Brazil, Singapore, everywhere — within seconds. That person just made every member of the community safer. Without knowing it. Without doing anything. Simply by being part of it.</p>

    <p>This is what sovereignty means at scale. Not one person protecting their own data. Every person protecting each other's — automatically, decentrally, and freely. Anchored in S4+i. Built in Rust. Proven on seL4. Open forever.</p>

    <div class="code-block">
      <span class="highlight">EdisonDB</span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="comment">→ Sovereign data intelligence</span><br>
      <span class="highlight">AIEONYX BASTION</span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="comment">→ Sovereign home fortress</span><br>
      <span class="highlight">AIEONYX Mesh Network</span>&nbsp;&nbsp;<span class="comment">→ Sovereign resilience layer</span><br>
      <span class="highlight">Aegis Collective</span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<span class="comment">→ Sovereign community defense</span><br>
      <br>
      <span class="comment">One philosophy:  S4+i</span><br>
      <span class="comment">One mission:     Digital sovereignty as a right, not a privilege</span><br>
      <span class="comment">One community:   Every BASTION owner, everywhere, protecting each other</span>
    </div>
  </div>

</main>

<section class="cta">
  <div class="cta-title">A Call To Build</div>
  <p class="cta-sub">EdisonDB is one person's vision seeking the hands of those who believe infrastructure should serve people, not corporations.</p>
  <div class="cta-links">
    <a href="https://github.com/your-handle/edisondb" class="btn-primary">View on GitHub</a>
    <a href="mailto:your@contact.com" class="btn-secondary">Get In Touch</a>
  </div>
</section>

<footer>
  <p>
    <span>EdisonDB</span> — A founding project of the <span>AIEONYX</span> Ecosystem &nbsp;·&nbsp;
    AGPLv3 Core / BUSL Connector &nbsp;·&nbsp;
    Built in <span>Rust</span> &nbsp;·&nbsp;
    Aligned with <span>EU AI Act</span> + GDPR
  </p>
</footer>

</body>
</html>
