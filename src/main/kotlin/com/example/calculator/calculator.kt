package com.example.calculator

import kotlinx.serialization.Serializable
import kotlin.math.*

/**
 * @property v0 Projectile's launch velocity (in m/s).
 * @property theta0 Angle of launch (above horizontal, in radians).
 * @property h0 Projectile's initial height (in m).
 */
@Serializable
data class Inputs(val v0: Double, val theta0: Double, val h0: Double)

/**
 * @property x Displacement (in m).
 * @property v Velocity at impact (in m/s).
 * @property theta Angle at impact (in radians).
 */
@Serializable
data class Outputs(val x: Double, val v: Double, val theta: Double)

/**
 * Constant of gravitational acceleration on Earth.
 */
const val g = 9.8

/**
 * Calculate the projectile's range and impact velocity vector
 */
fun Inputs.calculate(): Outputs {
    val vx = v0 * cos(theta0)
    val v0y = v0 * sin(theta0)

    /**
     * Displacement.
     */
    val dx = vx * (v0y + sqrt(v0y.pow(2) + 2 * g * h0)) / g

    // At peak of motion, the projectile's energy is all potential and defined by `E = mgh`
    // At impact, the projectile's energy is all kinetic and defined by `E = 0.5mv^2`.
    // Solving for `v` gives us `v = sqrt(2gh)` (where `h` is the height at the peak of motion)
    val peakHeight = h0 + v0y.pow(2) / (2 * g)

    /**
     * Velocity at impact.
     */
    val v = sqrt(2 * g * peakHeight)


    // vx isn't affected by gravity, so it remains constant throughout the projectile's flight.
    // FIXME: this will need to be changed if wind resistance is not ignored!
    /**
     * Angle of impact.
     */
    val crashAngle = acos(vx / v)

    return Outputs(
        x = dx,
        v = v,
        theta = crashAngle
    )
}
