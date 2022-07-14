struct Instance {
    @location(0) scale: vec2<f32>,
    @location(1) translation: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

var<private> full: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>(1.0, -1.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(-1.0, -1.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(-1.0, 1.0),
);

// Vertex shader

@vertex
fn vert_main(
    @builtin(vertex_index) vertex_index: u32,
    instance: Instance,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(full[vertex_index]*instance.scale+instance.translation, 0.0, 1.0);
    out.color = instance.color;
    return out;
}

// Fragment shader

@fragment
fn frag_main(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    return in.color;
}
