mod boid;

use bevy::prelude::*;
use boid::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_boids.after(setup)))
        .add_systems(Update, (
            update_cursor_position,
        ))
        .add_systems(FixedUpdate, (
            boid_behavior,
            apply_forces.after(boid_behavior),
            update_positions.after(apply_forces),
            screen_wrap.after(update_positions),
        ))
        .run();
}
