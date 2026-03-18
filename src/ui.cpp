#include "ui.h"
#include "terrain.h"
#include <vector>
#include <stdio.h>

struct MapUI {
    Texture2D texture;
    bool is_visible;
};

void init_ui_systems(flecs::world& world) {
    // Generate Map Texture
    int size = 256;
    Image mapImage = GenImageColor(size, size, BLACK);
    
    // We need to access pixel data. Raylib provides ImageDrawPixel but that's slow for a whole image.
    // Better to modify data buffer.
    // Raylib Image data is usually RGBA (4 bytes).
    
    // Note: In C++, Color* pixels = LoadImageColors(mapImage); is safer but let's just use ImageDrawPixel for simplicity 
    // since it runs once on startup.
    
    for (int y = 0; y < size; y++) {
        for (int x = 0; x < size; x++) {
            int wx = x - (size / 2);
            int wz = y - (size / 2);
            
            float h = get_noise_height(wx, wz);
            
            Color c;
            if (h < 0) c = BLUE;
            else if (h > 20) c = DARKGRAY;
            else c = GREEN;
            
            ImageDrawPixel(&mapImage, x, y, c);
        }
    }
    
    Texture2D texture = LoadTextureFromImage(mapImage);
    UnloadImage(mapImage);
    
    world.set<MapUI>({ texture, true });
}

void draw_ui(flecs::world& world) {
    const MapUI* ui = world.get<MapUI>();
    if (!ui || !ui->is_visible) return;

    // Draw Map (Top Left)
    DrawTexture(ui->texture, 10, 10, WHITE);
    DrawRectangleLines(10, 10, ui->texture.width, ui->texture.height, WHITE);
    
    // Draw Text
    DrawText("MC2 C++ Port", 10, 270, 20, WHITE);
    DrawText("WASD + Space/Shift to Move", 10, 300, 10, LIGHTGRAY);
    DrawText("Mouse to Look (ESC to unlock)", 10, 315, 10, LIGHTGRAY);
    
    DrawFPS(1200, 10);
}
