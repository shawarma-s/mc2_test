use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use std::collections::{HashSet, HashMap};
use crate::WorldSeed;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveChunks>()
            .init_resource::<TerrainMaterials>()
            .add_systems(Startup, (setup_materials, setup_light))
            .add_systems(Update, (manage_chunks, handle_quit));
    }
}

fn handle_quit(
    keys: Res<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::KeyQ) {
        exit.write(AppExit::Success);
    }
}

fn setup_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 12000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(100.0, 100.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.8, 0.9, 1.0),
        brightness: 600.0,
        ..default()
    });
}

#[derive(Resource, Default)]
pub struct TerrainMaterials {
    pub grass: Handle<StandardMaterial>,
    pub stone: Handle<StandardMaterial>,
    pub water: Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
struct ActiveChunks {
    chunks: HashMap<IVec2, Entity>,
}

#[derive(Component)]
struct Chunk;

fn setup_materials(mut materials: ResMut<Assets<StandardMaterial>>, mut terrain_mats: ResMut<TerrainMaterials>) {
    terrain_mats.grass = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.7, 0.1),
        perceptual_roughness: 0.9,
        ..default()
    });
    terrain_mats.stone = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.4, 0.4),
        perceptual_roughness: 0.8,
        ..default()
    });
    terrain_mats.water = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 0.4, 0.8, 0.6),
        alpha_mode: AlphaMode::Blend,
        metallic: 0.1,
        perceptual_roughness: 0.1,
        ..default()
    });
}

pub fn get_noise_height(x: i32, z: i32, perlin: &Perlin) -> f32 {
    let continental_scale = 1.0 / 250.0;
    let mountain_scale = 1.0 / 120.0;
    let detail_scale = 1.0 / 30.0;

    let c = perlin.get([x as f64 * continental_scale, z as f64 * continental_scale]) as f32;
    let m_mask = perlin.get([x as f64 * (continental_scale * 1.5), z as f64 * (continental_scale * 1.5)]) as f32;
    let m = perlin.get([x as f64 * mountain_scale, z as f64 * mountain_scale]) as f32;
    let d = perlin.get([x as f64 * detail_scale, z as f64 * detail_scale]) as f32;

    let c = (c + 1.0) * 0.5;
    let m_mask = (m_mask + 1.0) * 0.5;

    // Shift c down significantly to make more water (oceans/lakes)
    let c = (c - 0.55).max(-0.6); 

    let mut height = c * 50.0;

    if m_mask > 0.5 {
        let steepness = (m_mask - 0.5) * 2.0; 
        let mountain_peak = m.abs().powf(2.2) * 80.0; // steeper peaks
        height += mountain_peak * steepness;
    }

    height += d * 4.0;
    height
}

fn manage_chunks(
    mut commands: Commands,
    camera_query: Single<&Transform, With<Camera3d>>,
    mut active_chunks: ResMut<ActiveChunks>,
    materials: Res<TerrainMaterials>,
    mut meshes: ResMut<Assets<Mesh>>,
    seed: Res<WorldSeed>,
) {
    let camera_transform = *camera_query;

    let chunk_size = 16;
    let render_distance = 6;
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

    // Despawn old chunks
    let mut to_remove = Vec::new();
    for (&pos, &entity) in active_chunks.chunks.iter() {
        if !needed_chunks.contains(&pos) {
            commands.entity(entity).despawn();
            to_remove.push(pos);
        }
    }
    for pos in to_remove {
        active_chunks.chunks.remove(&pos);
    }

    let mesh_handle = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let perlin = Perlin::new(seed.0);

    for &chunk_pos in &needed_chunks {
        if !active_chunks.chunks.contains_key(&chunk_pos) {
            let entity = spawn_chunk(&mut commands, chunk_pos, chunk_size, &perlin, &materials, mesh_handle.clone());
            active_chunks.chunks.insert(chunk_pos, entity);
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
) -> Entity {
    let sea_level = 0;

    commands.spawn((
        Transform::default(),
        Visibility::default(),
        Chunk,
    )).with_children(|parent| {
        for x in 0..chunk_size {
            for z in 0..chunk_size {
                let world_x = chunk_pos.x * chunk_size + x;
                let world_z = chunk_pos.y * chunk_size + z;

                let height_val = get_noise_height(world_x, world_z, perlin);
                let height = height_val.round() as i32;

                if height >= sea_level {
                    parent.spawn((
                        Mesh3d(mesh_handle.clone()),
                        MeshMaterial3d(materials.grass.clone()),
                        Transform::from_xyz(world_x as f32, height as f32, world_z as f32),
                    ));
                } else {
                    parent.spawn((
                        Mesh3d(mesh_handle.clone()),
                        MeshMaterial3d(materials.stone.clone()),
                        Transform::from_xyz(world_x as f32, height as f32, world_z as f32),
                    ));
                }

                for y in (height - 3)..height {
                    parent.spawn((
                        Mesh3d(mesh_handle.clone()),
                        MeshMaterial3d(materials.stone.clone()),
                        Transform::from_xyz(world_x as f32, y as f32, world_z as f32),
                    ));
                }

                if height < sea_level {
                    for y in (height + 1)..=sea_level {
                        parent.spawn((
                            Mesh3d(mesh_handle.clone()),
                            MeshMaterial3d(materials.water.clone()),
                            Transform::from_xyz(world_x as f32, y as f32, world_z as f32),
                        ));
                    }
                }
            }
        }
    }).id()
}
