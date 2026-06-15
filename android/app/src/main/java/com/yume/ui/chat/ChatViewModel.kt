package com.yume.ui.chat

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.yume.network.*
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.launch

/** UI state for the chat screen. */
data class ChatUiState(
    val messages: List<ChatMessageItem> = emptyList(),
    val isStreaming: Boolean = false,
    val error: String? = null
)

/** A single visible message bubble in the chat. */
data class ChatMessageItem(
    val id: String,
    val role: String, // "user" or "assistant"
    val content: String,
    val isStreaming: Boolean = false
)

class ChatViewModel : ViewModel() {
    private val apiClient = ApiClient()

    private val _uiState = MutableStateFlow(ChatUiState())
    val uiState: StateFlow<ChatUiState> = _uiState.asStateFlow()

    private var conversationId = ""

    fun sendMessage(text: String) {
        if (text.isBlank()) return

        val userMsgId = java.util.UUID.randomUUID().toString()
        val currentMessages = _uiState.value.messages.toMutableList()

        currentMessages.add(
            ChatMessageItem(id = userMsgId, role = "user", content = text)
        )

        val assistantMsgId = java.util.UUID.randomUUID().toString()
        currentMessages.add(
            ChatMessageItem(
                id = assistantMsgId,
                role = "assistant",
                content = "",
                isStreaming = true
            )
        )

        _uiState.value = _uiState.value.copy(
            messages = currentMessages,
            isStreaming = true,
            error = null
        )

        viewModelScope.launch {
            val request = ChatRequest(
                conversation_id = conversationId,
                message = ChatMessage(content = text),
                stream = true
            )

            val fullResponse = StringBuilder()

            apiClient.streamChat(request).collect { event ->
                when (event) {
                    is ChatEvent.ChatStarted -> {
                        conversationId = event.conversation_id
                    }

                    is ChatEvent.MessageDelta -> {
                        fullResponse.append(event.delta.text)
                        updateAssistantMessage(
                            assistantMsgId,
                            fullResponse.toString(),
                            isStreaming = true
                        )
                    }

                    is ChatEvent.Done -> {
                        updateAssistantMessage(
                            assistantMsgId,
                            fullResponse.toString(),
                            isStreaming = false
                        )
                        _uiState.value = _uiState.value.copy(isStreaming = false)
                    }

                    is ChatEvent.Error -> {
                        if (fullResponse.isEmpty()) {
                            updateAssistantMessage(
                                assistantMsgId,
                                "Lỗi: ${event.message}",
                                isStreaming = false
                            )
                        }
                        _uiState.value = _uiState.value.copy(
                            isStreaming = false,
                            error = event.message
                        )
                    }

                    else -> { /* ignore usage, citation for now */ }
                }
            }
        }
    }

    private fun updateAssistantMessage(
        id: String,
        content: String,
        isStreaming: Boolean
    ) {
        val messages = _uiState.value.messages.toMutableList()
        val index = messages.indexOfFirst { it.id == id }
        if (index >= 0) {
            messages[index] = messages[index].copy(
                content = content,
                isStreaming = isStreaming
            )
            _uiState.value = _uiState.value.copy(messages = messages)
        }
    }

    fun clearError() {
        _uiState.value = _uiState.value.copy(error = null)
    }
}
