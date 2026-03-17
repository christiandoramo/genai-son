use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Uniforms {
    pub resolution: [f32; 2],
    pub time: f32,
    pub action: u32,
    pub camera_pos: [f32; 3],
    pub flashlight_on: u32,
    pub camera_front: [f32; 3],
    pub _padding3: f32,
}