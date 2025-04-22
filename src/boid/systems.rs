use bevy::prelude::*;
use super::{Acceleration, Boid, CursorWorldPosition, MovementSettings, Velocity};



// VARIABLE UPDATE SYSTEMS
pub fn update_cursor_position(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut cursor_world_pos: ResMut<CursorWorldPosition>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(screen_pos) = window.cursor_position() {
        match camera.viewport_to_world_2d(camera_transform, screen_pos){
            Ok(world_pos) => {
                cursor_world_pos.0 = world_pos;
            }
            Err(e) => {
                eprintln!("Failed to convert viewport to world coordinates: {:?}",e);
            }
        }
    }
}


// FIXED UPDATE SYSTEMS
pub fn apply_forces(
    mut query: Query<(&mut Velocity, &Acceleration)>,
    time: Res<Time>,
    settings: Res<MovementSettings>,
) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.0 += (acceleration.0 * time.delta_secs()).clamp_length_max(settings.max_speed);
    }
}

pub fn update_positions(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_secs();

        if velocity.0.length() > 0.001 {
            let angle = velocity.0.y.atan2(velocity.0.x);
            transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2)
        }
    }
}

pub fn screen_wrap(
    mut query: Query<&mut Transform, With<Boid>>,
    windows: Query<&Window>,
){
    let window = windows.single();
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for mut transform in query.iter_mut() {
        let mut pos = transform.translation;

        if pos.x > half_width {
            pos.x = -half_width;
        } else if pos.x < -half_width {
            pos.x = half_width;
        }

        if pos.y > half_height {
            pos.y = -half_height;
        } else if pos.y < -half_height {
            pos.y = half_height;
        }

        transform.translation = pos;
    }
}
