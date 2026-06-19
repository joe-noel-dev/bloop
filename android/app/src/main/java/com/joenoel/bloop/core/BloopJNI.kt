package com.joenoel.bloop.core

/**
 * Low-level JNI declarations for the Rust core library.
 *
 * Two native libraries are loaded:
 *  - `bloop`    — the Rust core (libbloop.so), prebuilt by scripts/build-android.sh
 *  - `bloopjni` — the thin C JNI bridge (libbloopjni.so), built by the Android CMake target
 *
 * Consumers should use [BloopCore] rather than calling these functions directly.
 */
internal object BloopJNI {

    init {
        System.loadLibrary("bloop")
        System.loadLibrary("bloopjni")
    }

    /** Receives raw Protobuf response bytes pushed from the Rust core. */
    fun interface ResponseCallback {
        fun onResponse(data: ByteArray)
    }

    /**
     * Initialise the Rust core.
     *
     * @param bloopHome App-owned directory used by core for local state.
     * @param callback Invoked on an arbitrary background thread for each response.
     * @return An opaque handle (> 0) on success, or 0 on failure.
     */
    external fun bloopInit(bloopHome: String, callback: ResponseCallback): Long

    /**
     * Send a raw Protobuf-encoded request to the core.
     *
     * @return 0 on success, non-zero on error (mirrors the native BloopErrorCode).
     */
    external fun bloopAddRequest(ctx: Long, request: ByteArray): Int

    /** Shut down the core and release all native resources for [ctx]. */
    external fun bloopShutdown(ctx: Long)
}
