#pragma once
#include "raylib.h"
#include "flecs.h"

void init_ui_systems(flecs::world& world);
void draw_ui(flecs::world& world);
