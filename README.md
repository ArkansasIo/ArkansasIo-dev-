# ArkansasIo Development Workspace

This repository hosts the **Life Engine Rust** project and its documentation, plus a lightweight GitHub Pages landing page.

## Website

The static site lives at the repository root:

- `index.html` — landing page
- `assets/site.css` — responsive dark command-center theme
- `assets/site.js` — small clipboard helper
- `.github/workflows/pages.yml` — validates and deploys the root site to GitHub Pages

### Enable GitHub Pages

1. Open **Settings → Pages** in this repository.
2. Under **Build and deployment**, choose **GitHub Actions** as the source.
3. Push or merge the site files to `main`, then check the **Actions** tab for the deployment.
4. GitHub will show the published URL under **Settings → Pages** and on the workflow deployment environment.

The workflow deploys only the static root site. It does not compile or execute Rust in a browser.

## Life Engine Rust

[Life Engine Rust](life-engine-rust/README.md) is a deterministic, text-mode artificial-life simulator with evolutionary agents, neural control, ecology, physics primitives, statistics, replay descriptors, CLI, and an optional WebAssembly boundary.

### Quick start

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

### Verification

From `life-engine-rust/`:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo build --release
rustup target add wasm32-unknown-unknown
cargo check --features wasm --target wasm32-unknown-unknown
```

Rust verification is configured in `life-engine-rust/.github/workflows/rust.yml`.

## Project links

- [Engine documentation](life-engine-rust/README.md)
- [Unified architecture](life-engine-rust/UNIFIED_ENGINE.md)
- [Source code](life-engine-rust/src/)
