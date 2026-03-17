#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Projectile {
    pub pos: [f32; 3],
    pub is_active: u32,
    pub vel: [f32; 3],
    pub p_type: u32,    // 1 = Míssil, 2 = Estilhaço
    pub mat_id: u32,    // NOVO: Qual material ele carrega?
    pub pad1: u32,
    pub pad2: u32,
    pub pad3: u32,
}

pub struct ProjectileSystem {
    pub buffer: wgpu::Buffer,
}

impl ProjectileSystem {
    pub fn new(device: &wgpu::Device) -> Self {
        let initial_data = vec![Projectile { 
            pos: [0.0; 3], is_active: 0, vel: [0.0; 3], p_type: 0, mat_id: 0, pad1: 0, pad2: 0, pad3: 0 
        }; 64];
        
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Projectiles Buffer"),
            size: (initial_data.len() * std::mem::size_of::<Projectile>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self { buffer }
    }
}