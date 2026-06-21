// Copyright (c) 2026 Edison Lepiten / AIEONYX
// EdisonDB P3-M4 — Sovereign embedding tests (20 tests)

use edisondb::sovereign_embed::{SovereignEmbedder, EmbeddingBackend, EMBED_DIM};

// ── T1: embed returns correct dimension ───────────────────────────────────────
#[test]
fn t1_embed_dim() {
    let e = SovereignEmbedder::new();
    let v = e.embed("sovereign data");
    assert_eq!(v.len(), EMBED_DIM);
}

// ── T2: embed is deterministic ────────────────────────────────────────────────
#[test]
fn t2_embed_deterministic() {
    let e = SovereignEmbedder::new();
    let v1 = e.embed("EdisonDB sovereign database");
    let v2 = e.embed("EdisonDB sovereign database");
    assert_eq!(v1, v2);
}

// ── T3: embed is L2-normalized ────────────────────────────────────────────────
#[test]
fn t3_embed_normalized() {
    let e = SovereignEmbedder::new();
    let v = e.embed("AXONYX language");
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    assert!((norm - 1.0).abs() < 1e-4, "norm={}", norm);
}

// ── T4: empty string returns zero vector ─────────────────────────────────────
#[test]
fn t4_embed_empty() {
    let e = SovereignEmbedder::new();
    let v = e.embed("");
    assert_eq!(v.len(), EMBED_DIM);
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    assert!(norm < 1e-6, "empty embed should be zero vector");
}

// ── T5: different texts produce different vectors ─────────────────────────────
#[test]
fn t5_embed_distinct() {
    let e = SovereignEmbedder::new();
    let v1 = e.embed("sovereign database");
    let v2 = e.embed("web protocol");
    let sim = SovereignEmbedder::similarity(&v1, &v2);
    // Different topics should not be near-identical
    assert!(sim < 0.99, "unrelated texts too similar: {}", sim);
}

// ── T6: similar texts have higher similarity than unrelated ──────────────────
#[test]
fn t6_embed_similarity_order() {
    let e = SovereignEmbedder::new();
    let base  = e.embed("sovereign database storage");
    let close = e.embed("sovereign storage database");
    let far   = e.embed("browser rendering engine");
    let sim_close = SovereignEmbedder::similarity(&base, &close);
    let sim_far   = SovereignEmbedder::similarity(&base, &far);
    assert!(sim_close > sim_far,
        "close={} far={} — similar text should score higher", sim_close, sim_far);
}

// ── T7: self-similarity is 1.0 ────────────────────────────────────────────────
#[test]
fn t7_self_similarity() {
    let e = SovereignEmbedder::new();
    let v = e.embed("AIEONYX sovereign stack");
    let sim = SovereignEmbedder::similarity(&v, &v);
    assert!((sim - 1.0).abs() < 1e-4, "self-similarity={}", sim);
}

// ── T8: similarity is symmetric ───────────────────────────────────────────────
#[test]
fn t8_similarity_symmetric() {
    let e = SovereignEmbedder::new();
    let a = e.embed("axon compiler");
    let b = e.embed("compiler axon");
    let ab = SovereignEmbedder::similarity(&a, &b);
    let ba = SovereignEmbedder::similarity(&b, &a);
    assert!((ab - ba).abs() < 1e-6, "ab={} ba={}", ab, ba);
}

// ── T9: similarity in [-1, 1] ─────────────────────────────────────────────────
#[test]
fn t9_similarity_bounds() {
    let e = SovereignEmbedder::new();
    let texts = ["hello", "world", "axon", "sovereign", "database", "browser"];
    for i in 0..texts.len() {
        for j in 0..texts.len() {
            let a = e.embed(texts[i]);
            let b = e.embed(texts[j]);
            let s = SovereignEmbedder::similarity(&a, &b);
            assert!(s >= -1.001 && s <= 1.001, "sim out of bounds: {} vs {}: {}", texts[i], texts[j], s);
        }
    }
}

// ── T10: custom dimension ─────────────────────────────────────────────────────
#[test]
fn t10_custom_dim() {
    let e = SovereignEmbedder::with_dim(64);
    let v = e.embed("test");
    assert_eq!(v.len(), 64);
}

// ── T11: different seeds produce different vectors ────────────────────────────
#[test]
fn t11_seed_independence() {
    let e1 = SovereignEmbedder::new();
    let mut e2 = SovereignEmbedder::new();
    e2.seed = 0xDEADBEEFCAFEBABE;
    let v1 = e1.embed("sovereign");
    let v2 = e2.embed("sovereign");
    assert_ne!(v1, v2, "different seeds must produce different vectors");
}

// ── T12: single character token ───────────────────────────────────────────────
#[test]
fn t12_single_char_filtered() {
    let e = SovereignEmbedder::new();
    // Single chars are filtered by tokenizer (len < 2)
    let v1 = e.embed("a b c");
    let v2 = e.embed("");
    // Both should have zero norm (all tokens filtered)
    let n1: f32 = v1.iter().map(|x| x*x).sum::<f32>().sqrt();
    let n2: f32 = v2.iter().map(|x| x*x).sum::<f32>().sqrt();
    assert!(n1 < 1e-6 && n2 < 1e-6);
}

// ── T13: EmbeddingBackend::sovereign() is always available ───────────────────
#[test]
fn t13_sovereign_backend_available() {
    let b = EmbeddingBackend::sovereign();
    assert!(b.is_available());
    assert_eq!(b.backend_name(), "sovereign");
}

// ── T14: EmbeddingBackend::sovereign() embed works ───────────────────────────
#[test]
fn t14_sovereign_backend_embed() {
    let b = EmbeddingBackend::sovereign();
    let v = b.embed("sovereign database").unwrap();
    assert_eq!(v.len(), EMBED_DIM);
}

// ── T15: EmbeddingBackend dim() ───────────────────────────────────────────────
#[test]
fn t15_backend_dim() {
    let b = EmbeddingBackend::sovereign();
    assert_eq!(b.dim(), EMBED_DIM);
}

// ── T16: EmbeddingBackend::auto() falls back to sovereign ────────────────────
#[test]
fn t16_auto_fallback() {
    // Ollama not available in CI — auto() should return sovereign
    let b = EmbeddingBackend::auto();
    // Regardless of which backend, embed must succeed
    let v = b.embed("auto fallback test").unwrap();
    assert!(!v.is_empty());
    assert!(b.is_available());
}

// ── T17: bigrams affect embedding ────────────────────────────────────────────
#[test]
fn t17_bigrams_affect_embedding() {
    let e = SovereignEmbedder::new();
    let v1 = e.embed("sovereign database");
    let v2 = e.embed("database sovereign");
    // Bigram order differs → vectors differ (not identical)
    assert_ne!(v1, v2, "word order should affect embedding via bigrams");
}

// ── T18: long text embeds correctly ──────────────────────────────────────────
#[test]
fn t18_long_text() {
    let e = SovereignEmbedder::new();
    let long = "EdisonDB is a sovereign AI-native multi-model database engine \
                built in Rust with WAL MVCC storage gRPC transport vector search \
                and offline sovereign embedding pipeline powered by AXONYX";
    let v = e.embed(long);
    assert_eq!(v.len(), EMBED_DIM);
    let norm: f32 = v.iter().map(|x| x*x).sum::<f32>().sqrt();
    assert!((norm - 1.0).abs() < 1e-4);
}

// ── T19: numeric text ─────────────────────────────────────────────────────────
#[test]
fn t19_numeric_text() {
    let e = SovereignEmbedder::new();
    let v = e.embed("version 0 65 0 release");
    assert_eq!(v.len(), EMBED_DIM);
    let norm: f32 = v.iter().map(|x| x*x).sum::<f32>().sqrt();
    assert!((norm - 1.0).abs() < 1e-4);
}

// ── T20: similarity of identical after round-trip ────────────────────────────
#[test]
fn t20_round_trip_identity() {
    let b = EmbeddingBackend::sovereign();
    let text = "AIEONYX sovereign digital civilization";
    let v1 = b.embed(text).unwrap();
    let v2 = b.embed(text).unwrap();
    let sim = SovereignEmbedder::similarity(&v1, &v2);
    assert!((sim - 1.0).abs() < 1e-4, "round-trip similarity={}", sim);
}
