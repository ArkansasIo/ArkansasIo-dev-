# Life Engine Unified

This project is a clean-room Rust implementation inspired by the documented mechanics of several artificial-life systems. It does not copy proprietary code.

## Selected foundation

The strongest feature foundation found was **ALIEN (Artificial LIfe ENvironment)**: GPU particle physics, multicellular particle organisms, genomes, neural control, sensors/muscles/weapons/constructors, spatial parameters, editing, and simulation networking. ALIEN is BSD-3-Clause licensed and supports CUDA; current upstream documentation also describes AMD HIP support.

The original **Life Engine** contributes cellular-automa evolution mechanics: food, walls, mouth, producer, mover, killer, armor, eye, reproduction, mutation, movement and emergent brain behavior.

Other useful concepts incorporated at the architecture level include neural-network evolution, cyclic neural graphs, crossover, lineage tracking, ecological resources, deterministic seeds, and terminal/headless execution.

## Unified feature model

1. Deterministic simulation clock and seeded RNG.
2. Spatial world and environmental fields.
3. Resources: energy, water and minerals.
4. Organisms composed of genetic cells/particles.
5. Mouth, producer, mover, killer, armor, eye, sensor, muscle, constructor, storage and receptor capabilities.
6. Energy, health, age and lifespan.
7. Reproduction with mutation and optional crossover.
8. Natural selection through survival and reproduction rather than an externally imposed fitness target.
9. Neural decision layer for seeking food, fleeing threats, finding mates, attacking and exploration.
10. Particle physics primitives for motion, mass, health and material types.
11. Predator/prey ecology and environmental pressure.
12. Lineage and generation tracking.
13. Headless CLI operation.
14. Future GPU backend boundary for CUDA/HIP.
15. Future WASM/browser renderer boundary.
16. Save/replay/statistics interfaces planned as separate modules.

## Design rule

The engine combines **compatible concepts**, not source-code copies. Each subsystem is implemented independently so licenses and attribution requirements remain manageable.

## Recommended evolution path

- Phase 1: CPU deterministic core and terminal sandbox.
- Phase 2: full cell-body topology and collision system.
- Phase 3: neural genome encoding and evolved recurrent networks.
- Phase 4: CUDA/HIP parallel simulation backend.
- Phase 5: interactive desktop editor and visual renderer.
- Phase 6: WebAssembly viewer and simulation replay format.
- Phase 7: distributed simulation/experiment runner.
