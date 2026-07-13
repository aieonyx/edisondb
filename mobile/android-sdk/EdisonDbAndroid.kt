// Copyright (c) 2026 Edison Lepiten / AIEONYX
// License: Apache-2.0
//
// EdisonDbAndroid — singleton JNI wrapper for EdisonDB embedded on Android.
// Manages the native DbHandle lifecycle; all calls are thread-safe via
// the underlying Rust Mutex<MobileDb>.

package com.aieonyx.edisondb

import android.content.Context
import android.util.Log
import java.io.File

object EdisonDbAndroid {

    private const val TAG = "EdisonDB"
    private const val LIB_NAME = "editsondb"

    @Volatile
    private var dbHandle: Long = 0L   // raw pointer from edisondb_open

    init {
        System.loadLibrary(LIB_NAME)
    }

    // ─── Lifecycle ────────────────────────────────────────────────────────────

    /**
     * Open (or create) the EdisonDB store under the app's files directory.
     * Call once from Application.onCreate() or before first use.
     */
    @Synchronized
    fun open(context: Context) {
        if (dbHandle != 0L) return
        val dbDir = File(context.filesDir, "edisondb").also { it.mkdirs() }
        val path = dbDir.absolutePath
        dbHandle = nativeOpen(path)
        if (dbHandle == 0L) {
            Log.e(TAG, "Failed to open EdisonDB at $path")
            throw IllegalStateException("EdisonDB open failed")
        }
        Log.i(TAG, "EdisonDB opened at $path")
    }

    /**
     * Close the database. Call from Application.onTerminate() or test teardown.
     */
    @Synchronized
    fun close() {
        if (dbHandle != 0L) {
            nativeClose(dbHandle)
            dbHandle = 0L
            Log.i(TAG, "EdisonDB closed")
        }
    }

    // ─── Public API ───────────────────────────────────────────────────────────

    /**
     * Insert [value] at [key] with an ARPi provenance header.
     * The header is generated automatically by [ArpiHeader].
     *
     * @param key   Record key (e.g. exposure event UUID)
     * @param value JSON-serialised record payload
     * @param tier  0=Critical, 1=Personal, 2=Noise
     * @return true on success
     */
    fun insert(key: String, value: String, tier: Byte = 1): Boolean {
        checkOpen()
        val header = ArpiHeader.build(value, tier)
        val rc = nativeInsert(dbHandle, key, value, header)
        if (rc != 0) Log.w(TAG, "insert($key) returned $rc")
        return rc == 0
    }

    /**
     * Query the value stored at [key], or null if absent.
     */
    fun query(key: String): String? {
        checkOpen()
        return nativeQuery(dbHandle, key)
    }

    /**
     * Delete the record at [key].
     * @return true if the key existed and was deleted.
     */
    fun delete(key: String): Boolean {
        checkOpen()
        return nativeDelete(dbHandle, key) == 0
    }

    // ─── Internal ─────────────────────────────────────────────────────────────

    private fun checkOpen() {
        check(dbHandle != 0L) { "EdisonDB is not open. Call EdisonDbAndroid.open(context) first." }
    }

    // ─── JNI declarations ─────────────────────────────────────────────────────

    private external fun nativeOpen(path: String): Long
    private external fun nativeClose(handle: Long)
    private external fun nativeInsert(handle: Long, key: String, value: String, arpi: ByteArray): Int
    private external fun nativeQuery(handle: Long, key: String): String?
    private external fun nativeDelete(handle: Long, key: String): Int
}
