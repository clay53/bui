#[macro_export]
macro_rules! typed_uniform {
    ($struct_name:ident, $content_type:ty, $label:literal) => {
        pub struct $struct_name {
            inner: wgpu::Buffer,
        }

        impl $struct_name {
            pub fn new(device: &wgpu::Device) -> Self {
                let buffer = device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some($label),
                    size: std::mem::size_of::<$content_type>() as wgpu::BufferAddress,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
        
                Self {
                    inner: buffer,
                }
            }
        
            pub fn binding(&self) -> wgpu::BindingResource {
                self.inner.as_entire_binding()
            }
        
            pub fn set(&self, contents: &$content_type, queue: &wgpu::Queue) {
                queue.write_buffer(&self.inner, 0, bytemuck::cast_slice(contents));
            }
        }
    };
}