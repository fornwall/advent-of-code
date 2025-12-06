pub struct Gpu {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl Gpu {
    pub fn new_buffer_from(&self, data: &[u32], usages: wgpu::BufferUsages) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(data),
                usage: usages,
            })
    }

    pub fn new_buffer_sized_as(
        &self,
        existing_buffer: &wgpu::Buffer,
        usages: wgpu::BufferUsages,
    ) -> wgpu::Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: existing_buffer.size(),
            usage: usages,
            mapped_at_creation: false,
        })
    }
}

pub fn setup() -> Result<Gpu, String> {
    async fn setup_async() -> Result<Gpu, String> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| format!("WGPU: Requesting adapter failed: {e}"))?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            })
            .await
            .map_err(|e| format!("request_device failed: {e}"))?;

        Ok(Gpu {
            instance,
            device,
            queue,
        })
    }
    pollster::block_on(setup_async())
}
