package com.example.api

import io.ktor.server.engine.*
import io.ktor.server.netty.*
import com.example.api.plugins.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.application.*
import io.ktor.server.plugins.contentnegotiation.*

fun main() {
    embeddedServer(Netty, port = 6060, host = "127.0.0.1") {
        install(ContentNegotiation) {
            json()
        }
        configureRouting()
    }.start(wait = true)
}
