// Copyright (c) 2026 Edison Lepiten / AIEONYX
// License: Apache-2.0
//
// ExposureStore — interface that exactly mirrors the Room ExposureDao contract.
// Swap implementations without touching any call site.

package com.aieonyx.aistop.db

import com.aieonyx.aistop.model.ExposureEvent
import kotlinx.coroutines.flow.Flow

interface ExposureStore {

    /** Insert a single exposure event. */
    suspend fun insert(event: ExposureEvent)

    /** Return all events, newest first. */
    fun getAllEvents(): Flow<List<ExposureEvent>>

    /** Return events for a specific app package. */
    fun getEventsForApp(packageName: String): Flow<List<ExposureEvent>>

    /** Return the total count of recorded events. */
    suspend fun count(): Int

    /** Delete a single event by its ID. */
    suspend fun delete(event: ExposureEvent)

    /** Wipe all events (GDPR Art.17 — right to erasure). */
    suspend fun deleteAll()

    /** Return events in the given time range (Unix ms). */
    fun getEventsBetween(startMs: Long, endMs: Long): Flow<List<ExposureEvent>>
}
