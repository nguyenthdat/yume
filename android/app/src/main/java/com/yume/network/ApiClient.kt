package com.yume.network

import com.google.gson.Gson
import com.google.gson.JsonParser
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.callbackFlow
import okhttp3.*
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.RequestBody.Companion.toRequestBody
import okhttp3.sse.*
import java.util.concurrent.TimeUnit

/**
 * HTTP/SSE client for the Yume Rust backend.
 * Android calls ONLY the backend (never OpenCode or DeepSeek directly).
 */
class ApiClient(
    private val baseUrl: String = "http://10.0.2.2:3000"
) {
    private val client = OkHttpClient.Builder()
        .connectTimeout(30, TimeUnit.SECONDS)
        .readTimeout(120, TimeUnit.SECONDS)
        .build()

    private val gson = Gson()
    private val jsonMediaType = "application/json".toMediaType()

    /**
     * Stream chat events from `POST /v1/chat/stream`.
     * Returns a cold Flow of [ChatEvent]s.
     */
    fun streamChat(request: ChatRequest): Flow<ChatEvent> = callbackFlow {
        val jsonBody = gson.toJson(request)
        val body = jsonBody.toRequestBody(jsonMediaType)

        val httpRequest = Request.Builder()
            .url("$baseUrl/v1/chat/stream")
            .post(body)
            .header("Accept", "text/event-stream")
            .build()

        val listener = object : EventSourceListener() {
            override fun onEvent(
                eventSource: EventSource,
                id: String?,
                type: String?,
                data: String
            ) {
                try {
                    val event = parseSseEvent(type ?: "message", data)
                    if (event != null) {
                        trySend(event)
                    }
                } catch (_: Exception) {
                    // Skip malformed events
                }
            }

            override fun onClosed(eventSource: EventSource) {
                close()
            }

            override fun onFailure(
                eventSource: EventSource,
                t: Throwable?,
                response: Response?
            ) {
                trySend(
                    ChatEvent.Error(
                        code = "CONNECTION_ERROR",
                        message = t?.message ?: "Connection failed",
                        recoverable = true
                    )
                )
                close(t)
            }
        }

        val factory = EventSources.createFactory(client)
        val eventSource = factory.newEventSource(httpRequest, listener)

        awaitClose {
            eventSource.cancel()
        }
    }

    private fun parseSseEvent(type: String, data: String): ChatEvent? {
        if (data.isBlank() || data == "[DONE]") return ChatEvent.Done("stop")

        val json = JsonParser.parseString(data).asJsonObject

        return when (type) {
            "chat.started" -> ChatEvent.ChatStarted(
                conversation_id = json.get("conversation_id").asString,
                message_id = json.get("message_id").asString
            )

            "message.delta" -> ChatEvent.MessageDelta(
                seq = json.get("seq").asLong,
                delta = TextDelta(
                    text = json.getAsJsonObject("delta").get("text").asString
                )
            )

            "usage" -> ChatEvent.Usage(
                input_tokens = json.get("input_tokens").asLong,
                output_tokens = json.get("output_tokens").asLong
            )

            "done" -> ChatEvent.Done(
                finish_reason = json.get("finish_reason").asString
            )

            "error" -> ChatEvent.Error(
                code = json.get("code").asString,
                message = json.get("message").asString,
                retry_after_ms = json.get("retry_after_ms")?.asLong,
                recoverable = json.get("recoverable")?.asBoolean ?: false
            )

            else -> null
        }
    }
}
