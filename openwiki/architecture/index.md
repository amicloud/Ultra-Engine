# Files

- [Engine Runtime and Crate Boundary](engine-runtime.md) - The `engine` library owns the platform runtime, rendering, audio mixer, shared asset services, and ordered ECS lifecycle, while the `Game` binary configures scenes and gameplay systems. This page defines the frame and fixed-step ordering that extensions must preserve.
