# Life Engine Unified Architecture

The repository now provides a cohesive CPU artificial-life core rather than a collection of disconnected experiments. The implementation remains a clean-room project and uses only general, independently implemented concepts.

## Runtime layers

```text
CLI / WASM boundary
        |
   LifeEngine
   /   |    \
Agents Ecology Statistics
  |      |      |
Genome Neural Physics Replay
  |
Mutation + Crossover
```

### LifeEngine

`unified.rs` owns the simulation clock, persistent deterministic RNG, agent population, reproduction, births/deaths, ecosystem integration, and population limits.

### Agents

Each agent carries a genome, neural controller, energy, age, generation, parent ID, and particle state. Agents perceive environmental signals and use their neural controller plus the deterministic behaviour layer to choose movement pressure.

### Genome and evolution

Genomes contain spatial cell genes and heritable traits. Mutation can change cell kinds, add/remove genes, and alter movement range. Two-parent crossover combines genes and averages core heritable parameters before mutation.

### Neural control

The neural module provides a small feed-forward network with deterministic/random initialization, bounded mutation, evaluation, and a high-level action classifier. It is deliberately renderer-independent so a more advanced recurrent or graph network can replace it later.

### Ecology

The ecosystem tracks energy, water, minerals, fertility, temperature, radiation, toxicity, predation, and seasons. Resources regenerate while living agents consume energy, creating environmental pressure.

### Physics

Particles provide position, velocity, mass, health, integration, and damage primitives. The current unified simulation uses these primitives without coupling them to a graphical renderer.

### Statistics and replay

`stats.rs` exports time-series CSV and JSON documents. `replay.rs` defines a stable text descriptor containing the seed, population, and tick count needed to reproduce a simulation configuration.

### CLI

The binary supports batch runs and an interactive console with stepping, running, reset, statistics, CSV/JSON export, and replay generation.

### WASM

The optional `wasm` feature exposes `WasmLifeEngine` through `wasm-bindgen`. This is a minimal browser boundary for stepping and inspecting the simulation; a browser renderer can be layered on later.

## Verification strategy

CI runs:

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test --all`
4. `cargo build --release`
5. WASM target compilation/check

The repository also contains unit tests for the neural layer, replay, statistics, and unified engine plus integration tests covering deterministic initialization, simulation progress, crossover, mutation bounds, and decision priority.

## Future renderer boundary

The simulation core deliberately has no dependency on Bevy, wgpu, CUDA, or HIP. Those systems can be implemented as adapters over snapshots/events. This keeps simulation correctness and deterministic tests independent from GPU availability.
