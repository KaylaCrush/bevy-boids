# Boids / Flocking Simulation

GPU-accelerated flocking using compute shaders in Rust and Bevy

![preview](./assets/img/boids.gif)

---

## Overview

This project implements Craig Reynolds’ Boids algorithm using a GPU-first architecture. The simulation updates thousands of independent agents in parallel using a compute shader written in WGSL, with rendering handled by the Bevy engine in Rust.

Boids follow the standard three steering rules:

1. Alignment – steer toward the average heading of neighbors
2. Cohesion – move toward the center of nearby agents
3. Separation – avoid crowding nearby agents

Running these rules on the GPU enables significantly larger flock sizes and smoother real-time dynamics than a CPU-based implementation.

---

## Key Features

* GPU compute pipeline using WGSL shaders
* Real-time simulation with large agent counts
* Adjustable steering forces (alignment, cohesion, separation)
* Tunable neighbor radius, max speed, turn rate, and noise
* Storage buffers for agent state with ping-pong update pattern
* Rendering and input handled through Bevy

---

## Technical Details

### Compute Shader Pipeline

Each frame consists of:

1. Reading agent positions and velocities from a storage buffer
2. Sampling neighbors within a radius
3. Computing the three steering forces
4. Updating velocity and position
5. Writing results to the alternate storage buffer
6. Rendering the updated agent positions

This structure avoids CPU bottlenecks and scales well for high agent counts.

### Data Layout

Each agent stores:

* `position: vec2<f32>`
* `velocity: vec2<f32>`
* Additional parameters as needed (e.g., speed limits)

All agent data resides in GPU-accessible buffers to minimize CPU-GPU synchronization.

### Performance Notes

* Neighbor checks use squared distance to avoid unnecessary square roots
* Workgroup sizing is tuned for Bevy/WGPU defaults
* The simulation can be extended with spatial hashing or grid partitioning for even larger scales

---

## Running the Project

Requirements:

* Rust
* Cargo
* Bevy (version you used)

Run with:

```bash
cargo run --release
```

---

## Possible Extensions

* Spatial partitioning (uniform grid or quad-tree)
* Multiple boid species with different parameters
* Predator–prey dynamics
* 3D flocking
* Obstacle avoidance
* Integration with other ecosystem simulations

---

## License

MIT