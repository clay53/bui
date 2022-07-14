use wgpu::{BufferAddress, include_wgsl};

use crate::rect::{SizeAndCenter, FillAspect};

pub struct LineRenderer {
    line_buffer: wgpu::Buffer,
    line_count: u32,
    pipeline: wgpu::RenderPipeline,
}

impl LineRenderer {
    pub fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat, max_line_count: u64) -> Self {
        let shader = device.create_shader_module(include_wgsl!("line.wgsl"));

        let line_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Line Buffer"),
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

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Line Pipeline"),
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Line Pipeline Layout"),
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
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
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
            line_buffer,
            line_count: 0,
            pipeline,
        }
    }

    pub fn set_line_buffer(&mut self, queue: &wgpu::Queue, data: &[LineRaw]) {
        self.line_count = data.len() as u32;
        queue.write_buffer(&self.line_buffer, 0, bytemuck::cast_slice::<LineRaw, u8>(data));
    }

    pub fn render_all(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, loadop: wgpu::LoadOp<wgpu::Color>) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Line Render Pass"),
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
        render_pass.set_vertex_buffer(0, self.line_buffer.slice(..));
        render_pass.draw(0..2, 0..self.line_count);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LineDescriptor {
    pub p1x: f32,
    pub p1y: f32,
    pub p2x: f32,
    pub p2y: f32,
}

pub struct UnfitLinesWithSquareUnits {
    pub lines: Vec<LineDescriptor>,
    pub unfit_min_x: f32,
    pub unfit_max_x: f32,
    pub unfit_min_y: f32,
    pub unfit_max_y: f32,
}

impl UnfitLinesWithSquareUnits {
    pub fn width(&self) -> f32 {
        self.unfit_max_x-self.unfit_min_x
    }

    pub fn height(&self) -> f32 {
        self.unfit_max_y-self.unfit_min_y
    }

    pub fn fit_raw(&self, placement_area: SizeAndCenter, resx: f32, resy: f32) -> Vec<LineRaw> {
        let width = self.width();
        let height = self.height();

        let target: SizeAndCenter = FillAspect {
            placement_area,
            centerx: 0.0,
            centery: 0.0,
            resx,
            resy,
            aspect: width/height,
        }.into();

        self.fill_raw_given_width_and_height(target, width, height)
    }

    pub fn fill_raw_given_width_and_height(&self, target: SizeAndCenter, width: f32, height: f32) -> Vec<LineRaw> {
        let sx = target.sx/width*2.0;
        let sy = target.sy/height*2.0;
        let offsetx = -(self.unfit_min_x+width/2.0)*sx+target.cx;
        let offsety = -(self.unfit_min_y+height/2.0)*sy+target.cy;

        let mut lines_raw = Vec::with_capacity(self.lines.len());
        for line in &self.lines {
            let fit_line = LineDescriptor {
                p1x: line.p1x*sx+offsetx,
                p1y: line.p1y*sy+offsety,
                p2x: line.p2x*sx+offsetx,
                p2y: line.p2y*sy+offsety
            };
            lines_raw.push(fit_line.into());
        }

        lines_raw
    }
}

pub const LINE_RAW_SIZE: BufferAddress = std::mem::size_of::<LineRaw>() as BufferAddress;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LineRaw {
    pub p1: [f32; 2],
    pub p2: [f32; 2],
}

impl From<LineDescriptor> for LineRaw {
    fn from(descriptor: LineDescriptor) -> Self {
        Self {
            p1: [descriptor.p1x, descriptor.p1y],
            p2: [descriptor.p2x, descriptor.p2y]
        }
    }
}