#include "raylib.h"
#include "flecs.h"
#include "camera.h"
#include "terrain.h"
#include "ui.h"
#include <time.h>

int main() {
    const int screenWidth = 1280;
    const int screenHeight = 720;

    InitWindow(screenWidth, screenHeight, "MC2 Game (C++)");
    SetTargetFPS(60);

    // Initialize Flecs world
    flecs::world world;

    // Set a random seed
    int seed = (int)time(NULL);

    // Set up systems
    init_camera_systems(world);
    init_terrain_systems(world, seed);
    init_ui_systems(world);

    // Main game loop
    while (!WindowShouldClose()) {
        // Run Flecs systems (Logic)
        world.progress(GetFrameTime());

        // Drawing is handled in the systems or here if we want a clear separation.
        // For Raylib + Flecs, it's often cleaner to have a "Draw" phase in systems
        // or just do the BeginDrawing here and let systems draw.
        // However, Flecs systems run in pipeline order. 
        // We will make the systems handle drawing within a "OnStore" or custom phase, 
        // OR we can just do a simple approach where systems update data, 
        // and we have a specific "Render" system.
        
        // For simplicity, we'll let systems run. We need to ensure
        // drawing happens between BeginDrawing and EndDrawing.
        
        // Actually, the cleanest way for a simple app:
        BeginDrawing();
        ClearBackground(SKYBLUE);
        
            BeginMode3D(GetCamera(world)); // Helper to get the camera from ECS
            
                // Draw Terrain (We'll make a system that runs on a custom phase or just call it here if it's simple)
                // Ideally, we register a system `DrawTerrain` that runs in `flecs::OnStore`.
                // But to keep it simple and readable for a beginner:
                draw_terrain(world);
                
            EndMode3D();
            
            draw_ui(world);
            
        EndDrawing();
    }

    CloseWindow();

    return 0;
}
