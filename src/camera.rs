use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, (rotate_camera, move_camera));
    }
}

#[derive(Component)]
pub struct FlyCamera {
    pub sensitivity: f32,
    pub speed: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            sensitivity: 0.1,
            speed: 10.0,
            yaw: -90.0f32.to_radians(),
            pitch: 0.0,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCamera::default(),
    ));
}

fn rotate_camera(
    mut mouse_motion_events: MessageReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut FlyCamera)>,
) {
    let mut rotation_delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        rotation_delta += event.delta;
    }

    if rotation_delta.length_squared() > 0.0 {
        for (mut transform, mut fly_camera) in query.iter_mut() {
            fly_camera.yaw -= rotation_delta.x * fly_camera.sensitivity.to_radians();
            fly_camera.pitch -= rotation_delta.y * fly_camera.sensitivity.to_radians();

            // Clamp pitch to prevent flipping
            fly_camera.pitch = fly_camera.pitch.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

            transform.rotation = Quat::from_euler(EulerRot::YXZ, fly_camera.yaw, fly_camera.pitch, 0.0);
        }
    }
}

fn move_camera(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &FlyCamera)>,
) {
    for (mut transform, fly_camera) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let forward = *transform.forward();
        let right = *transform.right();
        let up = Vec3::Y;

        if keys.pressed(KeyCode::KeyW) {
            velocity += forward;
        }
        if keys.pressed(KeyCode::KeyS) {
            velocity -= forward;
        }
        if keys.pressed(KeyCode::KeyA) {
            velocity -= right;
        }
        if keys.pressed(KeyCode::KeyD) {
            velocity += right;
        }
        if keys.pressed(KeyCode::Space) {
            velocity += up;
        }
        if keys.pressed(KeyCode::ShiftLeft) {
            velocity -= up;
        }

        if velocity != Vec3::ZERO {
            transform.translation += velocity.normalize() * fly_camera.speed * time.delta_secs();
        }
    }
}
