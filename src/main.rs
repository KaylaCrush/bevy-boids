mod boid;

use bevy::prelude::*;
use boid::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280.0, 720.0).into(), // <-- Set your width and height here
                title: "Boid Garden".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, setup_grid.after(setup), spawn_boids.after(setup)))
        .add_systems(Update, (update_cursor_position,))
        .add_systems(FixedUpdate, (
            update_spatial_grid.after(screen_wrap),
            boid_behavior.after(update_spatial_grid),
            apply_forces.after(boid_behavior),
            screen_wrap,
            update_positions.after(screen_wrap),
        ))
        .run();
}
