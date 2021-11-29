pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl Renderer {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .enumerate_adapters(wgpu::Backends::all())
            .filter(|adapter| {
                surface.get_preferred_format(&adapter).is_some()
            })
            .next()
            .unwrap();
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::VERTEX_WRITABLE_STORAGE,
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
        }
    }

    pub fn device(&self) -> &wgpu::Device { &self.device }
    pub fn surface(&self) -> &wgpu::Surface { &self.surface }
    pub fn queue(&self) -> &wgpu::Queue { &self.queue }
    pub fn config(&self) ->&wgpu::SurfaceConfiguration { &self.config }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0  {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.reconfigure();
        }
    }

    pub fn reconfigure(&mut self) {
        self.surface.configure(&self.device, &self.config);
    }
}