use winit::{
    event::{
        Event,
        WindowEvent,
    },
    event_loop::ControlFlow,
};
use bui::{
    rect,
    renderer
};

type RectRendererRef<'a> = &'a mut rect::RectRenderer;
type RendererRef<'a> = &'a renderer::Renderer;
constrainer::create_constrainer!(Contrainer {
    dynamic resx f32
    dynamic resy f32
    external rect_renderer RectRendererRef
    external renderer RendererRef
    listener compute_rects (resx, resy, rect_renderer, renderer) {
        println!("Computing rects!");
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
    
        rect_renderer.set_rect_buffer(renderer.queue(), &[
            square.into(),
            rectangle.into()
        ]);
    }

    opgenset (resx, resy)
});

fn main() {
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

    let mut constrainer = Contrainer::new(window.inner_size().width as f32, window.inner_size().height as f32, &mut rect_renderer, &renderer);
    
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
                            &mut rect_renderer,
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
                            &mut rect_renderer,
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