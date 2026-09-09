---
type: game workflow
title: Game Bootstrap, Input Controllers, and Scene Switching
description: How the Game binary builds its interactive physics and audio demo, attaches input-driven camera and gameplay systems, and defers Tab-triggered scene replacement until the engine frame ends. Includes the current demo limitations and the separate, currently unused settings facility.
tags: [game, bootstrap, input, camera, scene-switching, ecs, demo]
verified:
  - by: openwiki/0.5.0
    at: 2026-09-06T19:22:21.487Z
sources:
  - id: openwiki-source-fc3a0fdb70caaf5442752646
    resource: repo://engine/src/assets/model_loader.rs
  - id: openwiki-source-14c364ac054e4872698b2a5b
    resource: repo://engine/src/assets/sound.rs
  - id: openwiki-source-2080554b4271dd19fe76bebf
    resource: repo://engine/src/input.rs
  - id: openwiki-source-5a99994eeb2e588b0d7e125a
    resource: repo://engine/src/lib.rs
  - id: openwiki-source-a3ded95da5e0b918918b81b9
    resource: repo://engine/src/scene/scene_changer_resource.rs
  - id: openwiki-source-b9c584eef33a510c58625d64
    resource: repo://engine/src/scene/scene_services.rs
  - id: openwiki-source-2c5adbcfbece31368fdc54dd
    resource: repo://engine/src/scene/scene.rs
  - id: openwiki-source-eb02440b972dcc3270ddbf9a
    resource: repo://engine/src/time_resource.rs
  - id: openwiki-source-b7157e7ea3ba5ff01b6279b6
    resource: repo://game/src/camera_controller.rs
  - id: openwiki-source-5fedb0cc8a6319b54b46842b
    resource: repo://game/src/game_controller.rs
  - id: openwiki-source-b76610d665b3d6fb48b86d70
    resource: repo://game/src/main.rs
  - id: openwiki-source-cee4320bee3b1b0f73923487
    resource: repo://game/src/settings.rs
generated: { by: "openwiki/0.5.0", at: "2026-09-06T19:22:21.487Z" }
---

# Game Bootstrap, Input Controllers, and Scene Switching

`game/src/main.rs` is the application composition root for the `Game` binary. It creates `Engine`, directly populates `engine.scene.world`, registers game-owned systems on the scene schedules, loads the assets that those entities reference, and finally enters `Engine::run()`. The engine owns the SDL/OpenGL/audio runtime and its engine schedules; the binary owns the sample scene and its controls. See [Engine Runtime and Crate Boundary](/openwiki/architecture/engine-runtime.md) for the complete engine-frame ordering and [ECS Scene, Components, and Shared Resource Ownership](/openwiki/concepts/ecs-scenes-and-resources.md) for the scene/resource contract.

> **Demo status.** This binary is a development/demo composition, not a scene-loading framework. The random objects, gravity controls, direct audio keys, and Tab replacement scene are hard-coded examples. Treat their key bindings and entity makeup as sample behavior unless a product-level input or scene API is introduced.

## Bootstrap: from process entry to a playable demo

`main` starts with `Engine::new()`. That creates the first `Scene` with the engine's shared asset services and the required scene-local resources; game setup then mutates that initially empty scene. The setup ends with `engine.run()`, which is the blocking runtime loop.

### Cameras and active selection

The binary spawns two camera entities with a `CameraComponent` configured for 75° vertical FOV, a hard-coded `1024.0 / 769.0` aspect ratio, near plane `0.1`, and far plane `10000.0`:

- **Flying camera (initial active camera):** starts at `(0, 0, 2)`, has `FlyingCameraComponent` (yaw `-135`, sensitivity `0.1`, speed `100`), `VelocityComponent`, and `SingleAudioListenerComponent`. The program stores its entity in `ActiveCamera`; this one entity therefore supplies both the rendered view and the spatial-audio listener.
- **Orbit camera:** starts with `OrbitCameraComponent` targeting the origin at distance `100`, yaw `0`, pitch `-30`, sensitivity `0.2`. It is inactive initially, has no listener marker, and can follow the player once one exists.

A `CameraComponent` supplies projection parameters, while its required transform supplies the view pose. The controller systems intentionally return without doing anything when `ActiveCamera` is `None` or the selected entity does not carry the controller type that a system queries. That makes controller composition tolerant of switching between these two camera types rather than requiring every camera to carry every controller.

### Assets and entities

Before running, the demo synchronously imports a cube glTF, low-poly sphere OBJ, platform OBJ, and four WAV files. The cube render-body AABB is used to size a player cuboid collider; model loads and the AABB lookup use `unwrap`/`expect`, as do the sound loads, so missing or malformed startup assets fail fast rather than producing an empty fallback scene. The initial world includes:

- a dynamic cube **player**, with render body, cuboid collider on `CollisionLayer::Player`, physics parameters, velocity, `PlayerComponent`, and a `PhysicsEventListenerComponent`;
- a static platform with a mesh collider generated from its render body; and
- `10` randomized dynamic spheres in debug builds or `100` in non-debug builds. They have sphere colliders, sleeping state, and `SimpleOnHitAudioComponent` wired to `pop.wav`.

The loader returns shared typed handles, not entity-local geometry. Thus the current scene can render and collide with the imported bodies; the services held by the engine also survive a later `Scene` replacement. For importer behavior and the render-body/collider relationship, see [Asset Import and OpenGL Rendering](/openwiki/integrations/assets-and-rendering.md) and [Physics, Collision Detection, and Events](/openwiki/concepts/physics-and-collision.md).

The commented-out spatial-audio entity and bulk cube loop are not active. In particular, `spatial_audio_orbit_demo` is registered but has no matching `SpatialAudioDemoComponent` entity in the default uncommented world, so it currently has no observable effect.

## Which schedule runs which game behavior

The game adds two separate schedules to the current scene:

| Schedule | Registered behavior | Practical cadence and ordering |
| --- | --- | --- |
| `game_simulation_schedule` | `initialize_flying_camera_rotation`, orbit input/target update, flying look/movement, player movement, `do_gameplay`, `spatial_audio_orbit_demo` | The tuple is `.chain()`ed and runs after every engine physics step. A rendered frame can run zero through six such steps, so logic here must tolerate repeated execution. |
| `game_frame_schedule` | orbit input, camera switch, `sound_control`, `scene_switcher` | Runs once each rendered frame after the engine's own frame chain and before rendering. It is appropriate for edge-triggered actions such as `V`, audio toggles, and `Tab`. |

`apply_orbit_camera_input` appears in **both** schedules. While the flying camera is active, its orbit-camera query fails and it is inert. After selecting the orbit camera, it can process mouse/scroll input once at frame cadence and again for every fixed step; this is current demo behavior, not a general camera-input policy. It also means the later chained simulation invocation can overwrite the transform produced by the frame invocation.

The engine runs its physics systems before this game simulation schedule. Therefore player velocity changes made by the controllers are normally integrated by a following engine physics step, rather than the physics step that preceded the controller invocation. Rendering itself occurs before fixed simulation; visual effects of fixed-step work are normally represented in a later frame. See [Engine Runtime and Crate Boundary](/openwiki/architecture/engine-runtime.md) for these phase boundaries.

## Input model and control map

At the beginning of each engine frame, SDL events update `InputStateResource`. The resource copies the current keyboard/button sets to previous sets before polling, resets mouse and wheel deltas, and then applies key/button down/up events. `key_held` tests the current set, whereas `key_pressed` is a rising edge (`current && !previous`) and `key_released` is a falling edge. Mouse motion and wheel fields hold the last event's relative delta for that frame; they are not accumulated across events.

Because a rising edge remains true for every fixed step that occurs within the rendered frame, operations that must occur only once per visual frame belong in `game_frame_schedule`. The sample puts `scene_switcher`, camera selection, and direct sound control there. In contrast, systems that use `key_held` appropriately belong in its chained simulation schedule.

### Camera and player controls

| Input | Default demo effect | Notes |
| --- | --- | --- |
| Left mouse drag | Flying camera: mouse-look. Orbit camera: yaw/pitch around its target. | Both clamp pitch to `[-89.9, 89.9]`. Only the active entity's matching controller responds. |
| Middle mouse drag | Orbit camera pan. | No effect for the flying controller. |
| Right mouse drag or scroll wheel | Orbit zoom. | Orbit distance is clamped to `[10, 300]`. |
| Arrow keys | Flying camera look. | Each held arrow applies a fixed controller delta on each simulation run. |
| `W` / `A` / `S` / `D`, `LShift`, `LCtrl` | Flying camera sets its translational velocity along its rotated forward/right/up vectors. | If no direction is held, existing translational velocity is only damped; it is not explicitly zeroed. `P` prints flying position/velocity when edge-detected. |
| `W` / `A` / `S` / `D`, `Space` | Every non-camera `PlayerComponent` receives an addition to `VelocityComponent.angular`, derived from the active camera orientation projected into the world plane. | Despite the function name `apply_player_movement_impulses`, the current implementation changes **angular**, not translational, velocity. It falls back to `WorldBasis` when no usable active camera exists. |
| `V` | Selects the first queried `CameraComponent` entity that is not the active entity. | This is not a stable two-camera toggle: it relies on query iteration and does not store an ordered camera pair. |

The orbit controller uses the gravity direction and `WorldBasis` to calculate its pose, then `update_orbit_camera_target` makes every orbit camera target the single player transform when exactly one player exists. The first flying-camera rotation is synchronized from yaw/pitch only when `FlyingCameraComponent` is added. The world basis is Z-up and negative-Y forward; controllers should use the resource, as this demo does, rather than replicate axes.

### Gameplay and audio keys

`do_gameplay` rotates gravity about the world right/forward axes while `I`, `J`, `K`, or `L` is held, limits the deviation from default gravity to 30°, and otherwise springs gravity back to default. It uses `TimeResource::frame_delta_time()` even though it is registered in the fixed simulation schedule. Since the engine can invoke that schedule multiple times in one rendered frame, changing this behavior should deliberately choose fixed delta versus frame delta and test it under multi-step frames.

`sound_control` is a once-per-frame demo mapping:

| Key edge | `AudioControl` action |
| --- | --- |
| `N` | Play `sea_shanty_2.wav` as a track-2 one shot at volume `0.5`. |
| `P` / `O` | Pause / resume the entire mix. |
| `M` / `U` | Mute / unmute the entire mix. |

The sound lookup by name is unwrapped, but startup loaded that exact filename. These controls enqueue commands in scene-local `AudioControl`; the engine transfers them to the callback-owned mixer after fixed steps. They are not direct device calls. The `P` edge also reaches the flying-camera diagnostic when that controller is active. For the asynchronous audio boundary and spatial listener/source behavior, see [Audio Commands, Mixing, and Spatial Sound](/openwiki/integrations/audio-and-spatial-sound.md).

## Deferred Tab scene replacement

The Tab action is deliberately a request rather than an in-place mutation. `scene_switcher` is a `game_frame_schedule` system, so it observes `key_pressed(Keycode::Tab)` at most once per rendered frame. On that edge it gets the scene's `SceneServices`, constructs `Scene::new(&services)`, calls the demo-specific `make_test_scene`, then moves the completed scene into `SceneChangerResource::request_change`.

```mermaid
sequenceDiagram
    participant SDL as SDL Event Pump
    participant Loop as Engine Loop
    participant Input as InputStateResource
    participant Game as Game Frame Schedule
    participant Switcher as scene_switcher
    participant Services as SceneServices
    participant Pending as SceneChangerResource
    participant NewScene as Pending Scene
    participant Schedules as Engine Schedules

    SDL->>Loop: KeyDown Tab
    Loop->>Input: copy previous keys and apply events
    Loop->>Game: run game frame schedule
    Game->>Switcher: execute scene_switcher
    Switcher->>Input: key_pressed Tab
    Input-->>Switcher: current Tab and previous lacks Tab
    Switcher->>Services: borrow shared services
    Switcher->>NewScene: Scene::new and make_test_scene
    Switcher->>Pending: request_change new scene
    Note over Pending: one pending scene slot
    Loop->>Loop: render fixed steps audio cleanup and swap
    Loop->>Pending: take_pending at end of frame
    Pending-->>Loop: pending scene
    Loop->>Loop: replace Engine scene
    Loop->>Schedules: reset and rebuild frame physics cleanup schedules
```

*Tab edge detection creates a fully configured pending scene during game-frame work; the engine commits it only after the current frame has presented and rebuilds its engine schedules for the new world.*

The pending slot holds only one `Scene`. If another request occurs before commit, `request_change` logs a warning and replaces the earlier request; `take_pending` consumes the selected scene. The old world keeps running through the rest of its current frame—rendering, any fixed steps, audio transfer, cleanup, tracker clearing, pacing, and buffer swap. Only then does `Engine` assign the pending scene and reset/re-add its frame, physics, and cleanup schedules because Bevy schedules bind their systems to the world in which they have run. The new scene retains its already-populated game schedules; those are not rebuilt by the engine.

### What the current replacement actually creates

`make_test_scene` is intentionally narrower than the initial bootstrap. It creates the same two cameras, selects the flying camera, and registers the same game schedules, but it does **not** load assets or spawn the player, platform, randomized objects, audio emitters, or collision scene. The shared stores still contain initial assets because `Scene::new` receives cloned service wrappers, but the replacement has no uncommented `RenderBodyComponent` entities to display and no player for the orbit target to follow. As written, Tab therefore demonstrates lifecycle replacement rather than rebuilding the initial playable world.

To turn this into a real scene system, make a reusable scene-builder that fully initializes each target scene: start with `Engine::new_scene()` when the caller has the engine, or `Scene::new(&services)` from a system; insert custom per-scene resources; spawn entities; set `ActiveCamera`; register both game schedules; and only then call `request_change`. Never retain old `Entity` IDs across this boundary. Keep reusable assets as shared handles, and use game-owned data for any persistent identity.

## Settings are implemented but not connected to startup

`game/src/settings.rs` defines serializable `Settings` sections for general, renderer, and network values. Its optional initialization path reads user settings from the platform configuration directory at `Ultra/settings/user_settings.toml`, falls back to `default_settings.toml`, and on first-run fallback writes hard-coded defaults to both locations. File, TOML serialization/deserialization, and unavailable-configuration-directory failures are represented by `SettingsError`; `load_user_settings` logs an initialization error and returns in-memory hard-coded defaults.

However, the current `main` only declares `mod settings;`; it never calls `Settings::load_user_settings`, loads a file, saves settings, or applies any settings value to `Engine` or the demo. Consequently, no settings file I/O or renderer/network configuration occurs during normal `Game` bootstrap today. Connecting it is an explicit integration task: decide when errors should be surfaced, load before constructing/configuring the dependent subsystem, and actually map each setting to supported behavior rather than assuming the structs configure the engine.

## Verification and safe change checklist

Run the game package's existing unit tests with:

```text
cargo test -p Game
```

The concrete game tests are in the private `settings` module. They use serial execution and temporary configuration directories to cover TOML round trips, user/default precedence, creation of missing files/directories, corrupt-user fallback, and I/O/serialization errors. They do **not** exercise controller input, bootstrap entity composition, SDL polling, deferred scene replacement, rendering, physics, or audio.

For changes in this workflow, add focused, headless ECS tests before relying only on an interactive run:

1. Build a `Scene` with the required resources, synthesize current/previous input sets, and assert controller transforms/velocities and the `V` selection policy. Include zero and multiple fixed-step behavior for held versus edge input.
2. Run `scene_switcher` against a world with `SceneServices`, then assert that a Tab rising edge produces one pending, fully configured scene; assert that replacement requests overwrite the earlier pending scene. Unit-test the engine end-of-frame handoff separately, including schedule reconstruction against the new world.
3. Test a scene-builder's entity and resource contract: active camera validity, expected render/physics/audio components, and continued resolution of a shared handle after replacement. Explicitly test that no old-world entity ID is reused.
4. Retain the settings tests when changing persistence. Because they mutate `HOME` and `XDG_CONFIG_HOME`, keep them serialized and restore the environment; add an application-level test once startup actually invokes settings.
5. Finally smoke-test `cargo run -p Game` on a host with a display, OpenGL 3.3, and an available default audio device. Exercise camera controls, hit sound, `N`/mix controls, `V`, and Tab; after Tab, expect the intentionally camera-only demo scene unless its builder has been expanded.
