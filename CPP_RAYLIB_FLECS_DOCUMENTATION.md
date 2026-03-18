# C++ Port Documentation (Raylib + Flecs)

This document explains the C++ port of the `mc2` project, using **Raylib** for rendering and **Flecs** for the Entity Component System (ECS).

## Prerequisites
*   **CMake** (3.14 or higher)
*   **C++ Compiler** (GCC, Clang, or MSVC) supporting C++17.
*   **Git** (to fetch dependencies).

## Build Instructions
The project uses `FetchContent` to automatically download Raylib and Flecs. No manual installation of these libraries is required.

1.  Navigate to the `cpp_port` directory:
    ```bash
    cd cpp_port
    ```
2.  Create a build directory:
    ```bash
    mkdir build
    cd build
    ```
3.  Configure with CMake:
    ```bash
    cmake ..
    ```
4.  Build the project:
    ```bash
    cmake --build .
    ```
5.  Run the executable:
    *   **Mac/Linux:** `./mc2_cpp`
    *   **Windows:** `Debug\mc2_cpp.exe`

## Architecture Comparison

| Feature | Rust (Bevy) | C++ (Raylib + Flecs) |
| :--- | :--- | :--- |
| **Entry Point** | `main.rs` -> `App::new().run()` | `main.cpp` -> `while (!WindowShouldClose())` |
| **ECS World** | `App` builder | `flecs::world` object |
| **Components** | `#[derive(Component)] struct` | Plain `struct` |
| **Systems** | Functions registered in `App` | `world.system<...>().iter(...)` lambdas |
| **Resources** | `Res<T>` / `ResMut<T>` | `world.set<T>(...)` / `world.get<T>()` |
| **Rendering** | Bevy Renderer (WGPU) | Raylib `BeginDrawing()` / `DrawCube()` |

## Modules

### 1. `src/main.cpp`
*   Initializes the Raylib window.
*   Creates the `flecs::world`.
*   Calls initialization functions for Camera, Terrain, and UI.
*   Runs the main loop, calling `world.progress()` to update logic and then drawing the results.

### 2. `src/camera.cpp`
*   **Component:** `FlyCamera` (logic state), `GameCamera` (Raylib `Camera3D` wrapper).
*   **Systems:**
    *   `MoveCamera`: Reads WASD keys to update position.
    *   `RotateCamera`: Reads mouse delta to update yaw/pitch.
*   **Helper:** `GetCamera(world)` returns the active `Camera3D` for rendering.

### 3. `src/terrain.cpp`
*   **Library:** Uses `FastNoiseLite.h` (header-only) for Perlin noise.
*   **Logic:**
    *   `init_noise`: Sets up the noise seed.
    *   `get_noise_height`: Identical math to the Rust version.
    *   `manage_chunks`: Calculates visible chunks based on camera position. Adds `Chunk` components for new areas and removes old ones.
    *   `draw_terrain`: Iterates over `Chunk` entities and uses `DrawCube` to render blocks. 
    *   *Note:* `DrawCube` is used for simplicity. For a production game, you would generate a `Mesh` object per chunk to reduce draw calls.

### 4. `src/ui.cpp`
*   **Logic:**
    *   `init_ui_systems`: Generates a `Texture2D` map on startup by sampling noise.
    *   `draw_ui`: Draws the texture overlay and debug text (FPS, coords).

## Next Steps for C++ Development
1.  **Mesh Generation:** Replace the `DrawCube` loop in `draw_terrain` with `GenMeshCustom` to create a single mesh per chunk. This will drastically improve performance.
2.  **Multithreading:** Flecs supports multithreaded systems. You can move terrain generation to a background thread to prevent frame drops when moving.
3.  **Shaders:** Use custom shaders with Raylib (`LoadShader`) to add fog, shadows, and better water effects.
