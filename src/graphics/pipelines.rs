use wgpu::{Device, TextureFormat, Buffer};

// Shaders mapeados no topo do arquivo
const SHADER_RENDER: &str = include_str!("shaders/render.wgsl");
const SHADER_CORE: &str = include_str!("shaders/physics_core.wgsl");
const SHADER_FLUIDS: &str = include_str!("shaders/physics_fluids.wgsl");
const SHADER_PROJECTILES: &str = include_str!("shaders/projectiles.wgsl");
const SHADER_WEAPONS: &str = include_str!("shaders/weapons.wgsl");
const SHADER_WORLD_GEN: &str = include_str!("shaders/world_gen.wgsl"); // NOVO!

pub struct GpuPipelines {
    pub render_pipeline: wgpu::RenderPipeline,
    pub compute_pipeline: wgpu::ComputePipeline,
    pub world_gen_pipeline: wgpu::ComputePipeline, // NOVO!
    pub render_bind_group: wgpu::BindGroup,
    pub compute_bind_group: wgpu::BindGroup,
}

impl GpuPipelines {
    pub fn new(
        device: &Device,
        format: TextureFormat,
        uniform_buf: &Buffer,
        world_buf: &Buffer,
        macro_buf: &Buffer,
        proj_buf: &Buffer,
    ) -> Self {
        // --- BIND GROUP LAYOUTS ---
        let render_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            ],
            label: Some("Render BGL"),
        });

        let compute_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            ],
            label: Some("Compute BGL"),
        });

        // --- BIND GROUPS ---
        let render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &render_bgl,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniform_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: world_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: macro_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: proj_buf.as_entire_binding() },
            ],
            label: Some("Render Bind Group"),
        });

        let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &compute_bgl,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniform_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: world_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: macro_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: proj_buf.as_entire_binding() },
            ],
            label: Some("Compute Bind Group"),
        });

        // --- CARREGAMENTO DE SHADERS ---
        let render_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Render Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER_RENDER.into()),
        });

        let compute_shader_source = format!("{}\n{}\n{}\n{}", SHADER_CORE, SHADER_FLUIDS, SHADER_PROJECTILES, SHADER_WEAPONS);
        let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(compute_shader_source.into()),
        });
        
        let world_gen_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("World Gen Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER_WORLD_GEN.into()),
        });

        // --- PIPELINES ---
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&render_bgl],
            immediate_size: 0,
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState { module: &render_shader, entry_point: Some("vs_main"), compilation_options: Default::default(), buffers: &[] },
            fragment: Some(wgpu::FragmentState { module: &render_shader, entry_point: Some("fs_main"), compilation_options: Default::default(), targets: &[Some(format.into())] }),
            primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleList, cull_mode: None, ..Default::default() },
            depth_stencil: None, multisample: wgpu::MultisampleState::default(), multiview_mask: std::num::NonZeroU32::new(0), cache: None,
        });

        let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &[&compute_bgl],
            immediate_size: 0,
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: Some("cp_main"),
            compilation_options: Default::default(),
            cache: None,
        });
        
        // PIPELINE DE GERAÇÃO DO MUNDO
        let world_gen_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("World Gen Pipeline"),
            layout: Some(&compute_pipeline_layout), // Usa a mesma estrutura de layout da física
            module: &world_gen_shader,
            entry_point: Some("main_gen"),
            compilation_options: Default::default(),
            cache: None,
        });

        Self {
            render_pipeline,
            compute_pipeline,
            world_gen_pipeline, // Mapeado!
            render_bind_group,
            compute_bind_group,
        }
    }
}