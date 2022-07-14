use wgpu::{BufferAddress, include_wgsl};

use crate::line::{LINE_RAW_SIZE, LineRaw};

pub struct TextRenderer {
    line_buffer: wgpu::Buffer,
    line_count: u32,
    stencil_pipeline: wgpu::RenderPipeline,
    stencil_texture: wgpu::Texture,
    render_pipeline: wgpu::RenderPipeline,
}

impl TextRenderer {
    pub fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat, max_line_count: u64, resx: u32, resy: u32) -> Self {
        let stencil_shader = device.create_shader_module(include_wgsl!("text.wgsl"));
        let render_shader = device.create_shader_module(include_wgsl!("text_render.wgsl"));

        let line_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Text Line Buffer"),
            size: LINE_RAW_SIZE*max_line_count,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let line_layout = wgpu::VertexBufferLayout {
            array_stride: LINE_RAW_SIZE,
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
            ]
        };

        let stencil_face_state = wgpu::StencilFaceState {
            compare: wgpu::CompareFunction::Equal,
            fail_op: wgpu::StencilOperation::Invert,
            depth_fail_op: wgpu::StencilOperation::Keep,
            pass_op: wgpu::StencilOperation::Invert,
        };

        let stencil_texture = Self::generate_stencil_texture(device, resx, resy);

        let stencil_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Text Pipeline"),
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Text Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            })),
            vertex: wgpu::VertexState {
                module: &stencil_shader,
                entry_point: "vert_main",
                buffers: &[
                    line_layout
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &stencil_shader,
                entry_point: "frag_main",
                targets: &[]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: wgpu::StencilState {
                    front: stencil_face_state,
                    back: stencil_face_state,
                    read_mask: 0xff,
                    write_mask: 0xff,
                },
                bias: wgpu::DepthBiasState::default()
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None
        });

        let render_stencil_face_state = wgpu::StencilFaceState {
            compare: wgpu::CompareFunction::Equal,
            fail_op: wgpu::StencilOperation::Keep,
            depth_fail_op: wgpu::StencilOperation::Keep,
            pass_op: wgpu::StencilOperation::Keep,
        };

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Text Pipeline"),
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Text Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            })),
            vertex: wgpu::VertexState {
                module: &render_shader,
                entry_point: "vert_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &render_shader,
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
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: wgpu::StencilState {
                    front: render_stencil_face_state,
                    back: render_stencil_face_state,
                    read_mask: 0xff,
                    write_mask: 0xff,
                },
                bias: wgpu::DepthBiasState::default()
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None
        });

        Self {
            line_buffer,
            line_count: 0,
            stencil_pipeline,
            stencil_texture,
            render_pipeline
        }
    }

    pub fn set_line_buffer(&mut self, queue: &wgpu::Queue, data: &[LineRaw]) {
        self.line_count = data.len() as u32;
        queue.write_buffer(&self.line_buffer, 0, bytemuck::cast_slice::<LineRaw, u8>(data));
    }

    fn generate_stencil_texture(device: &wgpu::Device, resx: u32, resy: u32) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Text Stencil"),
            size: wgpu::Extent3d {
                width: resx,
                height: resy,
                ..Default::default()
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24PlusStencil8,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        })
    }

    pub fn on_resize(&mut self, device: &wgpu::Device, resx: u32, resy: u32) {
        self.stencil_texture = Self::generate_stencil_texture(device, resx, resy);
    }

    pub fn render_all(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, loadop: wgpu::LoadOp<wgpu::Color>) {
        let stencil_texture_view = self.stencil_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut stencil_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Text Stencil Pass"),
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &&stencil_texture_view,
                depth_ops: None,
                stencil_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(0),
                    store: true,
                })
            })
        });
        stencil_pass.set_pipeline(&self.stencil_pipeline);
        stencil_pass.set_vertex_buffer(0, self.line_buffer.slice(..));
        stencil_pass.set_stencil_reference(0);
        stencil_pass.draw(0..3, 0..self.line_count);

        drop(stencil_pass);
        
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Text Render Pass"),
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
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &stencil_texture_view,
                depth_ops: None,
                stencil_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                })
            }),
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_stencil_reference(0xff);
        render_pass.draw(0..3, 0..1);
    }
}