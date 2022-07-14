struct Vec2f32 {
    inner: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> res: Vec2f32;

struct Instance {
    @location(0) p1: vec2<f32>,
    @location(1) p2: vec2<f32>,
    @location(2) radius: f32,
    @location(3) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};
// Vertex shader

@vertex
fn vert_main(
    @builtin(vertex_index) vertex_index: u32,
    instance: Instance,
) -> VertexOutput {
    var pi = 3.14159; // TODO: not this
    var out: VertexOutput;
    let angle = atan((instance.p2.y-instance.p1.y)/(instance.p2.x-instance.p1.x));
    let rotated = angle+pi/2.0;
    switch (vertex_index) {
        case 0u: {
            out.position = vec4<f32>(instance.p2+vec2<f32>(instance.radius*(cos(angle)+cos(rotated)), instance.radius*(sin(angle)+sin(rotated))), 0.0, 1.0);
        }
        case 1u: {
            out.position = vec4<f32>(instance.p1+vec2<f32>(instance.radius*(-cos(angle)+cos(rotated)), instance.radius*(-sin(angle)+sin(rotated))), 0.0, 1.0);
        }
        case 2u: {
            out.position = vec4<f32>(instance.p2+vec2<f32>(instance.radius*(cos(angle)-cos(rotated)), instance.radius*(sin(angle)-sin(rotated))), 0.0, 1.0);
        }
        case 3u: {
            out.position = vec4<f32>(instance.p1+vec2<f32>(instance.radius*(-cos(angle)-cos(rotated)), instance.radius*(-sin(angle)-sin(rotated))), 0.0, 1.0);
        }
        default: {
            
        }
    }
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
