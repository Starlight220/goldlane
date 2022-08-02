package com.example.api.plugins

import com.example.calculator.Inputs
import com.example.calculator.calculate
import io.ktor.server.routing.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*

fun Application.configureRouting() {
    routing {
        calculatorRequest()
    }
}

/**
 * Define the HTTP API for the calculator.
 */
fun Route.calculatorRequest() {
    get("/") {
        // receive parameters
        val inputs = call.receive<Inputs>()
        // calculate
        val outputs = inputs.calculate()
        // send response
        call.respond(outputs)
    }
}
