# Files

- [ECS Scene, Components, and Shared Resource Ownership](ecs-scenes-and-resources.md) - Explains how a Scene creates an ECS world with fresh gameplay state while sharing Arc/RwLock-backed asset stores. Covers component contracts, transforms and coordinates, typed SlotMap handles, rendering relationships, and safe scene replacement.
- [Physics, Collision Detection, and Events](physics-and-collision.md) - Defines the engine's collider and rigid-body contracts, change-driven collision pipeline, contact manifold and PGS response rules, sleep behavior, and opt-in collision events. Use it to preserve the ordering, normal orientation, and numerical invariants behind physics changes.
