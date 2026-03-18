pub struct PipelineBuilder;

impl PipelineBuilder {
    pub fn create_compute_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::PipelineLayout,
        shader_src: &str,
        entry: &str,
    ) -> wgpu::ComputePipeline {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute Module"),
            source: wgpu::ShaderSource::Wgsl(shader_src.into()),
        });
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(layout),
            module: &module,
            entry_point: Some(entry),
            compilation_options: Default::default(),
            cache: None,
        })
    }
}