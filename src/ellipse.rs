use wgpu::{BufferAddress, include_wgsl};
use crate::{
    resolution_buffer::ResolutionBuffer,
};

pub struct EllipseRenderer {
    ellipse_buffer: wgpu::Buffer,
    ellipse_count: u32,
    pipeline: wgpu::RenderPipeline,
    resolution_bind_group: wgpu::BindGroup,
}

impl EllipseRenderer {
    pub fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat, resolution_buffer: &ResolutionBuffer, max_ellipse_count: u64) -> Self {
        let shader = device.create_shader_module(include_wgsl!("ellipse.wgsl"));

        let ellipse_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Ellipse Buffer"),
            size: ELLIPSE_BUFFER_SIZE*max_ellipse_count,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let ellipse_layout = wgpu::VertexBufferLayout {
            array_stride: ELLIPSE_BUFFER_SIZE,
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
                    format: wgpu::VertexFormat::Float32x4
                }
            ]
        };

        let resolution_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Ellipse Resolution Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ]
        });

        let resolution_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Ellipse Resolution Bind Group"),
            layout: &resolution_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: resolution_buffer.binding(),
                }
            ],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Ellipse Pipeline"),
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Ellipse Pipeline Layout"),
                bind_group_layouts: &[
                    &resolution_bind_group_layout
                ],
                push_constant_ranges: &[],
            })),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vert_main",
                buffers: &[
                    ellipse_layout
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "frag_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: texture_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
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
            ellipse_buffer,
            ellipse_count: 0,
            pipeline,
            resolution_bind_group
        }
    }

    pub fn set_ellipse_buffer(&mut self, queue: &wgpu::Queue, data: &[EllipseBuffer]) {
        self.ellipse_count = data.len() as u32;
        queue.write_buffer(&self.ellipse_buffer, 0, bytemuck::cast_slice::<EllipseBuffer, u8>(data));
    }

    pub fn render_all(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, loadop: wgpu::LoadOp<wgpu::Color>) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Menu Render Pass"),
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
        render_pass.set_bind_group(0, &self.resolution_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.ellipse_buffer.slice(..));
        render_pass.draw(0..6, 0..self.ellipse_count);
    }
}

#[derive(Debug)]
pub struct EllipseDescriptor {
    pub sizing: crate::rect::SizeAndCenter,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

const ELLIPSE_BUFFER_SIZE: BufferAddress = std::mem::size_of::<EllipseBuffer>() as BufferAddress;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EllipseBuffer {
    scale: [f32; 2],
    translation: [f32; 2],
    color: [f32; 4],
}

impl From<EllipseDescriptor> for EllipseBuffer {
    fn from(descriptor: EllipseDescriptor) -> Self {
        Self {
            scale: [descriptor.sizing.sx, descriptor.sizing.sy],
            translation: [descriptor.sizing.cx, descriptor.sizing.cy],
            color: [descriptor.r, descriptor.g, descriptor.b, descriptor.a]
        }
    }
}