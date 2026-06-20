package com.joenoel.bloop

import android.Manifest
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.result.contract.ActivityResultContracts
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.core.content.ContextCompat
import androidx.lifecycle.viewmodel.compose.viewModel
import com.joenoel.bloop.state.AppStoreViewModel
import com.joenoel.bloop.state.AppStoreViewModelFactory
import com.joenoel.bloop.ui.BloopApp
import com.joenoel.bloop.ui.theme.BloopTheme
import java.io.File

class MainActivity : ComponentActivity() {
    private val permissionLauncher =
        registerForActivityResult(ActivityResultContracts.RequestMultiplePermissions()) { }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        ensureDiscoveryPermissions()
        val bloopHome = File(filesDir, "bloop").absolutePath

        setContent {
            val store: AppStoreViewModel = viewModel(
                factory = AppStoreViewModelFactory(
                    appContext = applicationContext,
                    bloopHome = bloopHome,
                )
            )

            BloopTheme {
                BloopApp(store)
            }
        }
    }

    private fun ensureDiscoveryPermissions() {
        val permission = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            Manifest.permission.NEARBY_WIFI_DEVICES
        } else {
            Manifest.permission.ACCESS_FINE_LOCATION
        }

        if (ContextCompat.checkSelfPermission(this, permission) != PackageManager.PERMISSION_GRANTED) {
            permissionLauncher.launch(arrayOf(permission))
        }
    }
}
