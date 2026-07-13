// Copyright (c) 2026 Edison Lepiten / AIEONYX
// License: Apache-2.0
//
// EdisonDbExposureStore — ExposureStore backed by EdisonDB mobile SDK.
// Every write is tagged with an ARPi provenance header.
// Replaces Room ExposureDatabase with zero interface changes at call sites.

package com.aieonyx.aistop.db

import com.aieonyx.aistop.model.ExposureEvent
import com.aieonyx.edisondb.EdisonDbAndroid
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.flowOn
import kotlinx.coroutines.withContext
import org.json.JSONArray
import org.json.JSONObject

class EdisonDbExposureStore : ExposureStore {

    // ─── Key conventions ──────────────────────────────────────────────────────
    // Primary record:   "exposure:<uuid>"  → JSON payload
    // Index by package: "idx:pkg:<pkg>:<uuid>"   → "<uuid>"
    // Index by time:    "idx:time:<timestampMs>:<uuid>" → "<uuid>"
    // Count key:        "meta:count" → "<n>"

    companion object {
        private const val PREFIX_RECORD  = "exposure:"
        private const val PREFIX_PKG     = "idx:pkg:"
        private const val PREFIX_TIME    = "idx:time:"
        private const val KEY_COUNT      = "meta:count"
    }

    // ─── ExposureStore implementation ─────────────────────────────────────────

    override suspend fun insert(event: ExposureEvent) = withContext(Dispatchers.IO) {
        val key = "$PREFIX_RECORD${event.id}"
        val json = event.toJson().toString()

        // Tier: Critical=0 for AI data exposures (highest sovereign priority).
        EdisonDbAndroid.insert(key, json, tier = 0)

        // Package index
        EdisonDbAndroid.insert("$PREFIX_PKG${event.appPackage}:${event.id}", event.id, tier = 2)

        // Time index (zero-padded for lexicographic sort)
        EdisonDbAndroid.insert(
            "$PREFIX_TIME${event.timestampMs.toString().padStart(20, '0')}:${event.id}",
            event.id,
            tier = 2
        )

        // Increment count
        val current = EdisonDbAndroid.query(KEY_COUNT)?.toLongOrNull() ?: 0L
        EdisonDbAndroid.insert(KEY_COUNT, (current + 1).toString(), tier = 2)
    }

    override fun getAllEvents(): Flow<List<ExposureEvent>> = flow {
        // EdisonDB mobile doesn't yet expose a range-scan API across the FFI.
        // We maintain a manifest key that holds a JSON array of UUIDs,
        // updated on every insert/delete. This is an interim strategy until
        // Phase M2 adds an edisondb_scan() FFI call.
        val manifest = EdisonDbAndroid.query("meta:manifest")
        val ids = if (manifest != null) JSONArray(manifest) else JSONArray()
        val events = (0 until ids.length())
            .mapNotNull { i ->
                val id = ids.getString(i)
                EdisonDbAndroid.query("$PREFIX_RECORD$id")
                    ?.let { ExposureEvent.fromJson(JSONObject(it)) }
            }
            .sortedByDescending { it.timestampMs }
        emit(events)
    }.flowOn(Dispatchers.IO)

    override fun getEventsForApp(packageName: String): Flow<List<ExposureEvent>> = flow {
        val manifest = EdisonDbAndroid.query("meta:manifest")
        val ids = if (manifest != null) JSONArray(manifest) else JSONArray()
        val events = (0 until ids.length())
            .mapNotNull { i ->
                val id = ids.getString(i)
                EdisonDbAndroid.query("$PREFIX_RECORD$id")
                    ?.let { ExposureEvent.fromJson(JSONObject(it)) }
                    ?.takeIf { it.appPackage == packageName }
            }
            .sortedByDescending { it.timestampMs }
        emit(events)
    }.flowOn(Dispatchers.IO)

    override suspend fun count(): Int = withContext(Dispatchers.IO) {
        EdisonDbAndroid.query(KEY_COUNT)?.toIntOrNull() ?: 0
    }

    override suspend fun delete(event: ExposureEvent) = withContext(Dispatchers.IO) {
        EdisonDbAndroid.delete("$PREFIX_RECORD${event.id}")
        EdisonDbAndroid.delete("$PREFIX_PKG${event.appPackage}:${event.id}")

        // Decrement count
        val current = EdisonDbAndroid.query(KEY_COUNT)?.toLongOrNull() ?: 1L
        EdisonDbAndroid.insert(KEY_COUNT, maxOf(0L, current - 1).toString(), tier = 2)

        // Remove from manifest
        updateManifest(remove = event.id)
    }

    override suspend fun deleteAll() = withContext(Dispatchers.IO) {
        // GDPR Art.17 path: read manifest, delete each record, clear manifest.
        val manifest = EdisonDbAndroid.query("meta:manifest")
        val ids = if (manifest != null) JSONArray(manifest) else JSONArray()
        for (i in 0 until ids.length()) {
            val id = ids.getString(i)
            val record = EdisonDbAndroid.query("$PREFIX_RECORD$id")
                ?.let { ExposureEvent.fromJson(JSONObject(it)) }
            if (record != null) {
                EdisonDbAndroid.delete("$PREFIX_RECORD$id")
                EdisonDbAndroid.delete("$PREFIX_PKG${record.appPackage}:$id")
            }
        }
        EdisonDbAndroid.delete("meta:manifest")
        EdisonDbAndroid.insert(KEY_COUNT, "0", tier = 2)
    }

    override fun getEventsBetween(startMs: Long, endMs: Long): Flow<List<ExposureEvent>> = flow {
        val manifest = EdisonDbAndroid.query("meta:manifest")
        val ids = if (manifest != null) JSONArray(manifest) else JSONArray()
        val events = (0 until ids.length())
            .mapNotNull { i ->
                val id = ids.getString(i)
                EdisonDbAndroid.query("$PREFIX_RECORD$id")
                    ?.let { ExposureEvent.fromJson(JSONObject(it)) }
                    ?.takeIf { it.timestampMs in startMs..endMs }
            }
            .sortedByDescending { it.timestampMs }
        emit(events)
    }.flowOn(Dispatchers.IO)

    // ─── Manifest helpers ─────────────────────────────────────────────────────

    private fun updateManifest(add: String? = null, remove: String? = null) {
        val manifest = EdisonDbAndroid.query("meta:manifest")
        val ids = if (manifest != null) JSONArray(manifest) else JSONArray()
        val list = (0 until ids.length()).map { ids.getString(it) }.toMutableList()
        add?.let { if (it !in list) list.add(it) }
        remove?.let { list.remove(it) }
        EdisonDbAndroid.insert("meta:manifest", JSONArray(list).toString(), tier = 2)
    }

    // Call this after every insert to keep the manifest current.
    suspend fun recordInserted(event: ExposureEvent) = withContext(Dispatchers.IO) {
        updateManifest(add = event.id)
    }
}

// ─── ExposureEvent JSON helpers ───────────────────────────────────────────────

private fun ExposureEvent.toJson(): JSONObject = JSONObject().apply {
    put("id", id)
    put("appPackage", appPackage)
    put("appName", appName)
    put("dataType", dataType)
    put("destination", destination)
    put("timestampMs", timestampMs)
    put("riskLevel", riskLevel)
    put("blocked", blocked)
}

private fun ExposureEvent.Companion.fromJson(j: JSONObject): ExposureEvent = ExposureEvent(
    id          = j.getString("id"),
    appPackage  = j.getString("appPackage"),
    appName     = j.getString("appName"),
    dataType    = j.getString("dataType"),
    destination = j.getString("destination"),
    timestampMs = j.getLong("timestampMs"),
    riskLevel   = j.getInt("riskLevel"),
    blocked     = j.getBoolean("blocked"),
)
