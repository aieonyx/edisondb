// Copyright (c) 2026 Edison Lepiten / AIEONYX
// License: Apache-2.0
//
// ArpiHeader — 78-byte AXON Receptor Protocol Interface provenance header.
// Generated client-side on Android for every EdisonDB write.
// The Rust FFI layer validates magic + BLAKE3 hash before committing.
//
// Layout (matches Rust ArpiHeader in mobile/mod.rs):
//   Offset  Size  Field
//    0       4    magic: "ARPi"
//    4       8    write_counter (u64 LE) — set to 0 here; Rust assigns final
//   12       8    timestamp_us (u64 LE, Unix epoch microseconds)
//   20       1    tier (0=Critical, 1=Personal, 2=Noise)
//   21       3    reserved (zero)
//   24      32    BLAKE3 content hash of value bytes
//   56      22    node_id (UTF-8, zero-padded)

package com.aieonyx.edisondb

import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.security.MessageDigest

object ArpiHeader {

    const val SIZE = 78
    private val MAGIC = byteArrayOf(0x41, 0x52, 0x50, 0x69)  // "ARPi"

    /**
     * Build a 78-byte provenance header for [value] at the given [tier].
     * The BLAKE3 hash slot is filled with a SHA-256 approximation because
     * BLAKE3 isn't available in the Android standard library.
     * The Rust side verifies with real BLAKE3; this must match.
     *
     * Production note: add the `oolong` or `blake3-jvm` artifact to build.gradle
     * for a real BLAKE3 implementation, then replace [hashValue] below.
     */
    fun build(value: String, tier: Byte = 1, nodeId: String = ""): ByteArray {
        val buf = ByteBuffer.allocate(SIZE).order(ByteOrder.LITTLE_ENDIAN)

        // magic
        buf.put(MAGIC)

        // write_counter — Rust assigns the final monotonic counter on insert.
        buf.putLong(0L)

        // timestamp_us
        buf.putLong(System.currentTimeMillis() * 1_000L)

        // tier + reserved
        buf.put(tier)
        buf.put(ByteArray(3))

        // BLAKE3 content hash (32 bytes)
        buf.put(hashValue(value.toByteArray(Charsets.UTF_8)))

        // node_id (22 bytes, zero-padded)
        val nodeBytes = nodeId.toByteArray(Charsets.UTF_8).copyOf(22)
        buf.put(nodeBytes)

        return buf.array()
    }

    /**
     * SHA-256 stand-in for BLAKE3 until a JVM BLAKE3 artifact is wired.
     * Replace with `Blake3.hash(data)` once `com.ionspin.kotlin:blake3` is
     * added to app/build.gradle.
     *
     * The Rust FFI verifies BLAKE3; this will mismatch until replaced.
     * For Phase M1 development, set verification to lenient mode in
     * `MobileDb::insert` (`#[cfg(debug_assertions)]` guard).
     */
    private fun hashValue(data: ByteArray): ByteArray {
        return MessageDigest.getInstance("SHA-256").digest(data)
    }
}
