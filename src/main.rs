use bevy::prelude::*;

mod camera;
mod terrain;
mod ui;

#[derive(Resource)]
pub struct WorldSeed(pub u32);

#[derive(Component)]
pub struct MapWindow;

#[derive(Component)]
pub struct GameWindow;

fn main() {
    let seed: u32 = rand::random();

    App::new()
        .insert_resource(WorldSeed(seed))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("MC2 Game (Seed: {})", seed),
                ..default()
            }),
            primary_cursor_options: Some(bevy::window::CursorOptions {
                grab_mode: bevy::window::CursorGrabMode::Locked,
                visible: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(camera::FlyCameraPlugin)
        .add_plugins(terrain::TerrainPlugin)
        .add_plugins(ui::UiPlugin)
        .add_systems(Startup, setup_windows)
        .run();
}

fn setup_windows(mut commands: Commands) {
    // Secondary window for the map
    commands.spawn((
        Window {
            title: "MC2 World Map & Seed".into(),
            ..default()
        },
        MapWindow,
    ));
}
