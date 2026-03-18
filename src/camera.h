#pragma once
#include "raylib.h"
#include "flecs.h"

struct FlyCamera {
    float sensitivity = 0.1f;
    float speed = 30.0f;
    float yaw = -90.0f * (PI / 180.0f); // Radians
    float pitch = 0.0f;
    bool is_locked = true;
};

// Wrapper component to hold the Raylib Camera3D
struct GameCamera {
    Camera3D camera;
};

void init_camera_systems(flecs::world& world);
Camera3D GetCamera(flecs::world& world);
