use wgpu::{include_wgsl, BufferAddress};

pub struct Freeform2DCapsuleRenderer {
    capsule_buffer: wgpu::Buffer,
    capsule_count: u32,
    pipeline: wgpu::RenderPipeline,
}

impl Freeform2DCapsuleRenderer {
    pub fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat, max_capsule_count: u64) -> Self {
        let shader = device.create_shader_module(include_wgsl!("freeform_2dcapsule.wgsl"));

        let line_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Freeform 2D Capsule Buffer"),
            size: FREEFORM2DCAPSULE_SIZE*max_capsule_count,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let line_layout = wgpu::VertexBufferLayout {
            array_stride: FREEFORM2DCAPSULE_SIZE,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as BufferAddress*2,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<f32>() as BufferAddress*5,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4
                }
            ]
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Freeform 2D Capsule Pipeline"),
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Freeform 2D Capsule Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            })),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vert_main",
                buffers: &[
                    line_layout
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "frag_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: texture_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None
        });

        Self {
            capsule_buffer: line_buffer,
            capsule_count: 0,
            pipeline,
        }
    }

    pub fn set_capsule_buffer(&mut self, queue: &wgpu::Queue, data: &[Freeform2DCapsule]) {
        self.capsule_count = data.len() as u32;
        queue.write_buffer(&self.capsule_buffer, 0, bytemuck::cast_slice::<Freeform2DCapsule, u8>(data));
    }

    pub fn render_all(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, loadop: wgpu::LoadOp<wgpu::Color>) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Freeform 2D Capsule Render Pass"),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: loadop,
                        store: true,
                    }
                })
            ],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.capsule_buffer.slice(..));
        render_pass.draw(0..4, 0..self.capsule_count);
    }
}

pub const FREEFORM2DCAPSULE_SIZE: BufferAddress = std::mem::size_of::<Freeform2DCapsule>() as BufferAddress;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Freeform2DCapsule {
    pub p1: [f32; 2],
    pub p2: [f32; 2],
    pub radius: f32,
    pub color: [f32; 4],
}