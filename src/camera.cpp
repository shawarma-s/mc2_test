#include "camera.h"
#include "raymath.h"
#include <cmath>

void init_camera_systems(flecs::world& world) {
    // Spawn the camera entity
    Camera3D cam = { 0 };
    cam.position = (Vector3){ 0.0f, 60.0f, 60.0f };
    cam.target = (Vector3){ 0.0f, 0.0f, 0.0f };
    cam.up = (Vector3){ 0.0f, 1.0f, 0.0f };
    cam.fovy = 45.0f;
    cam.projection = CAMERA_PERSPECTIVE;

    world.entity("MainCamera")
        .set<GameCamera>({ cam })
        .set<FlyCamera>({});

    DisableCursor(); // Lock cursor by default

    // System: Toggle Cursor
    world.system<FlyCamera>("ToggleCursor")
        .kind(flecs::OnUpdate)
        .iter([](flecs::iter& it, FlyCamera* fc) {
            if (IsKeyPressed(KEY_ESCAPE)) {
                for (auto i : it) {
                    fc[i].is_locked = !fc[i].is_locked;
                    if (fc[i].is_locked) DisableCursor();
                    else EnableCursor();
                }
            }
        });

    // System: Rotate Camera
    world.system<GameCamera, FlyCamera>("RotateCamera")
        .kind(flecs::OnUpdate)
        .iter([](flecs::iter& it, GameCamera* gc, FlyCamera* fc) {
            Vector2 delta = GetMouseDelta();
            
            for (auto i : it) {
                if (!fc[i].is_locked) continue;

                fc[i].yaw -= delta.x * fc[i].sensitivity * (PI / 180.0f);
                fc[i].pitch -= delta.y * fc[i].sensitivity * (PI / 180.0f);

                // Clamp pitch
                if (fc[i].pitch > 89.0f * (PI / 180.0f)) fc[i].pitch = 89.0f * (PI / 180.0f);
                if (fc[i].pitch < -89.0f * (PI / 180.0f)) fc[i].pitch = -89.0f * (PI / 180.0f);

                // Update Camera Target based on Yaw/Pitch
                Vector3 direction;
                direction.x = cosf(fc[i].pitch) * sinf(fc[i].yaw);
                direction.y = sinf(fc[i].pitch);
                direction.z = cosf(fc[i].pitch) * cosf(fc[i].yaw);

                // Vector3Normalize is in raymath.h
                direction = Vector3Normalize(direction);
                
                gc[i].camera.target = Vector3Add(gc[i].camera.position, direction);
            }
        });

    // System: Move Camera
    world.system<GameCamera, const FlyCamera>("MoveCamera")
        .kind(flecs::OnUpdate)
        .iter([](flecs::iter& it, GameCamera* gc, const FlyCamera* fc) {
            float dt = it.delta_time();
            
            for (auto i : it) {
                if (!fc[i].is_locked) continue;

                Vector3 forward = Vector3Subtract(gc[i].camera.target, gc[i].camera.position);
                forward.y = 0; // Keep movement on XZ plane
                forward = Vector3Normalize(forward);

                Vector3 right = Vector3CrossProduct(forward, gc[i].camera.up);
                right = Vector3Normalize(right);

                Vector3 up = { 0.0f, 1.0f, 0.0f };
                
                Vector3 velocity = { 0.0f, 0.0f, 0.0f };

                if (IsKeyDown(KEY_W)) velocity = Vector3Add(velocity, forward);
                if (IsKeyDown(KEY_S)) velocity = Vector3Subtract(velocity, forward);
                if (IsKeyDown(KEY_D)) velocity = Vector3Add(velocity, right);
                if (IsKeyDown(KEY_A)) velocity = Vector3Subtract(velocity, right);
                if (IsKeyDown(KEY_SPACE)) velocity = Vector3Add(velocity, up);
                if (IsKeyDown(KEY_LEFT_SHIFT)) velocity = Vector3Subtract(velocity, up);

                if (Vector3Length(velocity) > 0) {
                    velocity = Vector3Normalize(velocity);
                    Vector3 move = Vector3Scale(velocity, fc[i].speed * dt);
                    
                    gc[i].camera.position = Vector3Add(gc[i].camera.position, move);
                    gc[i].camera.target = Vector3Add(gc[i].camera.target, move);
                }
            }
        });
}

Camera3D GetCamera(flecs::world& world) {
    Camera3D cam = { 0 };
    // Just return the first camera we find
    world.filter<GameCamera>()
        .each([&](GameCamera& gc) {
            cam = gc.camera;
        });
    return cam;
}
