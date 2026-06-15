package com.yume

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.lifecycle.viewmodel.compose.viewModel
import com.yume.ui.chat.ChatScreen
import com.yume.ui.chat.ChatViewModel
import com.yume.ui.theme.YumeTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        setContent {
            YumeTheme {
                val viewModel: ChatViewModel = viewModel()
                ChatScreen(viewModel = viewModel)
            }
        }
    }
}
