import bevy_sprite_render::mesh2d::{vertex_output::VertexOutput, view_bindings::view};

fn hash_1_1(p: f32) -> f32 {
    var x = fract(p * 0.1031);
    x *= x + 33.33;
    x *= x + x;
    return fract(x);
}

fn hash_1_2(p: f32) -> vec2f {
    var p2 = fract(vec2f(p) * vec2f(0.1031, 0.1030));
    p2 += dot(p2, p2.yx + 33.33);
    return fract((p2.xx + p2.yx) * p2.xy);
}

fn hash_2_1(p: vec2f) -> f32 {
    var p3 = fract(vec3<f32>(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

struct MaterialParams {
    time: vec4f,
}

const STAR_MIN_RADIUS = 2.0;
const STAR_MAX_RADIUS = 4.0;

const TWINKLE_OFF_SECS = 0.0;
const TWINKLE_MAX = 1.0;

const GRID_SIZE = 100.0;

const PARALLAX_LAYERS = 4;
const PARALLAX_MAX_FACTOR = 100.0;
const PARALLAX_MIN_FACTOR = 50.0;
const PARALLAX_STEP = (PARALLAX_MIN_FACTOR - PARALLAX_MAX_FACTOR) / (PARALLAX_LAYERS - 1);

@group(constants::MATERIAL_BIND_GROUP) @binding(0)
var<uniform> material: MaterialParams;

@fragment
fn main(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let view_offset = mesh.world_position.xy - view.world_position.xy;
    var final_color = vec3<f32>(0.0);

    for(var i: f32 = 0.0; i < PARALLAX_LAYERS - 0.1; i += 1.0) {
        let parallax_factor = PARALLAX_MAX_FACTOR + i * PARALLAX_STEP;
        let layer_coord = mesh.world_position.xy + (view_offset * (parallax_factor - 1.0));

        let grid_id = floor(layer_coord / (GRID_SIZE * parallax_factor));
        let grid_coord = fract(layer_coord / (GRID_SIZE * parallax_factor));
        let grid_rand = hash_2_1(grid_id + parallax_factor);

        if grid_rand > 0.45 {
            let world_radius = STAR_MIN_RADIUS
                + hash_1_1(grid_rand + 0.206) * (STAR_MAX_RADIUS - STAR_MIN_RADIUS);

            let grid_radius = world_radius / GRID_SIZE;
            let star_pos = grid_radius + hash_1_2(grid_rand) * (1.0 - grid_radius * 2.0);
            let dist = length(grid_coord - star_pos);

            var star_glow = 1.0 - smoothstep(grid_radius - 0.002, grid_radius + 0.002, dist);

            // Modulate star brightness over time
            let twinkle = sin(material.time.x * 1.0 + hash_2_1(star_pos) * 6.28318) * 0.5 + 0.5;
            star_glow *= smoothstep(0.1, 1.0, twinkle);

            final_color += star_glow * vec3<f32>(0.9, 0.95, 1.0);
        }
    }

    // Output final composition buffer
    return vec4f(final_color, 1.0);
}
