pub mod planet;
pub mod biomes;

pub struct WorldGenerator {
    pub seed: u32,
}

impl WorldGenerator {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    // Função que a Engine chama no início para forjar o planeta na GPU
    pub fn dispatch_initial_gen(&self, device: &wgpu::Device, queue: &wgpu::Queue, pipeline: &wgpu::ComputePipeline, bind_group: &wgpu::BindGroup) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Initial Gen Encoder") });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
            cpass.set_pipeline(pipeline);
            cpass.set_bind_group(0, bind_group, &[]);
            // Dispara 64 workgroups de 4x4x4 para cobrir 256^3
            cpass.dispatch_workgroups(64, 64, 64);
        }
        queue.submit(std::iter::once(encoder.finish()));
    }
}