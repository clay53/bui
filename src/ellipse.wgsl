struct Vec2f32 {
    inner: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> res: Vec2f32;

struct Instance {
    @location(0) scale: vec2<f32>,
    @location(1) translation: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) scale: vec2<f32>,
    @location(1) translation: vec2<f32>,
    @location(2) color: vec4<f32>,
};

// Vertex shader

@vertex
fn vert_main(
    @builtin(vertex_index) vertex_index: u32,
    instance: Instance,
) -> VertexOutput {
    var out: VertexOutput;
    switch (vertex_index) {
        case 0u, 3u: {
            out.position = vec4<f32>(vec2<f32>(-1.0, -1.0)*instance.scale+instance.translation, 0.0, 1.0);
        }
        case 1u: {
            out.position = vec4<f32>(vec2<f32>(1.0, -1.0)*instance.scale+instance.translation, 0.0, 1.0);
        }
        case 5u: {
            out.position = vec4<f32>(vec2<f32>(-1.0, 1.0)*instance.scale+instance.translation, 0.0, 1.0);
        }
        // case 2, 4
        default: {
            out.position = vec4<f32>(vec2<f32>(1.0, 1.0)*instance.scale+instance.translation, 0.0, 1.0);
        }
    }
    out.scale = instance.scale;
    out.translation = instance.translation;
    out.color = instance.color;
    return out;
}

// Fragment shader

@fragment
fn frag_main(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let res = res.inner;
    let pos = in.position.xy/res*vec2<f32>(2.0, -2.0)+vec2<f32>(-1.0, 1.0);
    let scale = in.scale;
    let translation = in.translation;
    var col = in.color;

    let normalized_and_translated = (translation-pos)/scale;
    let distcvec = normalized_and_translated*normalized_and_translated;

    if (distcvec.x+distcvec.y > 1.0) {
        discard;
    }

    return col;
}
