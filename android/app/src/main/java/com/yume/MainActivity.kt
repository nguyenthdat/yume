package com.yume

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.lifecycle.ViewModelProvider
import com.yume.ui.chat.ChatScreen
import com.yume.ui.chat.ChatViewModel
import com.yume.ui.theme.YumeTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        val viewModel = ViewModelProvider(this)[ChatViewModel::class.java]

        setContent {
            YumeTheme {
                ChatScreen(viewModel = viewModel)
            }
        }
    }
}
