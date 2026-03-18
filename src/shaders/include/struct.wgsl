struct Uniforms {
    resolution: vec2<f32>,
    time: f32,
    action: u32,
    camera_pos: vec3<f32>,
    flashlight_on: u32,
    camera_front: vec3<f32>,
    pad1: f32,
    camera_up: vec3<f32>,
    pad2: f32,
};

struct Projectile {
    pos: vec3<f32>,
    is_active: u32,
    vel: vec3<f32>,
    p_type: u32,    // 1 = Míssil, 2 = Estilhaço
    mat_id: u32,
    pad1: u32,
    pad2: u32,
    pad3: u32,
};

struct WorldBuffer {
    data: array<u32>,
};