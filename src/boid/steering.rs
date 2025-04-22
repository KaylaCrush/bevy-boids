use bevy::prelude::*;
use super::{Acceleration, Boid, BoidBehaviorWeights, MovementSettings, Velocity};

// FIXED UPDATE SYSTEMS
pub fn boid_behavior(
    mut query: Query<(Entity, &Transform, &Velocity, &mut Acceleration), With<Boid>>,
    move_settings: Res<MovementSettings>,
    behavior_settings: Res<BoidBehaviorWeights>,

) {
    let boid_data: Vec<(Entity, Vec3, Vec3)> = query
        .iter()
        .map(|(entity, transform, velocity, _)| (entity, transform.translation, velocity.0))
        .collect();

    for(this_boid, transform, velocity, mut acceleration) in query.iter_mut() {
        let pos = transform.translation;
        let vel = velocity.0;
        let sep = reynolds(separation(&this_boid, &pos, &boid_data, behavior_settings.separation_distance), vel, move_settings.max_speed, move_settings.max_force) * behavior_settings.separation;
        let ali = reynolds(alignment(&this_boid, &pos, &boid_data, behavior_settings.neighbor_distance), vel, move_settings.max_speed, move_settings.max_force) * behavior_settings.alignment;
        let coh = reynolds(cohesion(&this_boid, &pos, &boid_data, behavior_settings.neighbor_distance), vel, move_settings.max_speed, move_settings.max_force) * behavior_settings.cohesion;
        acceleration.0 = sep + ali + coh;
    }
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
