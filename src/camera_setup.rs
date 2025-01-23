use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

pub(super) fn plugin(app: &mut App) {
    app
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, move_camera);
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
            Camera3d::default(),
            Transform::from_translation(Vec3::new(0.0, 25.0, -50.0)).with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
        )
    );
}

pub fn move_camera(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>, 
    mut camera_transform_query: Query<&mut Transform, With<Camera3d>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let mut camera_transform = camera_transform_query.single_mut();
    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * 0.003;
        let pitch = -motion.delta.y * 0.002;

        
        camera_transform.rotate_y(yaw);
        camera_transform.rotate_local_x(pitch);
    }

    let mut frame_transation = Vec3::ZERO;

    if input.pressed(KeyCode::KeyW) {
        frame_transation += camera_transform.forward().as_vec3();
    } else if input.pressed(KeyCode::KeyS) {
        frame_transation -= camera_transform.forward().as_vec3();
    }
    if input.pressed(KeyCode::KeyD) {
        frame_transation += camera_transform.right().as_vec3();
    } else if input.pressed(KeyCode::KeyA) {
        frame_transation -= camera_transform.right().as_vec3();
    }

    if input.pressed(KeyCode::KeyE) {
        frame_transation += camera_transform.up().as_vec3();
    } else if input.pressed(KeyCode::KeyQ) {
        frame_transation -= camera_transform.up().as_vec3();
    }

    let speed = 10.0;

    camera_transform.translation += frame_transation *  speed * time.delta_secs();
}