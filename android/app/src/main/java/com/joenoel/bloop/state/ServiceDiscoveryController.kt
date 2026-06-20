package com.joenoel.bloop.state

import android.content.Context
import android.net.nsd.NsdManager
import android.net.nsd.NsdServiceInfo
import androidx.core.content.ContextCompat

interface ServiceDiscoveryController {
    fun restart(
        onScanningChanged: (Boolean) -> Unit,
        onServersChanged: (List<ServerEndpoint>) -> Unit,
        onError: (String) -> Unit,
    )
}

class AndroidNsdServiceDiscoveryController(
    context: Context,
) : ServiceDiscoveryController {

    private val serviceType = "_bloop._tcp"
    private val normalizedServiceType = normalizeServiceType(serviceType)
    private val nsdManager = context.getSystemService(Context.NSD_SERVICE) as NsdManager
    private val callbackExecutor = ContextCompat.getMainExecutor(context)
    private val lock = Any()

    private var scanningCallback: ((Boolean) -> Unit)? = null
    private var serversCallback: ((List<ServerEndpoint>) -> Unit)? = null
    private var errorCallback: ((String) -> Unit)? = null
    private var discoveryListener: NsdManager.DiscoveryListener? = null

    private val servicesByName = mutableMapOf<String, ServerEndpoint.HostPort>()

    override fun restart(
        onScanningChanged: (Boolean) -> Unit,
        onServersChanged: (List<ServerEndpoint>) -> Unit,
        onError: (String) -> Unit,
    ) {
        synchronized(lock) {
            scanningCallback = onScanningChanged
            serversCallback = onServersChanged
            errorCallback = onError
            servicesByName.clear()
            notifyServersLocked()
        }

        stopDiscoveryIfNeeded()
        startDiscovery()
    }

    private fun startDiscovery() {
        val listener = object : NsdManager.DiscoveryListener {
            override fun onStartDiscoveryFailed(serviceType: String, errorCode: Int) {
                synchronized(lock) {
                    notifyScanningLocked(false)
                    errorCallback?.invoke("Service discovery failed to start ($errorCode)")
                }
                stopDiscoveryIfNeeded()
            }

            override fun onStopDiscoveryFailed(serviceType: String, errorCode: Int) {
                synchronized(lock) {
                    notifyScanningLocked(false)
                    errorCallback?.invoke("Service discovery failed to stop ($errorCode)")
                }
                stopDiscoveryIfNeeded()
            }

            override fun onDiscoveryStarted(serviceType: String) {
                synchronized(lock) {
                    notifyScanningLocked(true)
                }
            }

            override fun onDiscoveryStopped(serviceType: String) {
                synchronized(lock) {
                    notifyScanningLocked(false)
                }
            }

            override fun onServiceFound(serviceInfo: NsdServiceInfo) {
                if (normalizeServiceType(serviceInfo.serviceType) != normalizedServiceType) {
                    return
                }
                resolveService(serviceInfo)
            }

            override fun onServiceLost(serviceInfo: NsdServiceInfo) {
                synchronized(lock) {
                    servicesByName.remove(serviceInfo.serviceName)
                    notifyServersLocked()
                }
            }
        }

        synchronized(lock) {
            discoveryListener = listener
        }
        try {
            nsdManager.discoverServices(
                serviceType,
                NsdManager.PROTOCOL_DNS_SD,
                null,
                callbackExecutor,
                listener
            )
        } catch (securityException: SecurityException) {
            synchronized(lock) {
                notifyScanningLocked(false)
                errorCallback?.invoke(
                    "Discovery permission missing: ${securityException.message ?: "unknown error"}"
                )
            }
        }
    }

    @Suppress("DEPRECATION")
    private fun resolveService(serviceInfo: NsdServiceInfo) {
        nsdManager.resolveService(
            serviceInfo,
            object : NsdManager.ResolveListener {
                override fun onResolveFailed(serviceInfo: NsdServiceInfo, errorCode: Int) {
                    synchronized(lock) {
                        errorCallback?.invoke(
                            "Failed to resolve ${serviceInfo.serviceName} ($errorCode)"
                        )
                    }
                }

                override fun onServiceResolved(serviceInfo: NsdServiceInfo) {
                    val host = serviceInfo.host?.hostAddress ?: return
                    if (serviceInfo.port <= 0) {
                        return
                    }

                    synchronized(lock) {
                        servicesByName[serviceInfo.serviceName] =
                            ServerEndpoint.HostPort(host = host, port = serviceInfo.port)
                        notifyServersLocked()
                    }
                }
            }
        )
    }

    private fun stopDiscoveryIfNeeded() {
        val listener = synchronized(lock) {
            val current = discoveryListener
            discoveryListener = null
            current
        }

        if (listener != null) {
            try {
                nsdManager.stopServiceDiscovery(listener)
            } catch (_: IllegalArgumentException) {
                synchronized(lock) {
                    notifyScanningLocked(false)
                }
            }
        }
    }

    private fun notifyServersLocked() {
        serversCallback?.invoke(
            servicesByName
                .values
                .sortedWith(compareBy({ it.host }, { it.port }))
        )
    }

    private fun notifyScanningLocked(scanning: Boolean) {
        scanningCallback?.invoke(scanning)
    }

    private fun normalizeServiceType(type: String): String {
        return type.trim().trimEnd('.').lowercase()
    }
}
