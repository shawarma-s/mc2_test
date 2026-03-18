#define FNL_IMPL
#include "FastNoiseLite.h"
#include "terrain.h"
#include "camera.h"
#include <vector>
#include <cmath>
#include <iostream>

// Global noise state (simplified for this example, could be a component/resource)
fnl_state noise_state;
int world_seed = 12345;

void init_noise(int seed) {
    world_seed = seed;
    noise_state = fnlCreateState();
    noise_state.seed = seed;
    noise_state.noise_type = FNL_NOISE_PERLIN;
    noise_state.frequency = 1.0f; // Ensure our 1/250.0f scales are absolute
}

float get_noise_height(int x, int z) {
    float continental = fnlGetNoise2D(&noise_state, (float)x * (1.0f/250.0f), (float)z * (1.0f/250.0f));
    float mountain_mask = fnlGetNoise2D(&noise_state, (float)x * (1.5f/250.0f), (float)z * (1.5f/250.0f));
    float mountain = fnlGetNoise2D(&noise_state, (float)x * (1.0f/120.0f), (float)z * (1.0f/120.0f));
    float detail = fnlGetNoise2D(&noise_state, (float)x * (1.0f/30.0f), (float)z * (1.0f/30.0f));

    // Map noise from [-1, 1] to [0, 1]
    continental = (continental + 1.0f) * 0.5f;
    mountain_mask = (mountain_mask + 1.0f) * 0.5f;

    // Shift continental down to create more ocean
    float c = (continental - 0.45f); // Changed from 0.55f to 0.45f to increase land mass slightly
    if (c < -0.6f) c = -0.6f;

    float height = c * 50.0f;

    if (mountain_mask > 0.5f) {
        float steepness = (mountain_mask - 0.5f) * 2.0f;
        float peak = std::pow(std::abs(mountain), 2.2f) * 80.0f;
        height += peak * steepness;
    }

    height += detail * 4.0f;
    return height;
}

// Helper to add a cube face to mesh buffers
void AddFace(std::vector<float>& vertices, std::vector<float>& texcoords, std::vector<float>& normals, 
             float x, float y, float z, float r, float g, float b, int face) {
    // Simple colored cube faces. Raylib expects triangles.
    // We will use Vertex colors? Raylib's default material supports textures. 
    // For colored vertices, we usually need a custom shader or use ImageDraw for a texture atlas.
    // To keep it simple, we won't do per-vertex colors in this basic port, 
    // we will just rely on lighting or basic textures.
    // HOWEVER, for a voxel engine, Vertex Colors are best. 
    // Raylib Mesh has 'colors' buffer.
    
    // ... Implementation omitted for brevity in this step-by-step. 
    // We will actually simplify: We will use Raylib's `DrawCube` for the *prototype* 
    // but limit render distance strongly. 
    // Generating a proper mesh manually in C++ without helper libs is verbose (100+ lines).
    // I will write a simplified `DrawChunk` system that uses `DrawCube` but is optimized by checking visibility.
}

// ----------------------------------------------------------------------------------

void manage_chunks(flecs::iter& it, GameCamera* gc) {
    auto world = it.world();
    
    Vector3 camPos = gc->camera.position;
    int chunk_size = 16;
    int render_dist = 4; // Low render distance for DrawCube performance

    int cam_cx = (int)floor(camPos.x / chunk_size);
    int cam_cz = (int)floor(camPos.z / chunk_size);

    // 1. Identify needed chunks
    std::vector<std::pair<int, int>> needed;
    for (int x = -render_dist; x <= render_dist; x++) {
        for (int z = -render_dist; z <= render_dist; z++) {
            needed.push_back({cam_cx + x, cam_cz + z});
        }
    }

    // 2. Remove far chunks
    // In Flecs, we'd query all chunks and delete if far. 
    // This is expensive to do every frame. We'll do it every 60 frames.
    static int frame_count = 0;
    if (frame_count++ % 30 == 0) {
        world.filter<Chunk>()
            .each([&](flecs::entity e, Chunk& c) {
                if (abs(c.x - cam_cx) > render_dist || abs(c.z - cam_cz) > render_dist) {
                    // Unload model if we had one
                    // UnloadModel(c.model); 
                    e.destruct();
                }
            });
    }

    // 3. Spawn missing chunks
    // This requires checking if a chunk exists.
    // A simple way is to name entities "Chunk_X_Z".
    for (auto& pos : needed) {
        char name[64];
        snprintf(name, sizeof(name), "Chunk_%d_%d", pos.first, pos.second);
        
        auto e = world.lookup(name);
        if (!e) {
            world.entity(name)
                .set<Chunk>({ pos.first, pos.second, {0} }); // Model {0} as placeholder
        }
    }
}

void init_terrain_systems(flecs::world& world, int seed) {
    init_noise(seed);

    world.system<GameCamera>("ManageChunks")
        .kind(flecs::OnUpdate)
        .iter(manage_chunks);
}

void draw_terrain(flecs::world& world) {
    int chunk_size = 16;

    world.filter<Chunk>()
        .each([&](Chunk& c) {
            for (int x = 0; x < chunk_size; x++) {
                for (int z = 0; z < chunk_size; z++) {
                    int world_x = c.x * chunk_size + x;
                    int world_z = c.z * chunk_size + z;

                    float h_val = get_noise_height(world_x, world_z);
                    int height = (int)round(h_val);

                    // Surface Color
                    Color color = GREEN; 
                    if (height < 0) color = BLUE;
                    else if (height < 2) color = BEIGE; // Sand
                    else if (height > 25) color = LIGHTGRAY; // Snow/Stone

                    // Draw Surface
                    DrawCube((Vector3){(float)world_x, (float)height, (float)world_z}, 1.0f, 1.0f, 1.0f, color);
                    
                    // Draw 2 layers of "dirt/stone" below for depth
                    for (int y = height - 1; y >= height - 2; y--) {
                        DrawCube((Vector3){(float)world_x, (float)y, (float)world_z}, 1.0f, 1.0f, 1.0f, DARKBROWN);
                    }

                    // Water Level
                    if (height < 0) {
                         DrawCube((Vector3){(float)world_x, 0.0f, (float)world_z}, 1.0f, 1.0f, 1.0f, Fade(BLUE, 0.6f));
                    }
                }
            }
        });
}
