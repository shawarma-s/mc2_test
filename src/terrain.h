#pragma once
#include "raylib.h"
#include "flecs.h"

void init_terrain_systems(flecs::world& world, int seed);
void draw_terrain(flecs::world& world);

struct Chunk {
    int x, z;
    Model model; // Raylib Model (contains the mesh)
};

float get_noise_height(int x, int z);
