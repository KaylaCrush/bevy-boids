struct Boid {
    pos: vec2<f32>,
    vel: vec2<f32>,
}

struct SimParams {
    deltaT: f32,
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

@group(0) @binding(0) var<uniform> params : SimParams;
@group(0) @binding(1) var<storage, read> boidsSrc: array<Boid>;
@group(0) @binding(2) var<storage, read_write> boidsDst: array<Boid>;

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
    var centerVel: vec2<f32> = vec2<f32>(0.0, 0.0);
    var sepVel: vec2<f32> = vec2<f32>(0.0, 0.0);
    var massCount: i32 = 0;
    var velCount: i32 = 0;

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

        if distance(pos, vPos) < params.sep_dist {
            sepVel -= pos - vPos;
        }
        if distance(pos, vPos) < params.coh_dist {
            centerMass += pos;
            massCount += 1;
        }
        if distance(pos, vPos) < params.ali_dist {
            centerVel += vel;
            velCount += 1;
        }

        continuing {
            i = i + 1u;
        }
    }

    if massCount > 0 {
        centerMass = centerMass * (1.0 / f32(massCount)) - vPos;
    }
    if velCount > 0 {
        centerVel *= 1.0 / f32(velCount);
    }

    vVel = vVel + (sepVel * params.sep_scale) + (centerMass * params.coh_scale) + (centerVel * params.ali_scale);

    vVel = normalize(vVel) * clamp(length(vVel), 0.0, 0.1);

    vPos += vVel * params.deltaT;

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