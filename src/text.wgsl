struct Instance {
    @location(0) p1: vec2<f32>,
    @location(1) p2: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};
// Vertex shader

@vertex
fn vert_main(
    @builtin(vertex_index) vertex_index: u32,
    instance: Instance,
) -> VertexOutput {
    var out: VertexOutput;
    if (vertex_index == 0u) {
        out.position = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    } else if (vertex_index == 1u) {
        out.position = vec4<f32>(instance.p1, 0.0, 1.0);
    } else {
        out.position = vec4<f32>(instance.p2, 0.0, 1.0);
    }
    return out;
}

// Fragment shader

@fragment
fn frag_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
