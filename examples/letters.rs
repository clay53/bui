use winit::{
    event::{
        Event,
        WindowEvent,
    },
    event_loop::ControlFlow,
};
use bui::{
    rect,
    renderer,
    line,
    ttf,
    ttf_outline,
};

type FaceRef<'a> = &'a mut ttf::CachedFace;
type RectRendererRef<'a> = &'a mut rect::RectRenderer;
type LineRendererRef<'a> = &'a mut line::LineRenderer;
type RendererRef<'a> = &'a renderer::Renderer;
constrainer::create_constrainer!(Contrainer {
    dynamic resx f32
    dynamic resy f32
    external face FaceRef
    external rect_renderer RectRendererRef
    external line_renderer LineRendererRef
    external renderer RendererRef

    listener compute (resx, resy, rect_renderer, face, line_renderer, renderer) {
        println!("Computing!");
        let square = rect::RectDescriptor {
            sizing: rect::FillAspect {
                placement_area: rect::Points {
                    p1x: -1.0,
                    p1y: 1.0,
                    p2x: 1.0,
                    p2y: -1.0
                }.into(),
                centerx: 0.0,
                centery: 0.0,
                resx: resx,
                resy: resy,
                aspect: 1.0,
            }.into(),
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0
        };
    
        let rectangle = rect::RectDescriptor {
            sizing: rect::FillAspect {
                placement_area: square.sizing,
                centerx: 0.0,
                centery: 0.0,
                resx: resx,
                resy: resy,
                aspect: 2.0
            }.into(),
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        };

        let lines = ttf_outline::FillTextFromFaceCurvesAsLines {
            face,
            text: "It's 漢字",
            placement_area: rectangle.sizing,
            curve_line_count: 10,
            resx,
            resy,
        };

        rect_renderer.set_rect_buffer(renderer.queue(), &[
            square.into(),
            rectangle.into()
        ]);
        line_renderer.set_line_buffer(renderer.queue(), Vec::from(lines).as_slice());
    }

    opgenset (resx, resy)
});

fn main() {
    env_logger::init();
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Square")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: 640,
            height: 360,
        })
        .build(&event_loop).unwrap();
    let mut renderer = futures::executor::block_on(renderer::Renderer::new(&window));
    let mut rect_renderer = rect::RectRenderer::new(renderer.device(), renderer.config().format, 2);
    let mut line_renderer = line::LineRenderer::new(renderer.device(), renderer.config().format, 12800);

    let font_bytes = include_bytes!("NotoSansJP-Regular.otf");
    let font_face = owned_ttf_parser::OwnedFace::from_vec(font_bytes.to_vec(), 0).unwrap();
    let mut face = ttf::CachedFace::new(font_face);

    let mut constrainer = Contrainer::new(window.inner_size().width as f32, window.inner_size().height as f32, &mut face, &mut rect_renderer, &mut line_renderer, &renderer);
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit
            },
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::Resized(physical_size) => {
                        renderer.resize(*physical_size);
                        constrainer.set_resx_resy(
                            physical_size.width as f32,
                            physical_size.height as f32,
                            &mut face,
                            &mut rect_renderer,
                            &mut line_renderer,
                            &renderer,
                        )
                    },
                    WindowEvent::ScaleFactorChanged {
                        new_inner_size,
                        ..
                    } => {
                        renderer.resize(**new_inner_size);
                        constrainer.set_resx_resy(
                            new_inner_size.width as f32,
                            new_inner_size.height as f32,
                            &mut face,
                            &mut rect_renderer,
                            &mut line_renderer,
                            &renderer,
                        )
                    },
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                match renderer.surface().get_current_texture() {
                    Ok(surface_texture) => {
                        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                        let mut encoder = renderer.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render encoder"),
                        });
                        rect_renderer.render_all(&mut encoder, &view, wgpu::LoadOp::Clear(wgpu::Color::BLACK));
                        line_renderer.render_all(&mut encoder, &view, wgpu::LoadOp::Load);
                        renderer.queue().submit(std::iter::once(encoder.finish()));
                        surface_texture.present();
                    },
                    Err(wgpu::SurfaceError::Lost) => {
                        eprintln!("Surface lost!");
                        renderer.reconfigure();
                    },
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        eprintln!("Out of memory!");
                        *control_flow = ControlFlow::Exit;
                    },
                    Err(e) => {
                        eprintln!("Surface error: {:?}", e);
                    },
                };
                std::thread::sleep(std::time::Duration::from_millis(1000/60)); // This limits FPS for my poor laptop that crashes if it runs at max fps
                window.request_redraw();
            },
            _ => ()
        }
    });
}