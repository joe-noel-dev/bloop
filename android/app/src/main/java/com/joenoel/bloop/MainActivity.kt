package com.joenoel.bloop

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.lifecycle.viewmodel.compose.viewModel
import com.joenoel.bloop.state.AppStoreViewModel
import com.joenoel.bloop.ui.BloopApp
import com.joenoel.bloop.ui.theme.BloopTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        setContent {
            val store: AppStoreViewModel = viewModel()

            BloopTheme {
                BloopApp(store)
            }
        }
    }
}
