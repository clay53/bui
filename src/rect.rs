use wgpu::{BufferAddress, include_wgsl};

pub struct RectRenderer {
    rect_buffer: wgpu::Buffer,
    rect_count: u32,
    pipeline: wgpu::RenderPipeline,
}

impl RectRenderer {
    pub fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat, max_rect_count: u64) -> Self {
        let shader = device.create_shader_module(&include_wgsl!("rect.wgsl"));

        let rect_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Rect Buffer"),
            size: RECT_BUFFER_SIZE*max_rect_count,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let rect_layout = wgpu::VertexBufferLayout {
            array_stride: RECT_BUFFER_SIZE,
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

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Rect Pipeline"),
            layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Rect Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            })),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "main",
                buffers: &[
                    rect_layout
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: texture_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            }
        });

        Self {
            rect_buffer,
            rect_count: 0,
            pipeline,
        }
    }

    pub fn set_rect_buffer(&mut self, queue: &wgpu::Queue, data: &[RectBuffer]) {
        self.rect_count = data.len() as u32;
        queue.write_buffer(&self.rect_buffer, 0, bytemuck::cast_slice::<RectBuffer, u8>(data));
    }

    pub fn render_all(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Menu Render Pass"),
            color_attachments: &[
                wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    }
                }
            ],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.rect_buffer.slice(..));
        render_pass.draw(0..6, 0..self.rect_count);
    }
}

#[derive(Debug)]
pub struct RectDescriptor {
    pub sizing: SizeAndCenter,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct SizeAndCenter {
    pub sx: f32,
    pub sy: f32,
    pub cx: f32,
    pub cy: f32,
}

impl From<FillAspect> for SizeAndCenter {
    fn from(fill_aspect: FillAspect) -> Self {
        let placement_area_width = fill_aspect.placement_area.sx;
        let placement_area_height = fill_aspect.placement_area.sy;
        let placement_area_centerx = fill_aspect.placement_area.cx;
        let placement_area_centery = fill_aspect.placement_area.cy;
        let centerx = fill_aspect.centerx;
        let centery = fill_aspect.centery;
        let resx = fill_aspect.resx;
        let resy = fill_aspect.resy;
        let aspect = fill_aspect.aspect;

        let surface_aspect = resx/resy;

        let caxdist = placement_area_width*(1.0-centerx.abs());
        let caydist = placement_area_height*(1.0-centery.abs());

        let area_aspect = (caxdist*resx)/(caydist*resy);

        let (sx, sy) = if area_aspect > aspect {
            let sy = caydist;
            let sx = sy*aspect/surface_aspect;
            (sx, sy)
        } else {
            let sx = caxdist;
            let sy = sx/aspect*surface_aspect;
            (sx, sy)
        };

        let (cx, cy) = (placement_area_centerx+placement_area_width*centerx, placement_area_centery+placement_area_height*centery);

        Self {
            sx,
            sy,
            cx,
            cy
        }
    }
}

impl From<Points> for SizeAndCenter {
    fn from(points: Points) -> Self {
        Self {
            sx: (points.p2x-points.p1x)/2.0,
            sy: (points.p1y-points.p2y)/2.0,
            cx: (points.p1x+points.p2x)/2.0,
            cy: (points.p1y+points.p2y)/2.0
        }
    }
}

#[derive(Clone, Copy)]
pub struct Points {
    pub p1x: f32,
    pub p1y: f32,
    pub p2x: f32,
    pub p2y: f32
}

pub struct FillAspect  {
    pub placement_area: SizeAndCenter,
    pub centerx: f32,
    pub centery: f32,
    pub resx: f32,
    pub resy: f32,
    pub aspect: f32,
}

const RECT_BUFFER_SIZE: BufferAddress = std::mem::size_of::<RectBuffer>() as BufferAddress;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectBuffer {
    scale: [f32; 2],
    translation: [f32; 2],
    color: [f32; 4],
}

impl From<RectDescriptor> for RectBuffer {
    fn from(descriptor: RectDescriptor) -> Self {
        Self {
            scale: [descriptor.sizing.sx, descriptor.sizing.sy],
            translation: [descriptor.sizing.cx, descriptor.sizing.cy],
            color: [descriptor.r, descriptor.g, descriptor.b, descriptor.a]
        }
    }
}