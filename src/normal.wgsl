#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var normal_prepass_texture: texture_2d<f32>;

fn prepass_normal(frag_coord: vec2f) -> vec3f {
    return textureLoad(normal_prepass_texture, vec2i(frag_coord), 0).xyz;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4f {
    let frag_coord = in.position.xy;
    let normal = prepass_normal(frag_coord);
    return vec4(normal, 1.0);
}