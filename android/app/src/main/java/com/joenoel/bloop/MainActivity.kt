package com.joenoel.bloop

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import com.joenoel.bloop.ui.BloopApp
import com.joenoel.bloop.ui.theme.BloopTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        setContent {
            BloopTheme {
                BloopApp()
            }
        }
    }
}
