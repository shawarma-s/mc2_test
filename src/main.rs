use bevy::prelude::*;

mod camera;
mod terrain;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "MC2 - Bevy 0.18".into(),
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
        .run();
}
