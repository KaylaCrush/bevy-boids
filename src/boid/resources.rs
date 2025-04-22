use bevy::prelude::*;

#[derive(Resource)]
pub struct SpatialHashGrid {
    pub cells: std::collections::HashMap<(i32, i32), Vec<Entity>>,
    pub cell_size: f32,
}

#[derive(Resource)]
pub struct CursorWorldPosition(pub Vec2);

#[derive(Resource)]
pub struct BoidBehaviorWeights {
    pub alignment: f32,
    pub cohesion: f32,
    pub separation: f32,
    pub separation_distance: f32,
    pub neighbor_distance: f32,
}
impl Default for BoidBehaviorWeights {
    fn default() -> Self {
        Self {
            alignment: 1.5,
            cohesion: 1.0,
            separation:1.5,
            separation_distance:25.0,
            neighbor_distance:50.0,
        }
    }
}

#[derive(Resource)]
pub struct MovementSettings {
    pub max_speed: f32,
    pub max_force: f32,
}
impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            max_speed: 120.0,
            max_force: 20.0,
        }
    }
}

#[derive(Resource)]
pub struct BoidSettings {
    pub num_boids: usize,
    pub boid_scale: f32,
}
impl Default for BoidSettings {
    fn default() -> Self {
        Self {
            num_boids: 150,
            boid_scale: 4.0,
        }
    }
}
