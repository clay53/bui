struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};
// Vertex shader

@vertex
fn vert_main(
    @builtin(vertex_index) vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    if (vertex_index == 0u) {
        out.position = vec4<f32>(1.0, -1.0, 0.0, 1.0);
    } else if (vertex_index == 1u) {
        out.position = vec4<f32>(1.0, 3.0, 0.0, 1.0);
    } else {
        out.position = vec4<f32>(-3.0, -1.0, 0.0, 1.0);
    }
    return out;
}

// Fragment shader

@fragment
fn frag_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}