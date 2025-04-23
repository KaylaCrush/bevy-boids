
use std::collections::HashMap;
use bevy::prelude::*;
use super::{Acceleration, Boid, BoidBehaviorWeights, BoidSettings, CursorWorldPosition, MovementSettings, SpatialHashGrid, Velocity};

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

pub fn setup_grid(
    mut commands: Commands<'_, '_>,
    behavior_settings: Res<BoidBehaviorWeights>,
) {
    commands.insert_resource(SpatialHashGrid {
        cells: HashMap::new(),
        cell_size: behavior_settings.neighbor_distance,
        grid_height: 0,
        grid_width: 0,
    });
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
        let vel = Vec3::new(angle.cos()*movement_settings.max_speed, angle.sin()*movement_settings.max_speed, 0.0);

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


// pub fn spawn_boids(
//     mut commands: Commands<'_, '_>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     boid_settings: Res<BoidSettings>,
//     movement_settings: Res<MovementSettings>,
// ) {
//     let boid_mesh = meshes.add(Triangle2d::new(
//         Vec2::Y * boid_settings.boid_scale * 2.0,
//         Vec2::new(-boid_settings.boid_scale, -boid_settings.boid_scale),
//         Vec2::new(boid_settings.boid_scale, -boid_settings.boid_scale),
//     ));

//     let half_w = boid_settings.screen_width / 2.0;
//     let half_h = boid_settings.screen_height / 2.0;

//     for i in 0..boid_settings.num_boids {
//         let color = Color::hsl(360. * i as f32 / boid_settings.num_boids as f32, 0.95, 0.7);
//         let edge = rand::random::<u8>() % 4;

//         let pos = match edge {
//             0 => Vec2::new(rand::random::<f32>() * boid_settings.screen_width - half_w, half_h),     // Top
//             1 => Vec2::new(rand::random::<f32>() * boid_settings.screen_width - half_w, -half_h),    // Bottom
//             2 => Vec2::new(-half_w, rand::random::<f32>() * boid_settings.screen_height - half_h),   // Left
//             _ => Vec2::new(half_w, rand::random::<f32>() * boid_settings.screen_height - half_h),    // Right
//         };

//         let angle = rand::random::<f32>() * std::f32::consts::TAU;
//         let vel = Vec3::new(
//             angle.cos() * movement_settings.max_speed,
//             angle.sin() * movement_settings.max_speed,
//             0.0,
//         );

//         commands.spawn((
//             Mesh2d(boid_mesh.clone()),
//             MeshMaterial2d(materials.add(color)),
//             Transform::from_xyz(pos.x, pos.y, 0.0),
//             Velocity(vel),
//             Acceleration(Vec3::ZERO),
//             Boid,
//         ));
//     }
// }
