use bevy::prelude::*;
use super::{Acceleration, Boid, BoidBehaviorWeights, BoidSettings, CursorWorldPosition, MovementSettings, Velocity};

// STARTUP SYSTEMS
pub fn setup(
    mut commands: Commands<'_, '_>
){
    commands.spawn(Camera2d);
    commands.insert_resource(CursorWorldPosition(Vec2::ZERO));
    commands.insert_resource(BoidSettings::default());
    commands.insert_resource(BoidBehaviorWeights::default());
    commands.insert_resource(MovementSettings::default());
}

pub fn spawn_boids(
    mut commands: Commands<'_, '_>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    boid_settings: Res<BoidSettings>,
    movement_settings: Res<MovementSettings>,
){

    let boid_mesh = meshes.add(Triangle2d::new(
        Vec2::Y*(boid_settings.boid_scale)*2.0,
        Vec2::new(-boid_settings.boid_scale,-boid_settings.boid_scale),
        Vec2::new(boid_settings.boid_scale,-boid_settings.boid_scale)
    ));

    for i in 0..boid_settings.num_boids {
        let color = Color::hsl(360. * i as f32 / boid_settings.num_boids as f32, 0.95, 0.7);
        let angle = rand::random::<f32>() * std::f32::consts::TAU;
        let radius = rand::random::<f32>() * 300.0;
        let pos = Vec2::new(radius * angle.cos(), radius * angle.sin());
        let vel = Vec3::new(movement_settings.max_speed, angle.sin()*movement_settings.max_speed, 0.0);

        commands.spawn((
            Mesh2d(boid_mesh.clone()),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(pos.x, pos.y, 0.0),
            Velocity(vel),
            Acceleration(Vec3::ZERO),
            Boid,
        ));
    }
}

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
