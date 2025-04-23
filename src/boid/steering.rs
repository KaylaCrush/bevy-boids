use bevy::prelude::*;
use super::{Acceleration, Boid, BoidBehaviorWeights, CursorWorldPosition, MovementSettings, SpatialHashGrid, Velocity};

// FIXED UPDATE SYSTEMS
pub fn boid_behavior(
    mut query: Query<(Entity, &Transform, &Velocity, &mut Acceleration), With<Boid>>,
    grid: Res<SpatialHashGrid>,
    move_settings: Res<MovementSettings>,
    behavior_settings: Res<BoidBehaviorWeights>,
    windows: Query<&Window>,
    pointer_pos: Res<CursorWorldPosition>

) {
    let window = windows.single();
    for(this_boid, transform, velocity, mut acceleration) in query.iter_mut() {
        let pos = transform.translation;
        let vel = velocity.0;
        let boid_data = get_neighbors_in_grid(pos, &grid);
        let sep = reynolds(separation(&this_boid, &pos, &boid_data, behavior_settings.separation_distance), vel, move_settings.max_speed, move_settings.max_force) * behavior_settings.separation;
        let ali = reynolds(alignment(&this_boid, &pos, &boid_data, behavior_settings.neighbor_distance), vel, move_settings.max_speed, move_settings.max_force) * behavior_settings.alignment;
        let coh = reynolds(cohesion(&this_boid, &pos, &boid_data, behavior_settings.neighbor_distance), vel, move_settings.max_speed, move_settings.max_force) * behavior_settings.cohesion;
        let edges = reynolds(avoid_edges(&pos, &window.width(), &window.height(), 25.0), vel, move_settings.max_speed, move_settings.max_force) * 2.0;
        let pointer = reynolds(avoid_pointer(pos, pointer_pos.0), vel, move_settings.max_speed, move_settings.max_force) * 5.0;
        acceleration.0 = sep + ali + coh;// + edges + pointer;
    }
}

fn avoid_pointer(
    pos: Vec3,
    pointer_pos: Vec2,
) -> Vec3 {
    let avoid_radius = 100.0;
    let max_force = 150.0;

    let to_pointer = pointer_pos.extend(0.0) - pos;
    let dist = to_pointer.length();

    if dist < avoid_radius && dist > 0.0 {
        let away = -to_pointer.normalize();
        let strength = ((avoid_radius - dist) / avoid_radius).powi(2); // smooth falloff
        away * max_force * strength
    } else {
        Vec3::ZERO
    }
}


fn avoid_edges(
    pos: &Vec3,
    screen_width: &f32,
    screen_height: &f32,
    avoid_distance: f32,
) -> Vec3 {
    let mut force = Vec3::ZERO;

    let half_width = screen_width / 2.0;
    let half_height = screen_height / 2.0;

    // Left edge
    let dist_left = pos.x + half_width;
    if dist_left < avoid_distance {
        force.x += 1.0 - dist_left / avoid_distance;
    }

    // Right edge
    let dist_right = half_width - pos.x;
    if dist_right < avoid_distance {
        force.x -= 1.0 - dist_right / avoid_distance;
    }

    // Bottom edge
    let dist_bottom = pos.y + half_height;
    if dist_bottom < avoid_distance {
        force.y += 1.0 - dist_bottom / avoid_distance;
    }

    // Top edge
    let dist_top = half_height - pos.y;
    if dist_top < avoid_distance {
        force.y -= 1.0 - dist_top / avoid_distance;
    }

    force
}


fn get_neighbors_in_grid(
    pos: Vec3,
    grid: &SpatialHashGrid,
) -> Vec<(Entity, Vec3, Vec3)> {
    let boid_cell = (
        (pos.x / grid.cell_size).floor() as i32,
        (pos.y / grid.cell_size).floor() as i32,
    );
    let mut neighbors = Vec::new();
    // Iterate over the surrounding 3x3 area (including the boid's cell itself)
    for x in -1..=1 {
        for y in -1..=1 {
            let neighbor_cell = (boid_cell.0 + x, boid_cell.1 + y);

            if let Some(boids_in_cell) = grid.cells.get(&neighbor_cell) {
                for (entity, pos, vel) in boids_in_cell {
                    // Add all boids in the neighboring cells
                    neighbors.push((*entity, *pos, *vel));
                }
            }
        }
    }
    neighbors
}

// Steering = desired - velocity
fn reynolds(
    mut force: Vec3,
    velocity: Vec3,
    max_speed: f32,
    max_force: f32,
) -> Vec3 {
    if force.length() > 0.0 {
        force = force.normalize();
        force = force * max_speed;
        force = force - velocity;
        force = force.clamp_length_max(max_force);
    }
    force
}

fn separation(
    this_boid: &Entity,
    this_pos: &Vec3,
    boid_data: &[(Entity, Vec3, Vec3)],
    separation_distance: f32,
) -> Vec3 {
    let mut steer = Vec3::ZERO;
    let mut count = 0;
    for (other_boid, other_pos, _ ) in boid_data.iter(){
        if this_boid == other_boid { continue; }
        let distance = this_pos.distance(*other_pos);
        if distance > 0.0 && distance < separation_distance {
            let diff = (this_pos - other_pos).normalize()/distance;
            steer += diff;
            count = count+1;
        }
    }
    if count > 0 {
        steer = steer / count as f32;
    }
    steer
}

fn alignment(
    this_boid: &Entity,
    this_pos: &Vec3,
    boid_data: &[(Entity, Vec3, Vec3)],
    neighbor_distance: f32,
) -> Vec3 {
    let mut steer = Vec3::ZERO;
    let mut count = 0;
    for (other_boid, other_pos, other_vel) in boid_data.iter() {
        if this_boid == other_boid { continue; }
        let distance = this_pos.distance(*other_pos);
        if distance > 0.0 && distance < neighbor_distance {
            steer = steer + other_vel;
            count = count + 1;
        }
        if count > 0 {
            steer = steer / count as f32;
        }
    }
    steer
}

fn cohesion(
    this_boid: &Entity,
    this_pos: &Vec3,
    boid_data: &[(Entity, Vec3, Vec3)],
    neighbor_distance: f32,
) -> Vec3 {
    let mut steer = Vec3::ZERO;
    let mut count = 0;
    for (other_boid, other_pos, _) in boid_data.iter() {
        if this_boid == other_boid { continue; }
        let distance = this_pos.distance(*other_pos);
        if distance > 0.0 && distance < neighbor_distance {
            steer = steer + other_pos;
            count = count + 1;
        }
        if count > 0 {
            steer = steer / count as f32;
        }
    }
    steer
}

