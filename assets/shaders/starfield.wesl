import bevy_sprite_render::mesh2d::{
    vertex_output::VertexOutput,
    view_bindings::view,
};

fn hash2d(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

struct MaterialParams {
    time: f32,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(0) var<uniform> material: MaterialParams;

@fragment
fn main(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var view_offset = mesh.world_position.xy - view.world_position.xy;
    let grid_size = 100.0;
    var final_color = vec3<f32>(0.0);

    // Generate stars at 4 parallax depths
    for (var i: f32 = 1.0; i < 4.1; i += 1.0) {
        let parallax_factor = i * 0.1 + 1;
        let layer_coord = mesh.world_position.xy + (view_offset * parallax_factor);
        
        let grid_id = floor(layer_coord / grid_size);
        let grid_coord = fract(layer_coord / grid_size);
        let grid_rand = hash2d(grid_id + vec2f(parallax_factor, parallax_factor));
        
        if (grid_rand > 0.45) {
            let radius = 5.0;
            let star_pos = vec2f(hash2d(grid_id + vec2f(grid_rand, 0)), hash2d(grid_id + vec2f(0, grid_rand)));
            let dist = length(grid_coord - star_pos) * grid_size;

            var star_glow = 1.0 - smoothstep(radius, radius + 0.005, dist);
            
            // Modulate star brightness over time
            let twinkle = sin(material.time * 1.0 + hash2d(star_pos) * 6.28318) * 0.5 + 0.5;
            star_glow *= smoothstep(0.1, 1.0, twinkle);
            
            final_color += star_glow * vec3<f32>(0.9, 0.95, 1.0);
        }
    }

    // Output final composition buffer
    return vec4f(final_color, 1.0);
}
