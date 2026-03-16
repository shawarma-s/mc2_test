use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use std::collections::HashSet;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveChunks>()
            .init_resource::<TerrainMaterials>()
            .add_systems(Startup, (setup_materials, setup_light))
            .add_systems(Update, manage_chunks);
    }
}

fn setup_light(mut commands: Commands) {
    // Add a light source
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 20.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    
    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
        ..default()
    });
}

#[derive(Resource, Default)]
struct TerrainMaterials {
    grass: Handle<StandardMaterial>,
    stone: Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
struct ActiveChunks {
    chunks: HashSet<IVec2>,
}

#[derive(Component)]
struct Chunk;

fn setup_materials(mut materials: ResMut<Assets<StandardMaterial>>, mut terrain_mats: ResMut<TerrainMaterials>) {
    terrain_mats.grass = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.8, 0.2),
        ..default()
    });
    terrain_mats.stone = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.5, 0.5),
        ..default()
    });
}

fn manage_chunks(
    mut commands: Commands,
    camera_query: Single<&Transform, With<Camera3d>>,
    mut active_chunks: ResMut<ActiveChunks>,
    materials: Res<TerrainMaterials>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let camera_transform = *camera_query;

    let chunk_size = 16;
    let render_distance = 4;
    let camera_chunk_pos = IVec2::new(
        (camera_transform.translation.x / chunk_size as f32).floor() as i32,
        (camera_transform.translation.z / chunk_size as f32).floor() as i32,
    );

    let mut needed_chunks = HashSet::new();
    for x in -render_distance..=render_distance {
        for z in -render_distance..=render_distance {
            needed_chunks.insert(camera_chunk_pos + IVec2::new(x, z));
        }
    }

    // Spawn new chunks
    let mesh_handle = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let perlin = Perlin::new(1234);
    let height_scale = 30.0;
    let noise_scale = 1.0 / 90.0;

    for &chunk_pos in &needed_chunks {
        if !active_chunks.chunks.contains(&chunk_pos) {
            spawn_chunk(&mut commands, chunk_pos, chunk_size, &perlin, &materials, mesh_handle.clone(), height_scale, noise_scale);
            active_chunks.chunks.insert(chunk_pos);
        }
    }
}

fn spawn_chunk(
    commands: &mut Commands,
    chunk_pos: IVec2,
    chunk_size: i32,
    perlin: &Perlin,
    materials: &TerrainMaterials,
    mesh_handle: Handle<Mesh>,
    height_scale: f64,
    noise_scale: f64,
) {
    commands.spawn((
        Transform::default(),
        Visibility::default(),
        Chunk,
    )).with_children(|parent| {
        for x in 0..chunk_size {
            for z in 0..chunk_size {
                let world_x = chunk_pos.x * chunk_size + x;
                let world_z = chunk_pos.y * chunk_size + z;

                let octaves = 4;
                let mut noise_val = 0.0;
                for i in 0..octaves {
                    let noise_scale = noise_scale as f32 * (2_i32.pow(i) as f32);
                    let amp = 1.0 / 2_i32.pow(i) as f32;
                    noise_val += amp * perlin.get([world_x as f64 * noise_scale as f64, world_z as f64 * noise_scale as f64]) as f32;
                }
                // let noise_val = perlin.get([world_x as f64 * noise_scale, world_z as f64 * noise_scale]);
                let height: i32 = ((noise_val as f64 + 1.0) * 0.5 * height_scale as f64).round() as i32;

                // Top block (Grass)
                parent.spawn((
                    Mesh3d(mesh_handle.clone()),
                    MeshMaterial3d(materials.grass.clone()),
                    Transform::from_xyz(world_x as f32, height as f32, world_z as f32),
                ));

                // Stone blocks below
                for y in (height - 3)..height {
                    parent.spawn((
                        Mesh3d(mesh_handle.clone()),
                        MeshMaterial3d(materials.stone.clone()),
                        Transform::from_xyz(world_x as f32, y as f32, world_z as f32),
                    ));
                }
            }
        }
    });
}
