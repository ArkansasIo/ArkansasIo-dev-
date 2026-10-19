# ArkansasIo Development Workspace

This repository currently hosts the **Life Engine Rust** project, a deterministic, text-mode artificial-life simulation.

## Project

- [Life Engine Rust](life-engine-rust/README.md) — Rust simulation core, evolutionary agents, neural control, ecology, physics, statistics, replay descriptors, CLI, and optional WebAssembly boundary.
- [Unified architecture](life-engine-rust/UNIFIED_ENGINE.md) — module boundaries and verification strategy.

## Quick start

Install a stable Rust toolchain, then run:

```sh
cd life-engine-rust
cargo run --release -- --help
cargo run --release -- --seed 1234 --population 100 --ticks 1000
```

Interactive mode:

```sh
cargo run --release -- --interactive
```

## Verification

From `life-engine-rust/`:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo build --release
rustup target add wasm32-unknown-unknown
cargo check --features wasm --target wasm32-unknown-unknown
```

The GitHub Actions workflow runs the same checks for changes under `life-engine-rust/`.

## Repository layout

```text
.
├── README.md
└── life-engine-rust/
    ├── Cargo.toml
    ├── README.md
    ├── UNIFIED_ENGINE.md
    ├── .github/workflows/rust.yml
    ├── src/
    └── tests/
```

## Development notes

- Keep simulation rules independent from rendering.
- Use seeded randomness for reproducible experiments.
- Add regression tests when changing lifecycle, reproduction, replay, or serialization behavior.
- Treat CI as the source of truth for formatting, linting, tests, release compilation, and the optional WASM target.
