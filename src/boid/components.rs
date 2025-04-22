use bevy::{prelude::*, sprite::Material2d};

#[derive(Component)]
pub struct Boid;

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default)]
pub struct Acceleration(pub Vec3);

#[derive(Bundle)]
pub struct BoidBundle<M: Material2d> {
    pub boid: Boid,
    pub velocity: Velocity,
    pub acceleration: Acceleration,

    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<M>,
    pub transform: Transform,
}
