package com.joenoel.bloop.core

/**
 * Kotlin wrapper around the native Rust core lifecycle.
 *
 * [BloopCore] owns the native context handle. Create one instance per session
 * and call [close] when the session ends to release native resources. The class
 * implements [AutoCloseable] so it can be used with Kotlin's `use` extension.
 *
 * [onResponse] is invoked on an arbitrary background thread; callers are
 * responsible for dispatching to the main thread if needed.
 *
 * @throws IllegalStateException if the native core fails to initialise.
 */
class BloopCore(
    private val bloopHome: String,
    private val onResponse: (ByteArray) -> Unit
) : AutoCloseable {

    private val handle: Long = BloopJNI.bloopInit(bloopHome) { data -> onResponse(data) }

    init {
        check(handle != 0L) { "Failed to initialise the Rust core" }
    }

    /**
     * Send a raw Protobuf-encoded request to the core.
     *
     * @return `true` if the request was accepted, `false` otherwise.
     */
    fun sendRequest(request: ByteArray): Boolean {
        return BloopJNI.bloopAddRequest(handle, request) == 0
    }

    /** Shut down the core and free all native resources. */
    override fun close() {
        BloopJNI.bloopShutdown(handle)
    }
}
