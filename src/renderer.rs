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
        // let adapter = instance
        //     .enumerate_adapters(wgpu::Backends::all())
        //     .filter(|adapter| {
        //         surface.get_preferred_format(&adapter).is_some()
        //     })
        //     .next()
        //     .unwrap();
        let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
        let adapter = wgpu::util::initialize_adapter_from_env_or_default(&instance, backend, Some(&surface)).await.unwrap();
        let mut limits = wgpu::Limits::downlevel_webgl2_defaults();
        limits.max_storage_textures_per_shader_stage = 8;
        limits.max_texture_dimension_2d = 8192;
        limits.max_compute_workgroup_size_x = 256;
        limits.max_compute_workgroup_size_y = 256;
        limits.max_compute_workgroup_size_z = 64;
        limits.max_compute_workgroups_per_dimension = 65535;
        limits.max_compute_invocations_per_workgroup = 256;
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits,
                label: None,
            },
            None,
        ).await.unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
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

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0  {
            self.config.width = width;
            self.config.height = height;
            self.reconfigure();
        }
    }

    pub fn reconfigure(&mut self) {
        self.surface.configure(&self.device, &self.config);
    }
}