package com.joenoel.bloop.state

import android.content.Context
import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider

class AppStoreViewModelFactory(
    private val appContext: Context,
    private val bloopHome: String,
) : ViewModelProvider.Factory {
    override fun <T : ViewModel> create(modelClass: Class<T>): T {
        if (modelClass.isAssignableFrom(AppStoreViewModel::class.java)) {
            @Suppress("UNCHECKED_CAST")
            return AppStoreViewModel(
                middlewares = listOf(
                    AppCodecMiddleware(),
                    ResponseMiddleware(),
                    DiscoveryMiddleware(
                        AndroidNsdServiceDiscoveryController(appContext)
                    ),
                    LocalCoreMiddleware(bloopHome = bloopHome),
                    RemoteMiddleware(),
                )
            ) as T
        }
        throw IllegalArgumentException("Unknown ViewModel class: ${modelClass.name}")
    }
}
