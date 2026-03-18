# Rust/Bevy Project Documentation

This document outlines the current architecture and implementation of the `mc2` project, a voxel terrain generator built with Rust and Bevy.

## Overview
The project is a 3D application that generates infinite procedural terrain using Perlin noise. It features a flying camera for exploration and a secondary window that displays a 2D map of the world based on the seed.

## Architecture
The project uses the **Bevy Game Engine**, which is built on the **Entity Component System (ECS)** pattern.

### Core Concepts (ECS)
*   **Entities:** Unique IDs that represent objects in the world (e.g., a camera, a terrain chunk, a block).
*   **Components:** Data structs attached to entities (e.g., `Transform`, `FlyCamera`, `Chunk`).
*   **Systems:** Functions that run every frame (or at specific times) to query and update entities with specific components.
*   **Resources:** Global singleton data (e.g., `WorldSeed`, `Time`, `Assets<Mesh>`).

## Modules

### 1. `main.rs` (Entry Point)
*   **Responsibilities:**
    *   Initializes the Bevy `App`.
    *   Generates a random `WorldSeed`.
    *   Configures the primary window (locks cursor, sets title).
    *   Registers plugins: `FlyCameraPlugin`, `TerrainPlugin`, `UiPlugin`.
    *   Spawns a secondary window for the Map via `setup_windows`.

### 2. `camera.rs` (Camera System)
*   **Components:**
    *   `FlyCamera`: Stores `yaw`, `pitch`, `speed`, `sensitivity`, and `is_locked` state.
*   **Systems:**
    *   `setup_camera`: Spawns a `Camera3d` bundle with the `FlyCamera` component.
    *   `move_camera`: Handles WASD + Space/Shift movement. Updates `Transform` translation.
    *   `rotate_camera`: Handles mouse motion. Updates `Transform` rotation using Quaternions calculated from yaw/pitch.
    *   `toggle_cursor`: Toggles cursor visibility and lock state with `Escape`.

### 3. `terrain.rs` (Terrain Generation)
*   **Libraries:** Uses `noise-rs` for Perlin noise generation.
*   **Resources:**
    *   `TerrainMaterials`: Stores handles to standard materials (Grass, Stone, Water) to avoid recreating them.
    *   `ActiveChunks`: A HashMap tracking which chunks are currently spawned.
*   **Systems:**
    *   `manage_chunks`:
        1.  Calculates the camera's current chunk position (chunk size = 16).
        2.  Identifies chunks within `render_distance` (6 chunks).
        3.  Despawns chunks that are out of range.
        4.  Spawns new chunks that are in range but missing.
*   **Logic:**
    *   `get_noise_height`: Combines multiple noise layers (continental, mountain, detail) to generate a heightmap value.
    *   `spawn_chunk`: Iterates x/z (0..16). Calculates height. Spawns `Mesh3d` entities for:
        *   Surface block (Grass/Stone/Water).
        *   3 layers of "dirt" below surface.
        *   Water blocks at sea level if needed.

### 4. `ui.rs` (Map & UI)
*   **Systems:**
    *   `setup_ui`:
        *   Finds the secondary `MapWindow`.
        *   Spawns a `Camera2d` targeting that window.
        *   Generates a 2D texture on the CPU by sampling the same `get_noise_height` function used by the terrain.
        *   Creates a Bevy UI tree (`Node`, `ImageNode`, `Text`) to display the seed and the generated map texture.

## How to Continue Development in Rust
1.  **Optimize Rendering:** Currently, `spawn_chunk` spawns individual entities for every block. This is extremely inefficient for large render distances.
    *   *Solution:* Use "Chunk Meshing" to generate a single mesh per chunk (combining faces) and spawn one entity per chunk.
2.  **Add Physics:** Add a physics engine (e.g., `bevy_xpbd` or `rapier`) to handle collisions so the camera/player doesn't fly through the ground.
3.  **Refactor UI:** Bevy's UI is robust but verbose. Consider using `bevy_egui` for debug tools.

---
**Note:** This project is being ported to C++ with Raylib and Flecs.
