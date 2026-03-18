use crate::engine::voxel::material::{MACRO_SIZE, WORLD_SIZE};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Projectile {
    pub pos: [f32; 3],
    pub is_active: u32,
    pub vel: [f32; 3],
    pub p_type: u32,
    pub mat_id: u32,
    pub pad1: u32,
    pub pad2: u32,
    pub pad3: u32,
}
pub struct VoxelStorage {
    pub world_buffer: wgpu::Buffer,
    pub macro_buffer: wgpu::Buffer,
    pub projectile_buffer: wgpu::Buffer,
}

impl VoxelStorage {
    pub fn new(device: &wgpu::Device) -> Self {
        let world_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("World Buffer"),
            size: (WORLD_SIZE.pow(3) * 4) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let macro_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Macro Buffer"),
            size: (MACRO_SIZE.pow(3) * 4) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let projectile_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Projectile Buffer"),
            size: (64 * std::mem::size_of::<Projectile>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            world_buffer,
            macro_buffer,
            projectile_buffer,
        }
    }
}
