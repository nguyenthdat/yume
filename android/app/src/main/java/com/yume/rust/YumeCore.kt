package com.yume.rust

import kotlinx.coroutines.channels.ProducerScope
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.callbackFlow

/**
 * Clean Kotlin API wrapping the UniFFI-generated Rust bindings.
 *
 * Falls back to HTTP/OkHttp when the native library isn't available
 * (e.g. during development without cross-compilation).
 */
object YumeCore {

    val isNativeAvailable: Boolean by lazy {
        try {
            System.loadLibrary("yume_ffi")
            true
        } catch (_: UnsatisfiedLinkError) {
            false
        }
    }

    fun sendMessage(
        backendUrl: String,
        message: String,
        conversationId: String,
    ): Flow<ChatStreamEvent> = callbackFlow {
        if (isNativeAvailable) {
            sendNative(backendUrl, message, conversationId)
        } else {
            sendHttp(backendUrl, message, conversationId)
        }
        awaitClose {}
    }

    // ------------------------------------------------------------------
    // Native path (UniFFI → libyume_ffi.so)
    // ------------------------------------------------------------------

    private fun ProducerScope<ChatStreamEvent>.sendNative(
        backendUrl: String,
        message: String,
        conversationId: String,
    ) {
        try {
            val cb = object : uniffi.yume.ChatCallback {
                override fun onStarted(conversationId: String, messageId: String) {
                    trySend(ChatStreamEvent.Started(conversationId, messageId))
                }
                override fun onDelta(seq: ULong, text: String) {
                    trySend(ChatStreamEvent.Delta(seq.toLong(), text))
                }
                override fun onDone(finishReason: String) {
                    trySend(ChatStreamEvent.Done(finishReason))
                    close()
                }
                override fun onError(code: String, message: String) {
                    trySend(ChatStreamEvent.Error(code, message))
                    close()
                }
            }
            uniffi.yume.`sendChatMessage`(backendUrl, message, conversationId, cb)
        } catch (e: Exception) {
            trySend(ChatStreamEvent.Error("FFI_ERROR", e.message ?: "Unknown"))
            close()
        }
    }

    // ------------------------------------------------------------------
    // Fallback path (OkHttp — works without native library)
    // ------------------------------------------------------------------

    private suspend fun ProducerScope<ChatStreamEvent>.sendHttp(
        backendUrl: String,
        message: String,
        conversationId: String,
    ) {
        val client = com.yume.network.ApiClient(backendUrl)
        val request = com.yume.network.ChatRequest(
            conversation_id = conversationId,
            message = com.yume.network.ChatMessage(content = message),
            stream = true
        )
        client.streamChat(request).collect { event ->
            when (event) {
                is com.yume.network.ChatEvent.ChatStarted ->
                    trySend(ChatStreamEvent.Started(event.conversation_id, event.message_id))
                is com.yume.network.ChatEvent.MessageDelta ->
                    trySend(ChatStreamEvent.Delta(event.seq, event.delta.text))
                is com.yume.network.ChatEvent.Done -> {
                    trySend(ChatStreamEvent.Done(event.finish_reason))
                    close()
                }
                is com.yume.network.ChatEvent.Error -> {
                    trySend(ChatStreamEvent.Error(event.code, event.message))
                    close()
                }
                else -> {}
            }
        }
    }
}

sealed class ChatStreamEvent {
    data class Started(val conversationId: String, val messageId: String) : ChatStreamEvent()
    data class Delta(val seq: Long, val text: String) : ChatStreamEvent()
    data class Done(val finishReason: String) : ChatStreamEvent()
    data class Error(val code: String, val message: String) : ChatStreamEvent()
}
