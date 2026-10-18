# Life Engine Rust

A clean-room Rust implementation inspired by the public design of **The Life Engine**: a cellular artificial-life simulation in which organisms consume food, move, reproduce, mutate, and develop simple perception/brain traits. The reference project describes a grid environment, organism cells, natural selection, mutation, movement, and eye/brain behavior. This project implements those concepts independently rather than copying its source code.

## Why Rust

Rust is the target language for this clone because it provides predictable performance, memory safety, strong data modeling, deterministic RNG options, and a natural path to a headless simulation, terminal UI, WebAssembly build, or GPU renderer later.

## Current implementation

- Deterministic seeded simulation
- Text/ASCII world renderer
- Grid environment with food
- Organism cell types: mouth, producer, mover, killer, armor, eye
- Energy, age, lifespan, death
- Reproduction and inheritance
- Mutation: cell-type change, cell addition, cell loss
- Simple eye/mover brain classification
- Population cap
- Configurable simulation parameters

## Run

```bash
cargo run --release
```

The default simulation uses an 80x30 terminal world and advances for up to 10,000 ticks.

## Architecture

```text
life-engine-rust/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs       # executable and simulation loop
    ├── config.rs     # tunable world parameters
    ├── organism.rs   # cells, organisms, mutation
    ├── brain.rs      # perception/response model
    └── world.rs      # environment, tick pipeline, renderer
```

## Roadmap

1. Complete directional eyes and chase/retreat behavior.
2. Add spatially correct multi-cell bodies and collision rules.
3. Add predator damage and armor mechanics.
4. Add lineage IDs and ancestry tracking.
5. Add statistics export (CSV/JSON).
6. Add replay files from `(seed, configuration, inputs)`.
7. Add terminal controls for pause, speed, reset, and mutation rates.
8. Add optional WebAssembly frontend.
9. Add optional Bevy/wgpu visualization without coupling it to the simulation core.

## Reference

The original Life Engine is available at https://github.com/MaxRobinsonTheGreat/LifeEngine and is GPL-3.0. This repository is a clean-room implementation and does not copy its source code.
