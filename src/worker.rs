use bevy::prelude::*;

use bevy_easy_compute::prelude::*;
use bytemuck::{Pod, Zeroable};

use rand::distr::{Distribution, Uniform};

use crate::NUM_BOIDS;

#[derive(ShaderType, Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct Params{
    max_speed: f32,
    max_force: f32,
    sep_dist: f32,
    coh_dist: f32,
    ali_dist: f32,
    sep_scale: f32,
    coh_scale: f32,
    ali_scale: f32,
}

#[derive(ShaderType, Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct FrameDelta{
    delta_t: f32,
}

#[derive(ShaderType, Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct Boid {
    pub pos: Vec2,
    pub vel: Vec2,
}

#[derive(TypePath)]
struct BoidsShader;

impl ComputeShader for BoidsShader {
    fn shader() -> ShaderRef {
        "shaders/boids.wgsl".into()
    }
}

pub struct BoidWorker;

impl ComputeWorker for BoidWorker {
    fn build(world: &mut World) -> AppComputeWorker<Self> {
        let params = Params {
            max_speed: 0.2,
            max_force: 0.03,
            sep_dist: 0.025,
            coh_dist: 0.05,
            ali_dist: 0.05,
            sep_scale: 2.0,
            coh_scale: 1.0,
            ali_scale: 1.5,
        };

        let mut initial_boids_data = Vec::with_capacity(NUM_BOIDS as usize);
        let mut rng = rand::rng();
        let unif = Uniform::new_inclusive(-1.,1.).unwrap();

        for _ in 0..NUM_BOIDS {
            initial_boids_data.push(Boid {
                pos: Vec2::new(
                    unif.sample(&mut rng), 
                    unif.sample(&mut rng),
                ),
                vel: Vec2::new(
                    unif.sample(&mut rng) * params.max_speed,
                    unif.sample(&mut rng) * params.max_speed,   
                ),
            });
    }

    AppComputeWorkerBuilder::new(world)
        .add_uniform("params", &params)
        .add_staging("boids_src", &initial_boids_data)
        .add_staging("boids_dst", &initial_boids_data)
        .add_staging("frame_delta", &0.0)
        .add_pass::<BoidsShader>(
            [(NUM_BOIDS+63) / 64, 1, 1],
            &["params","boids_src","boids_dst","frame_delta"],
        )
        .add_swap("boids_src", "boids_dst")
        .build()
    }
}