package com.joenoel.bloop

import android.Manifest
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.result.contract.ActivityResultContracts
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.viewModels
import androidx.core.content.ContextCompat
import com.joenoel.bloop.state.AppStoreViewModel
import com.joenoel.bloop.state.AppStoreViewModelFactory
import com.joenoel.bloop.ui.BloopApp
import com.joenoel.bloop.ui.theme.BloopTheme
import java.io.File

class MainActivity : ComponentActivity() {
    private val store: AppStoreViewModel by viewModels {
        AppStoreViewModelFactory(
            appContext = applicationContext,
            bloopHome = File(filesDir, "bloop").absolutePath,
        )
    }

    private val permissionLauncher =
        registerForActivityResult(ActivityResultContracts.RequestMultiplePermissions()) { result ->
            if (result.values.any { it }) {
                recreate()
            }
        }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        ensureDiscoveryPermissions()

        setContent {
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
