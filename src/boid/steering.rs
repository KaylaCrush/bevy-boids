use bevy::prelude::*;
use rand::Rng;
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
        let boid_data = get_neighbors_in_grid(&pos, &grid);
        let sep = reynolds(separation(&this_boid, &pos, &boid_data, behavior_settings.separation_distance, window), vel, move_settings.max_speed, move_settings.max_force) * behavior_settings.separation;
        let ali = reynolds(alignment(&this_boid, &pos, &boid_data, behavior_settings.neighbor_distance, window), vel, move_settings.max_speed, move_settings.max_force) * behavior_settings.alignment;
        let coh = reynolds(cohesion(&this_boid, &pos, &boid_data, behavior_settings.neighbor_distance, window), vel, move_settings.max_speed, move_settings.max_force) * behavior_settings.cohesion;
        let avoid_pointer = reynolds(avoid_pointer(&pos, &pointer_pos.0, &window), vel, move_settings.max_speed, move_settings.max_force) * 1.0;
        let wander = reynolds(wander(), vel, move_settings.max_speed, move_settings.max_force) * 0.01;
        acceleration.0 = sep + ali + coh + wander;// + avoid_pointer;
    }
}

fn wander() -> Vec3 {
    Vec3::new(
        rand::rng().random_range(-1.0..1.0),
        rand::rng().random_range(-1.0..1.0),
        0.0,
    )
}

fn toroidal_delta(a: Vec3, b: Vec3, width: f32, height: f32) -> Vec3 {
    let mut dx = a.x - b.x;
    let mut dy = a.y - b.y;

    if dx.abs() > width / 2.0 {
        dx = (width/2.0) - dx;
    }

    if dy.abs() > height / 2.0 {
        dy = (width/2.0) - dy;
    }

    Vec3::new(dx, dy, 0.0)
}


fn avoid_pointer(
    pos: &Vec3,
    pointer_pos: &Vec2,
    window: &Window,
) -> Vec3 {
    let avoid_radius = 200.0;
    let max_force = 150.0;

    let delta = toroidal_delta(pointer_pos.extend(0.0),*pos, window.width(), window.height());
    let dist = delta.length();

    if dist < avoid_radius && dist > 0.0 {
        let away = -delta.normalize();
        let strength = ((avoid_radius - dist) / avoid_radius).powi(2); // smooth falloff
        away * max_force * strength
    } else {
        Vec3::ZERO
    }
}


fn get_neighbors_in_grid(
    pos: &Vec3,
    grid: &SpatialHashGrid,
) -> Vec<(Entity, Vec3, Vec3)> {
    let boid_cell_x = (pos.x / grid.cell_size).floor() as i32;
    let boid_cell_y = (pos.y / grid.cell_size).floor() as i32;
    let mut neighbors = Vec::new();

    for dx in -1..=1 {
        for dy in -1..=1 {
            let wrapped_x = (boid_cell_x + dx + grid.grid_width) % grid.grid_width;
            let wrapped_y = (boid_cell_y + dy + grid.grid_height) % grid.grid_height;
            let neighbor_cell = (wrapped_x, wrapped_y);

            if let Some(boids_in_cell) = grid.cells.get(&neighbor_cell) {
                neighbors.extend(boids_in_cell.iter().map(|(e, p, v)| (*e, *p, *v)));
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
    window: &Window,
) -> Vec3 {
    let mut steer = Vec3::ZERO;
    let mut count = 0;
    for (other_boid, other_pos, _ ) in boid_data.iter(){
        if this_boid == other_boid { continue; }
        let delta = toroidal_delta(*this_pos, *other_pos, window.width(), window.height());
        let distance = delta.length();
        if distance > 0.0 && distance < separation_distance {
            let diff = delta.normalize()/distance;
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
    window: &Window,
) -> Vec3 {
    let mut steer = Vec3::ZERO;
    let mut count = 0;
    for (other_boid, other_pos, other_vel) in boid_data.iter() {
        if this_boid == other_boid { continue; }
        let delta = toroidal_delta(*this_pos, *other_pos, window.width(), window.height());
        let distance = delta.length();
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
    window: &Window,
) -> Vec3 {
    let mut steer = Vec3::ZERO;
    let mut count = 0;
    for (other_boid, other_pos, _) in boid_data.iter() {
        if this_boid == other_boid { continue; }
        let delta = toroidal_delta(*this_pos, *other_pos, window.width(), window.height());
        let distance = delta.length();
        if distance > 0.0 && distance < neighbor_distance {
            steer = steer + delta;
            count = count + 1;
        }
        if count > 0 {
            steer = steer / count as f32;
        }
    }
    steer
}