package com.yume.ui.chat

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.yume.rust.ChatStreamEvent
import com.yume.rust.YumeCore
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.launch

data class ChatUiState(
    val messages: List<ChatMessageItem> = emptyList(),
    val isStreaming: Boolean = false,
    val error: String? = null
)

data class ChatMessageItem(
    val id: String,
    val role: String,
    val content: String,
    val isStreaming: Boolean = false
)

class ChatViewModel : ViewModel() {
    // Use 127.0.0.1 with adb reverse (adb reverse tcp:3000 tcp:3000)
    // Fallback to 10.0.2.2 for standard emulator networking
    private val backendUrl = "http://127.0.0.1:3000"

    private val _uiState = MutableStateFlow(ChatUiState())
    val uiState: StateFlow<ChatUiState> = _uiState.asStateFlow()

    private var conversationId = ""

    init {
        val mode = if (YumeCore.isNativeAvailable) "Native (Rust FFI)" else "HTTP fallback"
        android.util.Log.i("Yume", "ChatViewModel init — mode: $mode")
    }

    fun sendMessage(text: String) {
        if (text.isBlank() || _uiState.value.isStreaming) return

        val userMsgId = java.util.UUID.randomUUID().toString()
        val messages = _uiState.value.messages.toMutableList()

        messages.add(ChatMessageItem(id = userMsgId, role = "user", content = text))

        val assistantMsgId = java.util.UUID.randomUUID().toString()
        messages.add(
            ChatMessageItem(id = assistantMsgId, role = "assistant", content = "", isStreaming = true)
        )

        _uiState.value = ChatUiState(messages = messages, isStreaming = true)

        viewModelScope.launch {
            val fullResponse = StringBuilder()

            YumeCore.sendMessage(backendUrl, text, conversationId)
                .collect { event ->
                    when (event) {
                        is ChatStreamEvent.Started -> {
                            conversationId = event.conversationId
                        }
                        is ChatStreamEvent.Delta -> {
                            fullResponse.append(event.text)
                            updateMessage(assistantMsgId, fullResponse.toString(), true)
                        }
                        is ChatStreamEvent.Done -> {
                            updateMessage(assistantMsgId, fullResponse.toString(), false)
                            _uiState.value = _uiState.value.copy(isStreaming = false)
                        }
                        is ChatStreamEvent.Error -> {
                            if (fullResponse.isEmpty()) {
                                updateMessage(assistantMsgId, "Lỗi: ${event.message}", false)
                            }
                            _uiState.value = _uiState.value.copy(
                                isStreaming = false,
                                error = event.message
                            )
                        }
                    }
                }
        }
    }

    private fun updateMessage(id: String, content: String, streaming: Boolean) {
        val msgs = _uiState.value.messages.toMutableList()
        val i = msgs.indexOfFirst { it.id == id }
        if (i >= 0) {
            msgs[i] = msgs[i].copy(content = content, isStreaming = streaming)
            _uiState.value = _uiState.value.copy(messages = msgs)
        }
    }

    fun clearError() {
        _uiState.value = _uiState.value.copy(error = null)
    }
}
