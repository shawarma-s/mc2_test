use bevy::prelude::*;
use bevy::asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::camera::RenderTarget;
use noise::Perlin;
use crate::{WorldSeed, MapWindow, terrain};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_ui);
    }
}

#[derive(Component)]
struct UiInitialized;

fn setup_ui(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    seed: Res<WorldSeed>,
    window_query: Query<Entity, With<MapWindow>>,
    initialized_query: Query<Entity, With<UiInitialized>>,
) {
    if !initialized_query.is_empty() {
        return;
    }

    let window_entity = match window_query.single() {
        Ok(w) => w,
        Err(_) => return,
    };

    commands.spawn(UiInitialized);

    // Spawn 2D Camera targeting the Map Window in Bevy 0.18
    let camera_entity = commands.spawn((
        Camera2d::default(),
        RenderTarget::Window(bevy::window::WindowRef::Entity(window_entity)),
    )).id();

    // Generate Map Image
    let size = 256;
    let mut data = vec![0u8; (size * size * 4) as usize];
    let perlin = Perlin::new(seed.0);

    for y in 0..size {
        for x in 0..size {
            let wx = x as i32 - (size / 2) as i32;
            let wz = y as i32 - (size / 2) as i32;
            
            let height = terrain::get_noise_height(wx, wz, &perlin);
            
            let r; let g; let b;
            if height < 0.0 {
                r = 0; g = 100; b = 255;
            } else if height > 20.0 {
                r = 100; g = 100; b = 100;
            } else {
                r = 25; g = 180; b = 25;
            }

            let index = ((y * size + x) * 4) as usize;
            data[index] = r;
            data[index + 1] = g;
            data[index + 2] = b;
            data[index + 3] = 255;
        }
    }

    let image = Image::new(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    let image_handle = images.add(image);

    // UI Layout in the Map Window using Bevy 0.18 Required Components
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        UiTargetCamera(camera_entity),
    )).with_children(|parent| {
        // Seed Text
        parent.spawn((
            Text::new(format!("World Seed: {}", seed.0)),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
        ));

        // Spacer
        parent.spawn(Node {
            height: Val::Px(20.0),
            ..default()
        });

        // Map Image
        parent.spawn((
            ImageNode::new(image_handle),
            Node {
                width: Val::Px(256.0),
                height: Val::Px(256.0),
                ..default()
            },
        ));
    });
}
