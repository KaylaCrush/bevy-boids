struct Boid {
    pos: vec2<f32>,
    vel: vec2<f32>,
}

struct FrameDelta {
    delta_t: f32,
}

struct SimParams {
    max_speed: f32,
    max_force: f32,
    sep_dist: f32,
    coh_dist: f32,
    ali_dist: f32,
    sep_scale: f32,
    coh_scale: f32,
    ali_scale: f32,
}

fn wrapped_position(pos1: vec2<f32>, pos2: vec2<f32>) -> vec2<f32> {
    var wrap_pos: vec2<f32> = pos2;
    var delta = pos2 - pos1;
    if abs(delta.x) > 1.0 {
      wrap_pos.x = pos1.x + (delta.x - (sign(delta.x))*2.0);
    }
    if abs(delta.y) > 1.0 {
      wrap_pos.y = pos1.y + (delta.y - (sign(delta.y))*2.0);
    }
    return wrap_pos;
}

fn reynolds(force: vec2<f32>, velocity: vec2<f32>) -> vec2<f32> {
    if length(force) > 0.0 {
        let desired = normalize(force) * params.max_speed;
        let steer = desired - velocity;
        if length(steer) > params.max_force {
            return normalize(steer) * params.max_force;
        }
        return steer;
    }
    return force;
}

@group(0) @binding(0) var<uniform> params : SimParams;
@group(0) @binding(1) var<storage, read> boidsSrc: array<Boid>;
@group(0) @binding(2) var<storage, read_write> boidsDst: array<Boid>;
@group(0) @binding(3) var<storage, read> frameDelta: FrameDelta;

@compute
@workgroup_size(64)
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {
    let total = arrayLength(&boidsSrc);
    let index = global_invocation_id.x;
    if index >= total {
        return;
    }

    var vPos: vec2<f32> = boidsSrc[index].pos;
    var vVel: vec2<f32> = boidsDst[index].vel;

    var centerMass: vec2<f32> = vec2<f32>(0.0, 0.0);
    var aliSteer: vec2<f32> = vec2<f32>(0.0, 0.0);
    var sepSteer: vec2<f32> = vec2<f32>(0.0, 0.0);
    var cohSteer: vec2<f32> = vec2<f32>(0.0, 0.0);
    var massCount: i32 = 0;
    var velCount: i32 = 0;
    var sepCount: i32 = 0;

    var i: u32 = 0u;
    loop {
        if i >= total {
            break;
        }
        if i == index{
            continue;
        }

        let pos = wrapped_position(vPos, boidsSrc[i].pos);
        let vel = boidsSrc[i].vel;
        let dist = distance(pos, vPos);

        if dist < params.sep_dist {
            sepSteer += normalize(vPos-pos) / dist;
            sepCount += 1;
        }
        if dist < params.coh_dist {
            centerMass += pos;
            massCount += 1;
        }
        if dist < params.ali_dist {
            aliSteer += vel;
            velCount += 1;
        }

        continuing {
            i = i + 1u;
        }
    }

    if sepCount > 0 {
        sepSteer = sepSteer * (1.0/f32(sepCount));
        sepSteer = reynolds(sepSteer, vVel);
    }

    if massCount > 0 {
        cohSteer = normalize(centerMass * (1.0 / f32(massCount)) - vPos) * params.max_speed;
        cohSteer = reynolds(cohSteer, vVel);
        
    }
    if velCount > 0 {
        aliSteer *= 1.0 / f32(velCount);
        aliSteer = reynolds(aliSteer, vVel);
    }
    let noise = vec2<f32>(
            f32(global_invocation_id.x % 10u) * 0.00001,
            f32(global_invocation_id.x % 7u) * 0.00001
        );
    
    let acceleration = (sepSteer * params.sep_scale) + (cohSteer * params.coh_scale) + (aliSteer * params.ali_scale);
    //acceleration += noise;
    vVel += acceleration * frameDelta.delta_t;
    vVel = normalize(vVel) * clamp(length(vVel), 0.0, params.max_speed);

    vPos += vVel * frameDelta.delta_t;

    // Wrap around boundary
    if vPos.x < -1.0 {
        vPos.x = 2.0 + vPos.x;
    }
    if vPos.x > 1.0 {
        vPos.x = -2.0 + vPos.x;
    }
    if vPos.y < -1.0 {
        vPos.y = 2.0 + vPos.y;
    }
    if vPos.y > 1.0 {
        vPos.y = -2.0 + vPos.y;
    }

    // Write back
    boidsDst[index] = Boid(vPos, vVel);  
}