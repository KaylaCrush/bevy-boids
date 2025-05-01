mod worker;

use bevy::color::palettes::css;

use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use bevy_easy_compute::prelude::*;

use worker::{Boid, BoidWorker};

const NUM_BOIDS: u32 = 2_000;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AppComputePlugin)
        .add_plugins(AppComputeWorkerPlugin::<BoidWorker>::default())
        .insert_resource(ClearColor(css::BLACK.into()))
        .add_systems(Startup, setup)
        .add_systems(Update, move_entities)
        .run();
}

#[derive(Component)]
struct BoidEntity(pub usize);

fn setup(
    mut commands:Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    
    let boid_mesh = meshes.add(RegularPolygon::new(4., 3));
    for i in 1..NUM_BOIDS {
        let color = Color::hsl(360. * i as f32 / NUM_BOIDS as f32, 0.95, 0.7);
        commands.spawn((
            BoidEntity(i as usize),
            Mesh2d(boid_mesh.clone()),
            MeshMaterial2d(materials.add(color)),
        ));
    }
}

fn move_entities(
    worker: ResMut<AppComputeWorker<BoidWorker>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_boid: Query<(&mut Transform, &BoidEntity), With<BoidEntity>>,
) {
    if !worker.ready(){
        return;
    }

    let window = q_window.single();
    let boids = worker.read_vec::<Boid>("boids_dst");

    q_boid
        .par_iter_mut()
        .for_each(|(mut transform, boid_entity)| {
            let world_pos = Vec2::new(
                (window.width() / 2.) * (boids[boid_entity.0].pos.x),
                (window.height() / 2.) * (boids[boid_entity.0].pos.y),
            );

            transform.translation = world_pos.extend(0.);
            transform.look_to(Vec3::Z, boids[boid_entity.0].vel.extend(0.));
        })

}