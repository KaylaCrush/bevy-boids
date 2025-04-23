use bevy::prelude::*;
use super::{Acceleration, Boid, CursorWorldPosition, MovementSettings, SpatialHashGrid, Velocity};



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
pub fn update_spatial_grid(
    mut grid: ResMut<SpatialHashGrid>,
    query: Query<(Entity, &Transform, &Velocity), With<Boid>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    grid.cells.clear();
    for (entity, transform, velocity) in query.iter() {
        let pos = transform.translation.truncate();
        let cell = (
            (pos.x / grid.cell_size).floor() as i32,
            (pos.y / grid.cell_size).floor() as i32,
        );
        grid.cells.entry(cell).or_default().push((entity, transform.translation, velocity.0));
    }
    grid.grid_width = (window.width() / grid.cell_size).ceil() as i32;
    grid.grid_height = (window.height() / grid.cell_size).ceil() as i32;

}

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

fn wrap_position(pos: Vec3, width: f32, height: f32) -> Vec3 {
    Vec3::new(
        ((pos.x + width / 2.0) % width + width) % width - width / 2.0,
        ((pos.y + height / 2.0) % height + height) % height - height / 2.0,
        pos.z,
    )
}

pub fn screen_wrap(
    mut query: Query<&mut Transform, With<Boid>>,
    windows: Query<&Window>,
){
    let window = windows.single();
    for mut transform in query.iter_mut() {
        transform.translation = wrap_position(transform.translation, window.width(), window.height());
    }
}
