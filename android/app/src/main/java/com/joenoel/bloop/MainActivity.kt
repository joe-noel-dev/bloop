package com.joenoel.bloop

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.lifecycle.viewmodel.compose.viewModel
import com.joenoel.bloop.state.AppStoreViewModel
import com.joenoel.bloop.state.AppStoreViewModelFactory
import com.joenoel.bloop.ui.BloopApp
import com.joenoel.bloop.ui.theme.BloopTheme
import java.io.File

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        val bloopHome = File(filesDir, "bloop").absolutePath

        setContent {
            val store: AppStoreViewModel = viewModel(
                factory = AppStoreViewModelFactory(bloopHome)
            )

            BloopTheme {
                BloopApp(store)
            }
        }
    }
}
