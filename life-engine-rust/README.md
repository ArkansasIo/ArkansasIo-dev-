# Life Engine Rust

A clean-room Rust artificial-life simulator combining cellular organisms, evolutionary mutation, neural decision-making, ecology, deterministic replay, physics primitives, and a text-mode CLI. The implementation is independent of the public projects that inspired its feature set.

## Features

- Deterministic seeded simulation with a persistent ChaCha8 RNG
- Cellular organism model: mouth, producer, mover, killer, armor, eye, sensor, muscle, constructor, storage, receptor
- Genome mutation and two-parent crossover
- Generation and parent tracking
- Energy, health, age, lifespan, population pressure, and resource consumption
- Neural networks with seeded weights and mutation
- Behaviour decisions: seek food, flee threats, seek mates, attack, and explore
- Ecosystem resources: energy, water, minerals, climate, fertility, toxicity
- Particle physics primitives
- Headless batch execution
- Interactive terminal controls
- CSV and JSON statistics export
- Deterministic replay descriptors
- Optional WebAssembly API boundary
- GitHub Actions checks for formatting, Clippy, tests, release build, and WASM compilation

## Run

From `life-engine-rust/`:

```bash
cargo run --release -- --seed 1234 --population 100 --ticks 1000
```

For the interactive terminal console:

```bash
cargo run --release -- --interactive
```

Commands include:

```text
help
step [n]
run [n]
stats
save-csv <file>
save-json <file>
replay <file>
reset
quit
```

## Test and verify

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo build --release
```

Optional browser/WASM boundary:

```bash
rustup target add wasm32-unknown-unknown
cargo check --features wasm --target wasm32-unknown-unknown
```

## Architecture

```text
life-engine-rust/
├── Cargo.toml
├── README.md
├── UNIFIED_ENGINE.md
├── .github/workflows/rust.yml
├── src/
│   ├── main.rs       # CLI and interactive console
│   ├── lib.rs        # public crate API
│   ├── config.rs     # simulation configuration
│   ├── world.rs      # grid/cellular world
│   ├── organism.rs   # cellular organisms
│   ├── genome.rs     # genes and mutation
│   ├── evolution.rs  # crossover and selection helpers
│   ├── neural.rs     # neural network and decisions
│   ├── ecology.rs    # climate and resources
│   ├── physics.rs    # vector/particle primitives
│   ├── unified.rs    # integrated evolutionary simulation
│   ├── stats.rs      # CSV/JSON statistics
│   ├── replay.rs     # deterministic replay descriptor
│   └── wasm.rs       # optional WASM API
└── tests/
    └── engine_tests.rs
```

## Determinism

A simulation is reproducible from its seed, population, and engine version/configuration. The unified engine owns a persistent seeded RNG rather than reseeding each tick. Replay files record the seed, population, and number of ticks needed to reproduce a run.

## Design boundaries

The CPU engine is intentionally independent from rendering. A future Bevy/wgpu, CUDA/HIP, or browser frontend can consume the public engine API without changing the simulation rules. The optional WASM module exposes a minimal browser-friendly stepping interface today.

## Licensing and inspiration

This is a clean-room implementation. It combines general artificial-life concepts such as cellular evolution, genomes, neural controllers, ecological resources, and particle simulation without copying source code from reference projects. See `UNIFIED_ENGINE.md` for the architecture rationale.
