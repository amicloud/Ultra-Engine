# Files

- [Asset Import and OpenGL Rendering](assets-and-rendering.md) - How models, textures, shaders, and WAV audio enter the engine's shared handle stores, become ECS render instances, and are culled, batched, uploaded, and drawn through OpenGL.
- [Audio Commands, Mixing, and Spatial Sound](audio-and-spatial-sound.md) - Explains how WAV assets and ECS audio components become callback-owned voices through the frame command queue and CPAL mixer. Covers collision sounds, tracks, listener/source lifecycle, spatial processing, and the important failure and timing limits.
