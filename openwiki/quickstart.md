---
type: engineering task-routing guide
title: Ultra Engine Wiki Guide
description: Start here to choose the right runtime domain, workflow, and validation path for a Rust Ultra Engine change. It identifies the workspace entry points, the runnable demo, and the architecture pages that define the contracts a change must preserve.
tags: [quickstart, navigation, rust, game-engine, ecs, validation]
verified:
  - by: openwiki/0.5.0
    at: 2026-09-09T04:05:08.193Z
sources:
  - id: openwiki-source-886fbb2336c3635ef8adb3c3
    resource: repo://.github/workflows/rust.yml
  - id: openwiki-source-651d1fb6c9e49916a916ab51
    resource: repo://Cargo.toml
  - id: openwiki-source-06f743e0bf1d940408f29fcb
    resource: repo://engine/Cargo.toml
  - id: openwiki-source-5a99994eeb2e588b0d7e125a
    resource: repo://engine/src/lib.rs
  - id: openwiki-source-a3ded95da5e0b918918b81b9
    resource: repo://engine/src/scene/scene_changer_resource.rs
  - id: openwiki-source-b9c584eef33a510c58625d64
    resource: repo://engine/src/scene/scene_services.rs
  - id: openwiki-source-2c5adbcfbece31368fdc54dd
    resource: repo://engine/src/scene/scene.rs
  - id: openwiki-source-55f7427857109da8a3699d91
    resource: repo://engine/tests/dynamic_aabb_tree_heap_test.rs
  - id: openwiki-source-c07ecc384163c3d60f39ec1c
    resource: repo://engine/tests/epa_heap_test.rs
  - id: openwiki-source-87c182ba6b704f44bdb0714b
    resource: repo://game/Cargo.toml
  - id: openwiki-source-b76610d665b3d6fb48b86d70
    resource: repo://game/src/main.rs
generated: { by: "openwiki/0.5.0", at: "2026-09-06T19:22:21.487Z" }
---

Ultra Engine is a two-member Cargo workspace: the reusable `engine` library supplies the runtime, while the `Game` binary is the runnable ECS demo and application composition root. Start with the change's behavior, not a directory: most work crosses an engine schedule, scene resource, or device-backed boundary.

## Run the right entry point

Run commands from the repository root.

```bash
# Compile both workspace members.
cargo build --workspace

# Run the Rust test suite for both members.
cargo test --workspace

# Run the interactive demo.
cargo run -p Game
```

`Game` is the package and binary name, and its binary is `game/src/main.rs`. It creates `Engine`, loads the demo's relative `resources/...` assets, configures its scene, and calls `engine.run()`. The last command is therefore an interactive graphical/audio smoke test, not a headless test: use a native environment with a display, an OpenGL-capable driver, and a default audio output device. For platform packages, failure behavior, profiles, and CI scope, use [Build, Platform Dependencies, and Automation](/openwiki/operations/build-and-ci.md).

## Choose a documentation route by task

| If the change affects... | Read this first | Then check |
| --- | --- | --- |
| Engine ownership, frame ordering, input polling, fixed-step timing, render presentation, or end-of-frame work | [Engine Runtime and Crate Boundary](/openwiki/architecture/engine-runtime.md) | Preserve the engine/game ownership boundary and phase ordering; manually run `Game` for lifecycle or device work. |
| Entity composition, transforms, handles, scene-local state, or shared assets across replacement | [ECS Scene, Components, and Shared Resource Ownership](/openwiki/concepts/ecs-scenes-and-resources.md) | [Game Bootstrap, Input Controllers, and Scene Switching](/openwiki/workflows/game-bootstrap-and-scene-switching.md) for the demo's concrete setup. |
| Colliders, contact generation, manifolds, impulses, integration, sleeping, or collision listeners | [Physics, Collision Detection, and Events](/openwiki/concepts/physics-and-collision.md) | [Testing, Numerical Regression, and Allocation Checks](/openwiki/testing/validation-and-performance.md) for tolerances, edge cases, and allocation regressions. |
| Model/image import, asset handles, render bodies, materials, shaders, queue construction, culling, or GPU upload/draw behavior | [Asset Import and OpenGL Rendering](/openwiki/integrations/assets-and-rendering.md) | Runtime guide for the render-before-simulation boundary; smoke-test actual assets and OpenGL output. |
| WAV loading, source/listener components, hit sounds, spatial position updates, mixer commands, or callback behavior | [Audio Commands, Mixing, and Spatial Sound](/openwiki/integrations/audio-and-spatial-sound.md) | Runtime guide for the ECS-to-mixer handoff; listen on a machine with a usable output device. |
| Demo controls, camera/player behavior, gameplay systems, initial entities, or Tab-triggered replacement | [Game Bootstrap, Input Controllers, and Scene Switching](/openwiki/workflows/game-bootstrap-and-scene-switching.md) | ECS/scene guide and an interactive demo run. |
| Cargo features or profiles, native dependencies, GitHub Actions, or what automation actually proves | [Build, Platform Dependencies, and Automation](/openwiki/operations/build-and-ci.md) | Validation guide; do not infer an interactive runtime test from CI's build/test gate. |
| A test plan, floating-point boundary, steady-state heap behavior, benchmark, or settings persistence | [Testing, Numerical Regression, and Allocation Checks](/openwiki/testing/validation-and-performance.md) | The domain page that defines the behavior being changed. |

## Runtime facts that decide safe placement

- **Game code extends ECS schedules; it does not own a second runtime.** `Engine` owns SDL input/windowing, the OpenGL renderer, the audio mixer, and the main loop. `Game` populates `Engine::scene` and registers game systems. Put once-per-render-frame actions such as edge-triggered controls in `game_frame_schedule`; put fixed-cadence state in `game_simulation_schedule`.
- **Ordering is observable behavior.** Per frame, the engine snapshots SDL input, runs its chained frame systems and the game-frame schedule, renders, executes zero to six fixed physics/game-simulation steps from an accumulator, hands audio commands to the mixer, performs cleanup and tracker clearing, presents, then commits a pending scene change. Rendering precedes fixed simulation, so later mutations are not automatically visible in the already-built queue.
- **A `Scene` is a complete ECS contract.** `Scene::new` installs scene-local resources for input, timing, rendering, physics, audio, active camera, and deferred switching while cloning the retained shared asset-service wrappers. Do not remove resources that engine schedules require, and fully configure a replacement scene before requesting it.
- **Scene replacement is deferred.** `SceneChangerResource` holds one pending scene; the engine consumes it only after the current frame and recreates its engine schedules for the new world. A later request replaces an earlier uncommitted one, so persistent data must not depend on old-world entity IDs.

The runtime guide explains the exact schedule/system order and failure modes. The scene and game-workflow guides show how to add systems/resources without violating those boundaries.

## Validation exit strategy

Use the narrowest check that exercises the changed contract, then broaden it when the change crosses crates or subsystems:

```bash
# Focused library and integration tests.
cargo test -p engine

# Heap-regression targets for repeated physics/data-structure paths.
cargo test -p engine --test dynamic_aabb_tree_heap_test
cargo test -p engine --test epa_heap_test

# Deliberate throughput measurement; it is not part of cargo test.
cargo bench -p engine --bench my_benchmark

# Application-package tests, including the current settings tests.
cargo test -p Game
```

The checked-in Rust workflow installs Linux SDL/window-system build dependencies and runs `cargo build --verbose` plus `cargo test --verbose` for pushes and pull requests to `master`. It does not launch `Game`, run benchmarks, or prove window, GPU, audio-device, controller, or full asset behavior. Finish changes in those areas with `cargo run -p Game` from the repository root and exercise the affected interaction; finish CPU-side shared changes with `cargo test --workspace`.

### Before opening a change

1. Identify the lifecycle phase and state owner before choosing a system or API.
2. Read the domain page in the table and preserve its stated ordering, numerical, handle, or callback contract.
3. Add or run a focused regression for the observable behavior, including boundary cases for physics and warm-up-aware checks for recurring allocations.
4. Run the workspace gate when interfaces or manifests cross `engine` and `Game`; add a real-device smoke test whenever the route reaches SDL, OpenGL, CPAL, or imported runtime assets.
